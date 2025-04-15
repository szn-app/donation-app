#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! sqlparser = "0.55.0"
//! clap = { version = "4.5", features = ["derive"] }
//! env_logger = "0.11"
//! log = "0.4"
//! ```

use clap::Parser as ClapParser;
use env_logger;
use log;
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;
use std::fs;
use std::path::Path;

#[derive(ClapParser)]
struct Args {
    #[arg(short, long, help = "Path to the SQL file to validate")]
    file: String,
    #[arg(
        short,
        long,
        help = "Run silently (no output, exit code indicates success)"
    )]
    silent: bool,
}

fn validate_sql_file(file_path: &str, silent: bool) -> Result<(), String> {
    // Read the file content
    let contents = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file {}: {}", file_path, e))?;

    // Initialize the dialect and parser
    let dialect = PostgreSqlDialect {};

    // Split the content into individual queries (assuming queries are separated by semicolons)
    let queries: Vec<&str> = contents
        .split(';')
        .map(|q| q.trim())
        .filter(|q| !q.is_empty())
        .collect();

    if queries.is_empty() {
        return Err("No SQL queries found in the file".to_string());
    }

    // Track if any query fails
    let mut any_error = None;

    // Validate each query
    for (i, query) in queries.iter().enumerate() {
        match Parser::parse_sql(&dialect, query) {
            Ok(_) => {
                if !silent {
                    log::info!("Query {}: Valid SQL syntax", i + 1);
                }
            }
            Err(e) => {
                log::error!("Query {}: Syntax error: {}", i + 1, e);
                any_error = Some(format!("Query {}: Syntax error: {}", i + 1, e));
            }
        }
    }

    // Return error if any query failed
    match any_error {
        Some(err) => Err(err),
        None => Ok(()),
    }
}

fn main() {
    let args = Args::parse();

    // Initialize logger only if not in silent mode
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    // Resolve the absolute path
    let file_path = Path::new(&args.file);
    let absolute_path = match fs::canonicalize(file_path) {
        Ok(path) => path.to_string_lossy().into_owned(),
        Err(e) => {
            eprintln!("Failed to resolve absolute path for {}: {}", args.file, e);
            std::process::exit(1);
        }
    };

    // Log file validation start only in non-silent mode
    if !args.silent {
        log::info!("Validating SQL syntax of file: {}", absolute_path);
    }

    // Validate the file
    match validate_sql_file(&absolute_path, args.silent) {
        Ok(_) => {
            if !args.silent {
                println!("SQL syntax validation completed.");
            }
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
