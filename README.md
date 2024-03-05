# LaunchDarkly sample Rust application

[![Build and run](https://github.com/launchdarkly/hello-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/launchdarkly/hello-rust/actions/workflows/ci.yml)

We've built a simple console application that demonstrates how LaunchDarkly's SDK works.

Below, you'll find the build procedure. For more comprehensive instructions, you can visit your [Quickstart page](https://app.launchdarkly.com/quickstart#/) or the [Rust SDK reference guide](https://docs.launchdarkly.com/sdk/server-side/rust).

## Build instructions

1. On the command line, set the value of the environment variable `LAUNCHDARKLY_SERVER_KEY` to your LaunchDarkly SDK key.
    ```bash
    export LAUNCHDARKLY_SERVER_KEY="1234567890abcdef"
    ```
1. On the command line, set the value of the environment variable `LAUNCHDARKLY_FLAG_KEY` to an existing boolean feature flag in your LaunchDarkly project that you want to evaluate.

    ```bash
    export LAUNCHDARKLY_FLAG_KEY="my-boolean-flag"
    ```
1. On the command line, run `cargo run -q`.

You should receive the message `"Feature flag '<flag key>' is <true/false> for this user"`.
