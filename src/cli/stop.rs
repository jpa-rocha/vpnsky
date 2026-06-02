use clap::Args;
use spdlog::info;
use std::error::Error;

#[derive(Args, Debug)]
pub struct StopCmd {}

impl StopCmd {
    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        info!("STOOOOOP");

        Ok(())
    }
}
