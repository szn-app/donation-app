#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! glob = "0.3.2"
//! log = "0.4.27"
//! env_logger = "0.11.8"
//! clap = "4.5.37"
//!
//! ```

use clap::{Arg, ArgAction, ArgMatches, Command};
use env_logger::Builder;
use glob::glob;
use log::LevelFilter;
use log::{debug, error, info, warn};
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Parse command line arguments and return the matches
fn parse_args() -> ArgMatches {
    Command::new("Proto File Copier")
        .version("1.0")
        .author("Developer")
        .about("Copies proto files from source to target directory")
        .arg(
            Arg::new("silent")
                .short('s')
                .long("silent")
                .help("Suppress all log output")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("dry-run")
                .short('d')
                .long("dry-run")
                .help("Only list files that would be copied without actually copying")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("source")
                .short('f')
                .long("from")
                .help("Source directory (overrides the default)")
                .value_name("SOURCE_DIR"),
        )
        .arg(
            Arg::new("target")
                .short('t')
                .long("to")
                .help("Target directory (overrides the default)")
                .value_name("TARGET_DIR"),
        )
        .get_matches()
}

fn main() -> io::Result<()> {
    // Parse command line arguments
    let matches = parse_args();

    // Get flags from parsed arguments
    let silent = matches.get_flag("silent");
    let dry_run = matches.get_flag("dry-run");

    // Initialize logger with appropriate level based on flags
    let log_level = if silent {
        LevelFilter::Error // Only show errors in silent mode
    } else {
        LevelFilter::Info
    };

    env_logger::Builder::new().filter_level(log_level).init();

    // Get the directory of the current executing file
    let current_file = file!();
    let base_dir = Path::new(current_file)
        .parent()
        .unwrap_or_else(|| Path::new(""));

    // Source and target folders - either from command line or defaults
    let default_source = "../../../dependency/ory-keto"; // Default folder containing proto files
    let default_target = "../src/protobuf-autogen"; // Default folder where proto files will be copied

    // Get source and target paths, using command line args if provided
    let source_folder = matches
        .get_one::<String>("source")
        .map(|s| s.as_str())
        .unwrap_or(default_source);
    let target_folder = matches
        .get_one::<String>("target")
        .map(|s| s.as_str())
        .unwrap_or(default_target);

    // Construct absolute paths for source and target
    let source_path = base_dir.join(source_folder);
    let target_path = base_dir.join(target_folder);

    debug!("Current file: {}", current_file);
    debug!("Base directory: {}", base_dir.display());
    info!("Source path: {}", source_path.display());
    info!("Target path: {}", target_path.display());

    if dry_run {
        info!("Running in dry-run mode - no files will be copied");
    } else {
        // Create target directory if it doesn't exist
        if !target_path.exists() {
            info!("Creating target directory: {}", target_path.display());
            fs::create_dir_all(&target_path)?;
        }
    }

    // Copy all .proto files from source to target
    let copied_files = copy_proto_files(&source_path, &target_path, dry_run)?;

    if dry_run {
        info!(
            "Found {} .proto files that would be copied to {}",
            copied_files,
            target_path.display()
        );
    } else {
        info!(
            "Copied {} .proto files to {}",
            copied_files,
            target_path.display()
        );
    }

    // generate .txt with package names
    let package_list_file = base_dir.join("../src/protobuf-autogen/packages_detected.txt");
    extract_package_names(&source_path, &package_list_file)?;

    Ok(())
}

fn copy_proto_files(source_dir: &Path, target_dir: &Path, dry_run: bool) -> io::Result<usize> {
    let mut count = 0;

    if !source_dir.exists() {
        warn!("Source directory does not exist: {}", source_dir.display());
        return Ok(0);
    }

    // Use glob pattern to find all .proto files recursively
    let pattern = source_dir.join("**/*.proto").to_string_lossy().to_string();
    debug!("Searching with pattern: {}", pattern);

    match glob(&pattern) {
        Ok(paths) => {
            for entry in paths {
                match entry {
                    Ok(path) => {
                        // Get just the filename without the directory structure
                        if let Some(file_name) = path.file_name() {
                            let target_file = target_dir.join(file_name);

                            if dry_run {
                                // Just list the file in dry-run mode
                                info!(
                                    "Would copy: {} -> {}",
                                    path.display(),
                                    target_file.display()
                                );
                                count += 1;
                            } else {
                                // Copy the file directly to the target directory
                                info!("Copying: {} -> {}", path.display(), target_file.display());

                                // Modify import statements to remove nested path
                                let content = fs::read_to_string(&path)?;
                                let modified_content = modify_imports(&content);

                                match fs::write(target_file, modified_content) {
                                    Ok(_) => count += 1,
                                    Err(e) => error!("Failed to copy {}: {}", path.display(), e),
                                }
                            }
                        }
                    }
                    Err(e) => error!("Error with path: {}", e),
                }
            }
        }
        Err(e) => error!("Failed to read glob pattern: {}", e),
    }

    Ok(count)
}

/**
 * Modify the .proto file import statements to match a flat folder structure
 */
fn modify_imports(content: &str) -> String {
    content
        .lines()
        .map(|line| {
            // modify path of import statements
            if line.trim_start().starts_with("import ") {
                let parts: Vec<&str> = line.split('"').collect();
                // apply modification only on lines paths related to ory/keto repo
                if parts.len() >= 2 && parts[1].contains("ory/keto") {
                    let flat_file_name = Path::new(parts[1]).file_name().unwrap().to_str().unwrap();
                    return format!("import \"{}\";", flat_file_name);
                }
            }
            line.to_string()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

/**
 * extract the package names inside the proto files
 */
fn extract_package_names(source_dir: &Path, output_file: &Path) -> io::Result<()> {
    let mut packages = Vec::new();

    let pattern = source_dir.join("**/*.proto").to_string_lossy().to_string();
    debug!("Scanning for packages with pattern: {}", pattern);

    match glob(&pattern) {
        Ok(paths) => {
            for entry in paths {
                if let Ok(path) = entry {
                    let content = fs::read_to_string(&path)?;
                    for line in content.lines() {
                        if line.trim_start().starts_with("package ") {
                            if let Some(package) = line
                                .trim()
                                .strip_prefix("package ")
                                .and_then(|s| s.strip_suffix(';'))
                            {
                                let trimmed = package.trim().to_string();
                                info!("Found package: {} in {}", trimmed, path.display());
                                packages.push(trimmed);
                            }
                        }
                    }
                }
            }
        }
        Err(e) => error!("Failed to read glob pattern: {}", e),
    }

    // Remove duplicates and sort
    packages.sort();
    packages.dedup();

    // Write to file
    fs::write(output_file, packages.join("\n"))?;
    info!(
        "Wrote {} package(s) to {}",
        packages.len(),
        output_file.display()
    );
    Ok(())
}
