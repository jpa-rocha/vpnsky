use spdlog::info;
use vpnsky::config::get_options;
use vpnsky::errors::{AppErrors, FolderCreationError};

use clap::Args;

#[derive(Args, Debug)]
pub struct StartCmd {}

impl StartCmd {
    pub fn execute(&self) -> Result<(), AppErrors> {
        info!("STAAAAAAAAAAAAAART");

        Ok(())
    }
}
