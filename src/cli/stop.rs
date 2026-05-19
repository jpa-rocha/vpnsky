use clap::Args;
use spdlog::info;
use vpnsky::config::get_options;
use vpnsky::errors::AppErrors;

#[derive(Args, Debug)]
pub struct StopCmd {}

impl StopCmd {
    pub fn execute(&self) -> Result<(), AppErrors> {
        info!("STOOOOOP");

        Ok(())
    }
}
