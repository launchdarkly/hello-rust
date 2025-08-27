use hyper::client::connect::Connection;
use hyper::client::HttpConnector;
use hyper::service::Service;
use hyper::Uri;
use launchdarkly_server_sdk::{
    EventProcessorBuilder, PollingDataSourceBuilder, StreamingDataSourceBuilder,
};
use rustls::pki_types::{InvalidDnsNameError, ServerName};
use std::convert::TryFrom;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_rustls::client::TlsStream;
use tokio_rustls::{rustls, TlsConnector};

/// An HTTPS connector that uses aws-lc-rs as the cryptographic provider for FIPS compliance.
#[derive(Clone)]
pub struct AwsLcRsHttpsConnector {
    http: HttpConnector,
    tls: TlsConnector,
}

impl AwsLcRsHttpsConnector {
    /// Create a new AwsLcRsHttpsConnector with default settings.
    ///
    /// This will use the system's native root certificates and enable both HTTP/1 and HTTP/2.
    pub fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let mut http = HttpConnector::new();
        http.enforce_http(false);
        http.set_nodelay(true);
        http.set_keepalive(Some(std::time::Duration::from_secs(60)));

        // Create a rustls config that uses aws-lc-rs
        let mut root_store = rustls::RootCertStore::empty();

        // Load system root certificates
        let certs = rustls_native_certs::load_native_certs();
        for cert in certs.certs {
            root_store.add(cert)?;
        }

        let config = rustls::ClientConfig::builder_with_provider(
            rustls::crypto::aws_lc_rs::default_provider().into(),
        )
        .with_safe_default_protocol_versions()?
        .with_root_certificates(root_store)
        .with_no_client_auth();

        let tls = TlsConnector::from(std::sync::Arc::new(config));

        Ok(Self { http, tls })
    }

    /// Create a new AwsLcRsHttpsConnector with custom configuration.
    pub fn with_config(
        config: rustls::ClientConfig,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let mut http = HttpConnector::new();
        http.enforce_http(false);
        http.set_nodelay(true);
        http.set_keepalive(Some(std::time::Duration::from_secs(60)));

        let tls = TlsConnector::from(std::sync::Arc::new(config));

        Ok(Self { http, tls })
    }
}

impl Default for AwsLcRsHttpsConnector {
    fn default() -> Self {
        Self::new().expect("Failed to create default AwsLcRsHttpsConnector")
    }
}

impl Service<Uri> for AwsLcRsHttpsConnector {
    type Response = AwsLcRsConnection;
    type Error = Box<dyn std::error::Error + Send + Sync>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.http.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, uri: Uri) -> Self::Future {
        let is_https = uri.scheme() == Some(&hyper::http::uri::Scheme::HTTPS);
        let host = uri.host().unwrap_or("").to_owned();

        let http_future = self.http.call(uri);
        let tls = self.tls.clone();

        Box::pin(async move {
            let tcp_stream = http_future.await?;

            if is_https {
                let domain = ServerName::try_from(host.clone())
                    .map_err(|e: InvalidDnsNameError| format!("Invalid DNS name: {}", e))?;
                let tls_stream = tls.connect(domain, tcp_stream).await?;
                Ok(AwsLcRsConnection::Https(tls_stream))
            } else {
                Ok(AwsLcRsConnection::Http(tcp_stream))
            }
        })
    }
}

/// Connection type that can handle both HTTP and HTTPS connections.
pub enum AwsLcRsConnection {
    /// HTTP connection
    Http(tokio::net::TcpStream),
    /// HTTPS connection using aws-lc-rs
    Https(TlsStream<tokio::net::TcpStream>),
}

impl Connection for AwsLcRsConnection {
    fn connected(&self) -> hyper::client::connect::Connected {
        match self {
            AwsLcRsConnection::Http(stream) => stream.connected(),
            AwsLcRsConnection::Https(_) => hyper::client::connect::Connected::new(),
        }
    }
}

impl AsyncRead for AwsLcRsConnection {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        match &mut *self {
            AwsLcRsConnection::Http(stream) => Pin::new(stream).poll_read(cx, buf),
            AwsLcRsConnection::Https(stream) => Pin::new(stream).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for AwsLcRsConnection {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        match &mut *self {
            AwsLcRsConnection::Http(stream) => Pin::new(stream).poll_write(cx, buf),
            AwsLcRsConnection::Https(stream) => Pin::new(stream).poll_write(cx, buf),
        }
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        match &mut *self {
            AwsLcRsConnection::Http(stream) => Pin::new(stream).poll_flush(cx),
            AwsLcRsConnection::Https(stream) => Pin::new(stream).poll_flush(cx),
        }
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        match &mut *self {
            AwsLcRsConnection::Http(stream) => Pin::new(stream).poll_shutdown(cx),
            AwsLcRsConnection::Https(stream) => Pin::new(stream).poll_shutdown(cx),
        }
    }
}

/// Convenience function to create a streaming data source using aws-lc-rs for FIPS compliance.
pub fn streaming_data_source_fips() -> StreamingDataSourceBuilder<AwsLcRsHttpsConnector> {
    let mut builder = StreamingDataSourceBuilder::new();
    builder.https_connector(AwsLcRsHttpsConnector::default());
    builder
}

/// Convenience function to create a polling data source using aws-lc-rs for FIPS compliance.
pub fn polling_data_source_fips() -> PollingDataSourceBuilder<AwsLcRsHttpsConnector> {
    let mut builder = PollingDataSourceBuilder::new();
    builder.https_connector(AwsLcRsHttpsConnector::default());
    builder
}

/// Convenience function to create an event processor using aws-lc-rs for FIPS compliance.
pub fn event_processor_fips() -> EventProcessorBuilder<AwsLcRsHttpsConnector> {
    let mut builder = EventProcessorBuilder::new();
    builder.https_connector(AwsLcRsHttpsConnector::default());
    builder
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_aws_lc_rs_connector() {
        let connector = AwsLcRsHttpsConnector::new();
        assert!(connector.is_ok());
    }

    #[test]
    fn streaming_data_source_fips_creates_builder() {
        let _builder = streaming_data_source_fips();
    }

    #[test]
    fn polling_data_source_fips_creates_builder() {
        let _builder = polling_data_source_fips();
    }

    #[test]
    fn event_processor_fips_creates_builder() {
        let _builder = event_processor_fips();
    }
}
