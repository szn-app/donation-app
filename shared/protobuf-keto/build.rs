use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

/**
 *  build protobuf stubs
 *
 * - prost-build (& by extension tonic_build) recognizes Google's well-known types (imports in .proto with `google` path) from the prost-types crate and generate the corresponding Rust structs.
 */
fn main() -> Result<(), Box<dyn Error>> {
    // Path to the directory where your .proto files are located
    let proto_dir = Path::new("src/protobuf-autogen");

    // Get all files in the directory
    let proto_files = fs::read_dir(proto_dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().map_or(false, |ext| ext == "proto")) // Filter only .proto files
        .collect::<Vec<PathBuf>>();

    // Compile each .proto file and generate a separate descriptor set for each
    for path in &proto_files {
        let descriptor_set_path = {
            let file_stem = path
                .file_stem()
                .ok_or("Failed to get proto file stem")?
                .to_string_lossy();

            // Construct output file path for the descriptor
            proto_dir.join(format!("{}_descriptor.bin", file_stem))
        };

        tonic_build::configure()
            .build_server(false) // build only client interface
            .file_descriptor_set_path(&descriptor_set_path) // Output the descriptor set for this file
            .compile_protos(&[path], &[proto_dir])?;
    }

    Ok(())
}
