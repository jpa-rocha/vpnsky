use clap::Args;
use spdlog::{error, info};
use std::error::Error;
use std::{thread, time::Duration};

use crate::cli::{is_alive, running_vpnc_pid};

const EXIT_TIMEOUT: Duration = Duration::from_secs(10);
const EXIT_POLL: Duration = Duration::from_millis(100);

#[derive(Args, Debug)]
pub struct StopCmd {}

impl StopCmd {
    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        match running_vpnc_pid() {
            Some(pid) => {
                let output = std::process::Command::new("sudo")
                    .arg("vpnc-disconnect")
                    .output()?;

                if !output.status.success() {
                    error!("vpnsky could not disconnect");
                    let stderr = String::from_utf8_lossy(&output.stdout);
                    return Err(format!("stop: {}", stderr).into());
                }

                // vpnc runs vpnc-script on SIGTERM to restore routes; starting a
                // new vpnc before it exits races that cleanup.
                let mut waited = Duration::ZERO;
                while is_alive(pid) {
                    if waited >= EXIT_TIMEOUT {
                        error!("vpnc (pid {}) did not exit after disconnect", pid);
                        return Err(format!("stop: vpnc (pid {}) still running", pid).into());
                    }
                    thread::sleep(EXIT_POLL);
                    waited += EXIT_POLL;
                }
                info!("vpnsky disconnected succesfuly");
                Ok(())
            }
            None => {
                info!("vpn is not running");
                Ok(())
            }
        }
    }
}
