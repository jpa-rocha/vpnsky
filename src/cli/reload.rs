use spdlog::{error, info};
use std::error::Error;

use clap::Args;

use crate::cli::{start::StartCmd, stop::StopCmd};

#[derive(Args, Debug)]
pub struct ReloadCmd {}

impl ReloadCmd {
    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        info!("reloading vpn");
        let stop = StopCmd {};
        let start = StartCmd {};
        match stop.execute() {
            Ok(_) => match start.execute() {
                Ok(_) => {
                    info!("reloaded vpn sucessfuly");
                    Ok(())
                }
                Err(e) => {
                    error!("failed to reload the vpn");
                    Err(e)
                }
            },
            Err(e) => {
                error!("failed to reload the vpn");
                Err(e)
            }
        }
    }
}
