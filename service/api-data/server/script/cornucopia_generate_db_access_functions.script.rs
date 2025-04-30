// TODO: REQUIRES FIXING versions and dependencies and async 

#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! env_logger = "0.11"
//! log = "0.4"
//! tokio = { version = "1.44.2", features = ["full"] }
//! postgres-types = { version = "0.2.9", features = ["derive"] }
//! cornucopia_async = { version = "0.6.0", features = ["with-serde_json-1"] } # async version relies on tokio-postgres
//! cornucopia = { version = "0.9.0", features = [] }
//! tokio-postgres = { version = "0.7.13", features = [ "with-serde_json-1", "with-time-0_3", "with-uuid-1", "with-eui48-1"] }
//! futures = "0.3.31"
//! deadpool-postgres = { version = "0.14.1" }
//! serde = { version = "1.0.140", features = ["derive"] } # Row serialization
//! serde_json = { version = "1.0.140", features = []}
//! time = { version = "0.3.41", features = []}
//! uuid = { version = "1.16.0", features = []}
//! eui48 = { version = "1.1.0", features = []}
//! rust_decimal = { version = "1.37.1", features = ["db-postgres"] }
//! ```

// TODO: once cornucopia is back to maintenance, evaluate if to use it https://github.com/cornucopia-rs/cornucopia/issues/247

// https://cornucopia-rs.netlify.app/book/using_cornucopia/api
// https://docs.rs/crate/cornucopia/latest
// supported types: https://cornucopia-rs.netlify.app/book/introduction/types#base-types
// Cornucopia extra types - https://cornucopia-rs.netlify.app/book/introduction/dependencies#optional-extra-types

use cornucopia::{CodegenSettings, Error};
use postgres::{Client, NoTls};
use std::fs;
use std::path::Path;
use tokio;

const DB_URL: &str = "postgresql://postgres:postgres@localhost:5432/app";

// This script will generate cornucopia code using a live database connection
// This allows for validation of queries against the actual database schema
fn main() -> Result<(), Error> {
    let queries_path = "./src/database/sql";
    let output_path = "./tmp/";
    let settings = CodegenSettings {
        is_async: true,
        derive_ser: false,
    };

    // Connect to the database
    let mut client = Client::connect(DB_URL, NoTls).expect("Failed to connect to PostgreSQL");

    // Generate code using the live database connection
    let generated_code =
        cornucopia::generate_live(&mut client, queries_path, output_path, settings)?;

    // You can now use the generated code at runtime
    println!("Generated code size: {} bytes", generated_code.len());

    // Optionally write the generated code to a file if needed
    if let Some(file_path) = std::env::var_os("OUTPUT_FILE") {
        std::fs::write(file_path, &generated_code).expect("Failed to write generated code to file");
    }

    Ok(())
}

// ./script.sh cornucopia_generate_db_access_functions.script.rs
// #[tokio::main] // Make the build script asynchronous
// async fn main() {
//     println!("cargo:rerun-if-changed=queries"); // Tell Cargo to re-run this script if the 'queries' directory changes

//     // Define the input SQL directory and the output Rust file
//     let queries_dir = Path::new("queries");
//     let output_file = Path::new("src/db/queries.rs");

//     panic!("nothing here");

//     // Ensure the output directory exists
//     if let Some(parent) = output_file.parent() {
//         fs::create_dir_all(parent).expect("Failed to create output directory for cornucopia");
//     }

//     println!("Generating database query code using cornucopia library...");

//     // Connect to the database to get schema information
//     let (client, connection) = tokio_postgres::connect(DB_URL, NoTls).await
//         .expect("Failed to connect to the database for cornucopia code generation. Ensure DATABASE_URL is correct and the database is running.");

//     // Spawn the connection task
//     tokio::spawn(async move {
//         if let Err(e) = connection.await {
//             eprintln!(
//                 "Database connection error during cornucopia code generation: {}",
//                 e
//             );
//         }
//     });

//     // Read the SQL query files
//     let query_files = fs::read_dir(queries_dir)
//         .expect("Failed to read queries directory")
//         .filter_map(|entry| {
//             let entry = entry.ok()?;
//             let path = entry.path();
//             if path.extension().map_or(false, |ext| ext == "sql") {
//                 Some(path)
//             } else {
//                 None
//             }
//         })
//         .collect::<Vec<_>>();

//     // Create the codegen instance for tokio-postgres
//     let mut codegen = TokioCodegen::new(&client);

//     // Add query files to the codegen
//     for query_file in query_files {
//         let sql = fs::read_to_string(&query_file).expect(&format!(
//             "Failed to read SQL file: {}",
//             query_file.display()
//         ));
//         codegen.add_query(&sql, query_file.to_str().unwrap()); // Add query content and filename
//     }

//     // Generate the Rust code
//     codegen
//         .generate()
//         .await
//         .expect("Failed to generate cornucopia code");

//     // Write the generated code to the output file
//     fs::write(output_file, codegen.to_string()).expect(&format!(
//         "Failed to write generated code to {}",
//         output_file.display()
//     ));

//     println!("Cornucopia code generation successful.");

//     --- Refinery Integration (Optional) ---
//     This part remains the same as it's less common to run migrations
//     directly within the build script using the library approach.
//     Migrations are typically run as a separate step.
// }
