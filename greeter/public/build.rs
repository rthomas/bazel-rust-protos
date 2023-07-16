fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = vec!["greeter.proto"];

    for proto in protos {
        tonic_build::compile_protos(proto)?;
    }
    Ok(())
}
