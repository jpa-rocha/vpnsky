mod cli;

use clap::Parser;
use cli::{Cli, Commands};
use spdlog::{debug, error};
use std::error::Error;
use vpnsky::config::{CONFIG, CONFIG_PATH, load_options};
use vpnsky::logs::init_logs;

fn main() -> Result<(), Box<dyn Error>> {
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
            debug!("logger set up complete")
        }
        Err(e) => {
            error!("could not configure logger");
            return Err(e);
        }
    }

    let args = Cli::parse();
    match args.command {
        Commands::Start(cmd) => cmd.execute(),
        Commands::Stop(cmd) => cmd.execute(),
    }
}
