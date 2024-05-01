fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .compile(
            &["proto/serevices.proto"], //Path to your proto file
            &["proto"],
        )?;
    Ok(())
}