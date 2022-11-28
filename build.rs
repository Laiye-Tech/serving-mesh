fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(false)
        .compile(&["proto/model-mesh.proto"], &["proto/"])?;
    tonic_build::configure()
        .build_server(false)
        .compile(&["proto/model-runtime.proto"], &["proto/"])?;
    Ok(())
}
