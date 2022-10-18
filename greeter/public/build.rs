fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = vec!["greeter.proto"];

    // Terrible terrible hack...
    #[cfg(target_os = "macos")]
    std::env::set_var("PROTOC", "/opt/homebrew/bin/protoc");
    #[cfg(not(target_os = "macos"))]
    std::env::set_var("PROTOC", "/usr/bin/protoc");

    for proto in protos {
        tonic_build::compile_protos(proto)?;
    }
    Ok(())
}
