use launchdarkly_server_sdk::{Client, ConfigBuilder, ContextBuilder};

#[tokio::main]
async fn main() {
    env_logger::init();

    let sdk_key = std::env::var("LAUNCHDARKLY_SERVER_KEY")
        .expect("LAUNCHDARKLY_SERVER_KEY env should be set");
    let feature_flag_key =
        std::env::var("LAUNCHDARKLY_FLAG_KEY").expect("LAUNCHDARKLY_FLAG_KEY env should be set");

    let config = ConfigBuilder::new(&sdk_key)
        .build()
        .expect("Config failed to build");
    let client = Client::build(config).expect("Client failed to build");

    // Starts the client using the currently active runtime.
    client.start_with_default_executor();

    // Wait to ensure the client has fully initialized.
    if !client.initialized_async().await {
        panic!("SDK failed to initialize");
    }

    // Set up the context properties. This context should appear on your LaunchDarkly contexts dashboard
    // soon after you run the demo.
    let context = ContextBuilder::new("example-user-key")
        .build()
        .expect("Context failed to build");

    let result = client.bool_variation(&context, &feature_flag_key, false);
    println!(
        "Feature flag '{}' is {} for this context",
        feature_flag_key, result
    );

    // Here we ensure that the SDK shuts down cleanly and has a chance to deliver analytics events
    // to LaunchDarkly before the program exits. If analytics events are not delivered, the context
    // properties and flag usage statistics will not appear on your dashboard. In a normal
    // long-running application, the SDK would continue running and events would be delivered
    // automatically in the background.
    client.close();
}
