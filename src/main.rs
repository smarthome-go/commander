#[macro_use]
extern crate rocket;

use clap::Parser;
use config::Config;
use std::{env, process};

mod config;
mod guards;
mod route;
mod schema;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Microservice for executing shell commands via API requests
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // The API access token which is required to execute commands
    #[clap(short, long, value_parser, required = false)]
    token: Option<String>,
}

#[launch]
fn rocket() -> _ {
    // Parse and intialize configuration
    let args = Args::parse();

    let token = args.token.unwrap_or_else(|| {
        env::var("COMMANDER_TOKEN").unwrap_or_else(|_| {
            eprintln!("The environment variable `COMMANDER_TOKEN` or the `-t`flag is required.");
            process::exit(1)
        })
    });

    println!("Commander v{VERSION} is starting...");

    rocket::build()
        .mount("/", routes![route::index_handler, route::exec_handler])
        .manage(Config { token })
}
