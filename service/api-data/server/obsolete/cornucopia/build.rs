// https://cornucopia-rs.netlify.app/book/using_cornucopia/api
// https://docs.rs/crate/cornucopia/latest
// supported types: https://cornucopia-rs.netlify.app/book/introduction/types#base-types

use std::fs;
use std::path::Path;

use cornucopia_core::Codegen;
use cornucopia_tokio::Codegen as TokioCodegen;
use dotenv::dotenv;
use tokio_postgres::NoTls; // Or your preferred TLS config // To load environment variables from .env file

#[tokio::main] // Make the build script asynchronous
async fn main() {
    // --- Cornucopia Integration (using the library) ---
    // Add these dependencies to the [build-dependencies] section of your Cargo.toml:
    // cornucopia = { version = "...", features = ["tokio"] }
    // tokio-postgres = "..."
    // tokio = { version = "...", features = ["full"] } // Or appropriate features
    // dotenv = "..." // If you use a .env file

    // Load environment variables from a .env file if it exists
    dotenv().ok();

    println!("cargo:rerun-if-changed=queries"); // Tell Cargo to re-run this script if the 'queries' directory changes

    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable must be set for cornucopia");

    // Define the input SQL directory and the output Rust file
    let queries_dir = Path::new("queries");
    let output_file = Path::new("src/db/queries.rs"); // Example output location

    // Ensure the output directory exists
    if let Some(parent) = output_file.parent() {
        fs::create_dir_all(parent).expect("Failed to create output directory for cornucopia");
    }

    println!("Generating database query code using cornucopia library...");

    // Connect to the database to get schema information
    let (client, connection) = tokio_postgres::connect(&db_url, NoTls).await
        .expect("Failed to connect to the database for cornucopia code generation. Ensure DATABASE_URL is correct and the database is running.");

    // Spawn the connection task
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!(
                "Database connection error during cornucopia code generation: {}",
                e
            );
        }
    });

    // Read the SQL query files
    let query_files = fs::read_dir(queries_dir)
        .expect("Failed to read queries directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "sql") {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // Create the codegen instance for tokio-postgres
    let mut codegen = TokioCodegen::new(&client);

    // Add query files to the codegen
    for query_file in query_files {
        let sql = fs::read_to_string(&query_file).expect(&format!(
            "Failed to read SQL file: {}",
            query_file.display()
        ));
        codegen.add_query(&sql, query_file.to_str().unwrap()); // Add query content and filename
    }

    // Generate the Rust code
    codegen
        .generate()
        .await
        .expect("Failed to generate cornucopia code");

    // Write the generated code to the output file
    fs::write(output_file, codegen.to_string()).expect(&format!(
        "Failed to write generated code to {}",
        output_file.display()
    ));

    println!("Cornucopia code generation successful.");

    // --- Refinery Integration (Optional) ---
    // This part remains the same as it's less common to run migrations
    // directly within the build script using the library approach.
    // Migrations are typically run as a separate step.
}

// Helper function to make the build script asynchronous
// This is needed because tokio_postgres::connect is async
// We use #[tokio::main] above, but this is an alternative if needed
/*
fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime");

    runtime.block_on(async_main());
}

async fn async_main() {
    // ... rest of the async logic from the original main function ...
}
*/
