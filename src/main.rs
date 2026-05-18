mod cli;
pub mod config;
pub mod errors;
pub mod logs;

use clap::Parser;
use cli::{Cli, Commands};
use spdlog::{error, info};
use vpnsky::config::{CONFIG, CONFIG_PATH, load_options};
use vpnsky::errors::AppErrors;
use vpnsky::logs::init_logs;

fn main() -> Result<(), AppErrors> {
    // Initialize Options
    let options = match load_options() {
        Ok(options) => options,
        Err(e) => {
            error!("could not open config file at {}: {:?}", CONFIG_PATH, e);
            return Err(e.into());
        }
    };
    CONFIG.set(options).unwrap();

    match init_logs() {
        Ok(_) => {
            info!("logger set up complete")
        }
        Err(e) => {
            error!("could not configure logger");
            return Err(e);
        }
    }

    let args = Cli::parse();
    match args.command {
        Commands::New(cmd) => cmd.execute(),
        Commands::Solve(cmd) => cmd.execute(),
    }
}
