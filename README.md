This is based on the "Greeter" example, using the examples from both `grpc` and `tonic`

It exists as a repository that demonstrates how to build a `rust` based `grpc` service using `bazel`, whilst using `tonic` and `prost` on the `rust` side.

This also contains a `c++` client for the same `rust` server.

Dependencies are managed with `cargo-raze` remotely, from the `//third_party` path.

Note: Currently `protoc` is set in a terrible manner in `//greeter/public/build.rs` by just sticking it into the `PROTOC` env var. Ideally this should be passed in from the `BUILD` `build_script_env` parameter.

To run:

server: `bazel run //greeter/server:greeter_server`

rust client: `bazel run //greeter/client:greeter_client_rs`

c++ client: `bazel run //greeter/client:greeter_client_cc`
