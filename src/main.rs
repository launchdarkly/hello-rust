use crate::aws_lc_rs::{event_processor_fips, streaming_data_source_fips};
use std::time::Duration;
use std::{thread, time};

pub mod aws_lc_rs;

use launchdarkly_server_sdk::{Client, ConfigBuilder, ContextBuilder};

fn show_evaluation_result(feature_flag_key: &str, result: bool) {
    println!(
        "*** The {} feature flag evaluates to {}",
        feature_flag_key, result,
    );

    if result {
        show_banner();
    }
}

fn show_banner() {
    println!("                 ");
    println!("        ██       ");
    println!("          ██     ");
    println!("      ████████   ");
    println!("         ███████ ");
    println!("██ LAUNCHDARKLY █");
    println!("         ███████ ");
    println!("      ████████   ");
    println!("          ██     ");
    println!("        ██       ");
    println!("                 ");
}

#[tokio::main]
async fn main() {
    env_logger::init();

    // Set sdk_key to your LaunchDarkly SDK key.
    let sdk_key =
        std::env::var("LAUNCHDARKLY_SDK_KEY").expect("LAUNCHDARKLY_SDK_KEY env should be set");
    // Set feature_flag_key to the feature flag key you want to evaluate.
    let feature_flag_key =
        std::env::var("LAUNCHDARKLY_FLAG_KEY").unwrap_or(String::from("sample-feature"));
    let ci = std::env::var("CI").ok();

    // Create a streaming data source that uses aws-lc-rs instead of ring
    let mut streaming_data_source = streaming_data_source_fips();
    streaming_data_source.initial_reconnect_delay(Duration::from_secs(1));

    // Create an event processor that also uses aws-lc-rs instead of ring
    let mut event_processor = event_processor_fips();
    event_processor.flush_interval(Duration::from_secs(5));

    let config = ConfigBuilder::new(&sdk_key)
        .data_source(&streaming_data_source)
        .event_processor(&event_processor)
        .build()
        .expect("failed to build config");

    let client = Client::build(config).expect("Client failed to build");

    // Starts the client using the currently active runtime.
    client.start_with_default_executor();

    // Wait to ensure the client has fully initialized.
    if !client.initialized_async().await {
        panic!("*** SDK failed to initialize. Please check your internet connection and SDK credential for any typo.");
    }

    println!("*** SDK successfully initialized.");

    // Set up the evaluation context. This context should appear on your LaunchDarkly contexts dashboard soon after you run the demo.
    let context = ContextBuilder::new("example-user-key")
        .kind("user")
        .name("Sandy")
        .build()
        .expect("Context failed to build");

    let mut last_value = None;

    loop {
        let result = client.bool_variation(&context, &feature_flag_key, false);

        if Some(result) != last_value {
            show_evaluation_result(&feature_flag_key, result);
            last_value = Some(result);
        }

        thread::sleep(time::Duration::from_secs(1));

        if ci.is_some() {
            break;
        }
    }
}
