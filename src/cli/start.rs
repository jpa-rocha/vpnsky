use std::error::Error;
use vpnsky::config::get_options;

use clap::Args;
use vpnsky::secrets::get_sops_secrets;

#[derive(Args, Debug)]
pub struct StartCmd {}

// "--gateway",
// config.Opt.Settings.Address,
// "--id",
// config.Opt.Secrets.VPN.IPSec.Username,
// "--secret",
// config.Opt.Secrets.VPN.IPSec.Password,
// "--username",
// config.Opt.Secrets.VPN.ID.Username,
// "--password",
// config.Opt.Secrets.VPN.ID.Password,
// "--ifmtu",
// strconv.Itoa(config.Opt.Settings.MTU),
impl StartCmd {
    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
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

        let _output = std::process::Command::new("sudo")
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

        Ok(())
    }
}
