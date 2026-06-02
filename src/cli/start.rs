use spdlog::{error, info};
use std::error::Error;
use vpnsky::config::get_options;

use clap::Args;
use vpnsky::secrets::get_sops_secrets;

use crate::cli::check_vpn_pid;

#[derive(Args, Debug)]
pub struct StartCmd {}

impl StartCmd {
    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        match check_vpn_pid() {
            true => {
                info!("vpn is already running");
                Ok(())
            }
            false => {
                let username = match get_sops_secrets(vec!["vpn", "id", "username"]) {
                    Ok(r) => r,
                    Err(_) => panic!("no username available"),
                };

                let password = match get_sops_secrets(vec!["vpn", "id", "password"]) {
                    Ok(r) => r,
                    Err(_) => panic!("no password available"),
                };

                let ipsec_username = match get_sops_secrets(vec!["vpn", "ipsec", "username"]) {
                    Ok(r) => r,
                    Err(_) => panic!("no username available"),
                };

                let ipsec_password = match get_sops_secrets(vec!["vpn", "ipsec", "password"]) {
                    Ok(r) => r,
                    Err(_) => panic!("no password available"),
                };

                let output = std::process::Command::new("sudo")
                    .arg("vpnc")
                    .arg("--gateway")
                    .arg(get_options().settings.address.clone())
                    .arg("--id")
                    .arg(ipsec_username)
                    .arg("--secret")
                    .arg(ipsec_password)
                    .arg("--username")
                    .arg(username)
                    .arg("--password")
                    .arg(password)
                    .arg("--ifmtu")
                    .arg(get_options().settings.mtu.to_string())
                    .output()?;

                if !output.status.success() {
                    error!("vpnsky could not start");
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("start: {}", stderr).into());
                } else {
                    info!("vpnsky started succesfuly")
                }

                Ok(())
            }
        }
    }
}
