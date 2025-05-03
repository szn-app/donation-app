// build protobuf stubs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path("src/user_sync_descriptor.bin")
        .compile_protos(&["src/user_sync.proto"], &["proto"])?;

    // testing protobuf
    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path("src/test_descriptor.bin")
        .compile_protos(&["src/test.proto"], &["proto"])?;

    Ok(())
}
