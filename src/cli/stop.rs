use clap::Args;
use spdlog::{error, info};
use std::error::Error;
use std::{thread, time::Duration};

use crate::cli::check_vpn_pid;

#[derive(Args, Debug)]
pub struct StopCmd {}

impl StopCmd {
    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        match check_vpn_pid() {
            true => {
                let output = std::process::Command::new("sudo")
                    .arg("vpnc-disconnect")
                    .output()?;

                if !output.status.success() {
                    error!("vpnsky could not disconnect");
                    let stderr = String::from_utf8_lossy(&output.stdout);
                    return Err(format!("stop: {}", stderr).into());
                } else {
                    info!("vpnsky disconnected succesfuly")
                }
                thread::sleep(Duration::from_secs(1));
                Ok(())
            }
            false => {
                info!("vpn is not running");
                Ok(())
            }
        }
    }
}
