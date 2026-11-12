use spdlog::{error, info};
use std::error::Error;
use std::io::Write;
use tempfile::NamedTempFile;
use vpnsky::config::get_options;

use clap::Args;
use vpnsky::secrets::get_sops_secrets;

use crate::cli::{remove_stale_gateway_route, running_vpnc_pid, stop::StopCmd};

#[derive(Args, Debug)]
pub struct StartCmd {}

impl StartCmd {
    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        let running = match running_vpnc_pid().is_some() {
            true => {
                info!("vpn is already running");

                let stop = StopCmd {};
                stop.execute()
            }
            false => Ok(()),
        };

        match running {
            Ok(_) => {
                remove_stale_gateway_route(&get_options().settings.address)?;

                let username = get_sops_secrets(vec!["work", "vpn", "id", "username"])
                    .expect("no username available");

                let password = get_sops_secrets(vec!["work", "vpn", "id", "password"])
                    .expect("no password available");

                let ipsec_username = get_sops_secrets(vec!["work", "vpn", "ipsec", "username"])
                    .expect("no username available");

                let ipsec_password = get_sops_secrets(vec!["work", "vpn", "ipsec", "password"])
                    .expect("no password available");

                let mut file = NamedTempFile::new().expect("could not create config file");

                writeln!(
                    file,
                    "IPSec gateway {}",
                    get_options().settings.address.clone()
                )
                .expect("could not write gateway to config file");

                writeln!(file, "IPSec ID {}", ipsec_username)
                    .expect("Could not write ipsec id to file");
                writeln!(file, "IPSec secret {}", ipsec_password)
                    .expect("Could not write ipsec password to file");
                writeln!(file, "Xauth username {}", username)
                    .expect("Could not write username to file");
                writeln!(file, "Xauth password {}", password).expect("Could not write id to file");
                writeln!(
                    file,
                    "Interface MTU {}",
                    get_options().settings.mtu.to_string()
                )
                .expect("Could not write id to file");
                writeln!(file, "Local Port 0").expect("Could not write port to file");

                // stdout/stderr are inherited so vpnc's progress and errors are
                // visible live; vpnc reopens them on /dev/null when it detaches.
                let status = std::process::Command::new("sudo")
                    .arg("vpnc")
                    .arg(file.path())
                    .status()?;

                if !status.success() {
                    error!("vpnsky could not start");
                    return Err(format!("start: vpnc exited with {}", status).into());
                } else {
                    info!("vpnsky started succesfuly")
                }

                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
