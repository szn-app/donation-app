use atty;
use dotenv;
use env_logger;
use log;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::Read;
use tera::{Context, Tera};

#[derive(Serialize, Deserialize)]
struct Config {
    environment: String,
}

fn main() {
    if cfg!(debug_assertions) {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::init();
    }

    log::debug!("Starting script...");

    // Parse command line arguments
    let arguments = parse_arguments();
    {
        // check if stdin argument provided
        if atty::is(atty::Stream::Stdin) {
            panic!("No template provided via stdin");
        }
    }

    // Create and validate config structure
    let config = Config {
        environment: arguments
            .get_one::<String>("environment")
            .expect("environment should have a value")
            .to_string(),
    };

    // Read template from stdin
    let mut template = String::new();
    std::io::stdin()
        .read_to_string(&mut template)
        .expect("Failed to read template from stdin");

    // Initialize Tera with empty string (we'll add template manually)
    let mut tera = Tera::default();
    tera.add_raw_template("template", &template)
        .expect("Failed to add template");

    // Create a context and insert the environment variable
    let mut context = Context::new();
    {
        // Load all environment variables into the context
        for (key, value) in env::vars() {
            context.insert(key, &value);
        }

        context.insert("environment", &config.environment);
    }

    log::debug!("Rendering template...");
    // Render the template
    let rendered = tera
        .render("template", &context)
        .expect("Failed to render template");

    // Write to stdout
    println!("{}", rendered);
    log::debug!("Template rendered and printed to stdout.");
}

fn parse_arguments() -> clap::ArgMatches {
    log::debug!("parse_arguments called");
    clap::Command::new("template-renderer")
        .arg(
            clap::Arg::new("environment")
                .long("environment")
                .short('e')
                .help("Sets the environment")
                .required(true),
        )
        .get_matches()
}
