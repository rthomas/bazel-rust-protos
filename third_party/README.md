The `rust` dependencies are managed by `cargo-raze` - they are setup as remote deps, but can also be configured as vendored ones.

To update them, first add the dependency to `Cargo.toml` and then run `$ cargo raze` from the `crates` directory.

Crates that use the `include_str!` macro (e.g. `prost` and `axum`) will need to have their `BUILD` files updated to specify the `.md` files as `compile_data`. e.g. in the `rust_library` section add the following line:

````
compile_data = glob(["**/*.md"]),
````

Failing to do so will result in a file-not-found error from `rustc` when trying to build.