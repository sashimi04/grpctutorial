fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .compile(
            &["proto/services.proto"], // path to proto file
            &["proto"],                // path to directory
        )?;
    Ok(())
}
