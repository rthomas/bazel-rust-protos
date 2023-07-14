This is based on the "Greeter" example, using the examples from both `grpc` and `tonic`

It exists as a repository that demonstrates how to build a `rust` based `grpc` service using `bazel`, whilst using `tonic` and `prost` on the `rust` side.

This also contains a `c++` client for the same `rust` server.

Dependencies are managed with `cargo-raze` remotely, from the `//third_party` path.

Note: `protoc` is passed to the build script through the `PROTOC` env var. However, the `@com_google_protobuf//:protoc`
rule provided by `rules_proto_grpc` compiles `protoc` from the source, whereas `rules_proto` can use
pre-compiled binaries. If you are only using `prost` and `tonic` consider using `rules_proto`.

In `0.25.0`, `rules_rust` also added bazel-style `protobuf/grpc` support using `prost/tonic` in
parallel with the rust `grpc/protobuf` crates. This approach remains to be useful if one wants to customize
the build process (e.g., using `pbjson` to derive `serde` traits for protobuf messages).


To run:

server: `bazel run //greeter/server:greeter_server`

rust client: `bazel run //greeter/client:greeter_client_rs`

c++ client: `bazel run //greeter/client:greeter_client_cc`
