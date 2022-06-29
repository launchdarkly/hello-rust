# LaunchDarkly sample Rust application

We've built a simple console application that demonstrates how LaunchDarkly's SDK works.

Below, you'll find the basic build procedure. For more comprehensive instructions, you can visit your [Quickstart page](https://app.launchdarkly.com/quickstart#/) or the [Rust SDK reference guide](https://docs.launchdarkly.com/sdk/server-side/rust).

## Build instructions

1. Edit main.rs and set the value of the environment variable `SDK_KEY` to your LaunchDarkly SDK key. If there is an existing boolean feature flag in your LaunchDarkly project that you want to evaluate, set the environment variable `FEATURE_FLAG_KEY` to the flag key.

```

```

2. On the command line, run `cargo run -q`.

You should receive the message `"Feature flag '<flag key>' is <true/false> for this user"`.
