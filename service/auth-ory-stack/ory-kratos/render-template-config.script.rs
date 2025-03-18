use atty;
use dotenv;
use env_logger;
use log;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::Read;
use tera::{Context, Tera};

// defines the structure of the environment variables file
const env_file_required_fields: [&str; 5] = [
    "AUTHENTICATION_URL",
    "APP_URL",
    "AUTH_BASE_URL",
    "DOMAIN_NAME",
    "WILDCARD_SUBDOMAIN_APP_URL",
];

const db_env_file_required_fields: [&str; 2] = ["DB_USER", "DB_PASSWORD"];

const jsonnet_env_file_required_fields: [&str; 1] = ["GOOGLE_JSONNET_MAPPER_BASE64"];

const secret_env_file_required_fields: [&str; 2] =
    ["GOOGLE_OIDC_CLIENT_ID", "GOOGLE_OIDC_CLIENT_PASSWORD"];

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

    // Load environment variables from .env file
    load_environment_variables(&config.environment);

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

fn load_environment_variables(environment: &str) {
    log::debug!("Loading environment variables for {}", environment);
    let dotenv_file = format!("./config/.env.{}", environment);
    let dotenv_file_local = format!("./config/.env.{}.local", environment);

    // Load environment variables from .env files
    if std::path::Path::new(&dotenv_file).exists() {
        log::debug!("Loading environment from {}", dotenv_file);
        dotenv::from_filename(&dotenv_file).ok();
    } else if std::path::Path::new(&dotenv_file_local).exists() {
        log::debug!("Loading environment from {}", dotenv_file_local);
        dotenv::from_filename(&dotenv_file_local).ok();
    } else {
        // list found .env files
        let paths = fs::read_dir("./config").unwrap();
        for path in paths {
            let path = path.unwrap().path();
            if let Some(extension) = path.extension() {
                if extension == "env" || extension.to_str().unwrap_or("").starts_with("env.") {
                    println!("Found .env file: {:?}", path);
                }
            }
        }
        panic!("No .env file found, verify a valid .env.<> or .env.<>.local file exists");
    }
    {
        // Validate that all required fields are present
        for field in env_file_required_fields {
            if env::var(field).is_err() {
                panic!("Required environment variable {} is missing", field);
            }
        }
    }

    // load environment variables for database
    let dotenv_db_file = format!("./config/db_kratos_secret.env");
    dotenv::from_filename(&dotenv_db_file).ok();
    {
        for field in db_env_file_required_fields {
            if env::var(field).is_err() {
                panic!("Required environment variable {} is missing", field);
            }
        }
    }

    // load environment variables for jsonnet
    let dotenv_db_file = format!("./config/jsonnet.env");
    dotenv::from_filename(&dotenv_db_file).ok();
    {
        for field in jsonnet_env_file_required_fields {
            if env::var(field).is_err() {
                panic!("Required environment variable {} is missing", field);
            }
        }
    }

    // load environment variables for secret
    let dotenv_db_file = format!("./config/secret.env");
    dotenv::from_filename(&dotenv_db_file).ok();
    {
        for field in secret_env_file_required_fields {
            if env::var(field).is_err() {
                panic!("Required environment variable {} is missing", field);
            }
        }
    }

    log::debug!("All required environment variables have been loaded.");
}
