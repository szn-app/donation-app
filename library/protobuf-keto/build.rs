use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

/**
 * Build protobuf stubs by grouping proto files according to their package
 *
 * - prost-build (& by extension tonic_build) recognizes Google's well-known types (imports in .proto with `google` path)
 *   and generates the corresponding Rust structs from the prost-types crate.
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

    // Map to store package -> list of proto files
    let mut package_to_files: HashMap<String, Vec<PathBuf>> = HashMap::new();

    // Process each proto file to extract its package
    for path in &proto_files {
        let package = extract_package_from_proto(path)?;
        package_to_files
            .entry(package)
            .or_default()
            .push(path.clone());
    }

    // Compile each package group
    for (package, files) in package_to_files {
        let sanitized_package = package.replace('.', "_").replace('-', "_");
        let descriptor_set_path = proto_dir.join(format!("{}_descriptor.bin", sanitized_package));

        println!(
            "Compiling package: {} with {} file(s)",
            package,
            files.len()
        );

        tonic_build::configure()
            .build_server(false) // bind client only without server interfaces
            .file_descriptor_set_path(&descriptor_set_path)
            .compile_protos(&files, &[proto_dir])?;
    }

    // Tell cargo to rerun this build script if the proto files change
    for proto_file in &proto_files {
        println!("cargo:rerun-if-changed={}", proto_file.display());
    }

    Ok(())
}

// Function to extract package name from a proto file
fn extract_package_from_proto(path: &Path) -> Result<String, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        // Look for package definition
        if trimmed.starts_with("package ") && trimmed.ends_with(";") {
            // Extract package name between "package " and ";"
            let package = trimmed
                .strip_prefix("package ")
                .unwrap_or_default()
                .trim_end_matches(';')
                .trim();

            return Ok(package.to_string());
        }
    }

    // If no package is found, use a default name based on the filename
    let default_package = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("default_package");

    Ok(format!("unspecified.{}", default_package))
}
