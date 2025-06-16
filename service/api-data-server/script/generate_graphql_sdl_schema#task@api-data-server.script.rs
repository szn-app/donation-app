#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! api-data-server = { path = "../"}
//! async-graphql = {version = "7.0.16", features = ["graphiql", "uuid", "time", "dataloader", "decimal", "url"]}
//! ```

/**
 * NOTE: must run script with `--force` flag for `rust-script` to generated latest changes from api-data imported modules
 */
use std::fs;
use std::path::Path;

use api_data_server::api::graphql::handler::{EmptySubscription, Mutation, Query};
use api_data_server::database::model::user::Account;
use api_data_server::server::connection::PostgresPool;
use async_graphql::Schema;

// generates GraphQL schema SDL from the async_graphql code schema (which serves as origin for definition schema and also implementation of it)
fn main() {
    let schema = {
        let postgres_pool_group = PostgresPool::new_mock();

        let query_resolver = Query::new(postgres_pool_group.clone()); // pass context as instance value
        let mutation_resolver = Mutation::new(postgres_pool_group); // pass context as instance value
        let subscription_resolver = EmptySubscription;

        Schema::build(query_resolver, mutation_resolver, subscription_resolver).finish()
    };

    // Generate SDL
    let sdl = schema.sdl();

    // Write to file
    {
        let output_path = Path::new(file!())
            .parent()
            .unwrap()
            .join("../")
            .join("./config/schema-autogen.graphql");

        // Create directory structure if it doesn't exist
        if let Some(parent_dir) = output_path.parent() {
            fs::create_dir_all(parent_dir).expect("Failed to create output directory");
        }

        fs::write(&output_path, sdl).expect("Failed to write schema.graphql");
        println!("GraphQL SDL schema exported to: {}", output_path.display());
    }
}
