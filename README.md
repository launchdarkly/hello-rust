# LaunchDarkly Sample Rust Application 

We've built a simple console application that demonstrates how LaunchDarkly's SDK works.

Below, you'll find the basic build procedure, but for more comprehensive instructions, you can visit your [Quickstart page](https://app.launchdarkly.com/quickstart#/) or the [Rust SDK reference guide](https://docs.launchdarkly.com/sdk/server-side/rust).

## Build instructions 

1. Set the environment variable `LAUNCHDARKLY_SDK_KEY` to your LaunchDarkly SDK key.
2. Set the environment variable `FEATURE_FLAG_KEY` to the feature flag key in your LaunchDarkly project.
3. On the command line, run `cargo run -q`.

You should see the messsage `"Feature flag '<flag key>' is <true/false> for this user"`.
