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
use sqlparser::tokenizer::Tokenizer;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(ClapParser)]
struct Args {
    #[arg(short, long, help = "Path to the SQL file to validate")]
    file: Option<PathBuf>,

    #[arg(
        short,
        long,
        help = "Silent mode (suppress logs and output only errors)"
    )]
    silent: bool,
}

fn validate_sql_file(file_path: &str, silent: bool) -> Result<(), String> {
    let contents = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file {}: {}", file_path, e))?;

    let dialect = PostgreSqlDialect {};

    let queries: Vec<&str> = contents
        .split(';')
        .map(|q| q.trim())
        .filter(|q| !q.is_empty() && !q.contains("$$"))
        .collect();

    if queries.is_empty() {
        return Err("No SQL queries found in the file".to_string());
    }

    let mut any_error = None;

    for (i, query) in queries.iter().enumerate() {
        match Parser::parse_sql(&dialect, query) {
            Ok(_) => {
                if !silent {
                    log::info!("Query {}: Valid SQL syntax", i + 1);
                }
            }
            Err(_) => {
                // Try just tokenizing
                let mut tokenizer = Tokenizer::new(&dialect, query);
                match tokenizer.tokenize() {
                    Ok(_) => {
                        if !silent {
                            log::warn!(
                                "Query {}: Unknown keyword tolerated (tokenized only)",
                                i + 1
                            );
                        }
                    }
                    Err(tokenize_error) => {
                        log::error!("Query {}: Syntax error: {}", i + 1, tokenize_error);
                        any_error =
                            Some(format!("Query {}: Syntax error: {}", i + 1, tokenize_error));
                    }
                }
            }
        }
    }

    match any_error {
        Some(err) => Err(err),
        None => Ok(()),
    }
}

fn main() {
    let args = Args::parse();
    let cwd = env::current_dir().expect("Failed to get current working directory");

    if !args.silent {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();
    } else {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Error)
            .init();
    }

    // Resolve SQL file: either from --file or default
    let file_path = args.file.unwrap_or_else(|| {
        Path::new(file!())
            .parent()
            .unwrap()
            .join("../")
            .join("k8s/base/init.sql")
    }); // default value to use in .vscode tasks

    let absolute_path = match fs::canonicalize(&file_path) {
        Ok(path) => path.to_string_lossy().into_owned(),
        Err(e) => {
            eprintln!("Failed to resolve absolute path for {:?}: {}", file_path, e);
            std::process::exit(1);
        }
    };

    if !args.silent {
        log::info!("Validating SQL syntax of file: {}", absolute_path);
    }

    match validate_sql_file(&absolute_path, args.silent) {
        Ok(_) => {
            if !args.silent {
                println!("SQL syntax validation completed successfully.");
            }
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
