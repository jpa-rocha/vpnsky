mod reload;
mod start;
mod stop;

use clap::{Args as ClapArgs, Parser, Subcommand};
use spdlog::info;
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, ToSocketAddrs};
use std::path::Path;
use std::process::Command;
use reload::ReloadCmd;
use start::StartCmd;
use stop::StopCmd;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(ClapArgs, Debug)]
pub struct Args {}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Start(StartCmd),
    Stop(StopCmd),
    Reload(ReloadCmd),
}

const VPNC_PID_FILE: &str = "/run/vpnc.pid";

/// Returns the pid of the running vpnc, if any.
///
/// The pid file alone is not enough: when vpnc dies on its own (e.g. the
/// wired link changes and DPD gives up) the file is left behind, and
/// `vpnc-disconnect` then fails with "no vpnc found running".
fn running_vpnc_pid() -> Option<u32> {
    vpnc_pid_from(Path::new(VPNC_PID_FILE), Path::new("/proc"))
}

fn vpnc_pid_from(pid_file: &Path, proc_root: &Path) -> Option<u32> {
    let pid: u32 = std::fs::read_to_string(pid_file).ok()?.trim().parse().ok()?;
    let comm = std::fs::read_to_string(proc_root.join(pid.to_string()).join("comm")).ok()?;
    (comm.trim() == "vpnc").then_some(pid)
}

fn is_alive(pid: u32) -> bool {
    Path::new("/proc").join(pid.to_string()).exists()
}

/// Deletes host routes to the gateway left behind by a previous session.
///
/// vpnc-script pins the gateway to the uplink that was default at connect
/// time and does not always remove it (e.g. when vpnc dies on its own). If the
/// uplink changed since, new connections go out the wrong interface and fail
/// with "no response from target". Must only be called while vpnc is stopped.
fn remove_stale_gateway_route(gateway: &str) -> Result<(), Box<dyn Error>> {
    for ip in gateway_ipv4s(gateway)? {
        let ip = ip.to_string();
        let output = Command::new("ip")
            .args(["route", "show", "exact", &ip])
            .output()?;

        // vpnc-script may add one route per uplink; `ip route del` drops one
        // at a time, so delete once per listed route.
        for route in String::from_utf8_lossy(&output.stdout).lines() {
            info!("removing stale route to vpn gateway: {}", route);
            let status = Command::new("sudo")
                .args(["ip", "route", "del", &ip])
                .status()?;
            if !status.success() {
                return Err(format!("start: could not remove stale route to {}", ip).into());
            }
        }
    }
    Ok(())
}

fn gateway_ipv4s(gateway: &str) -> Result<Vec<Ipv4Addr>, Box<dyn Error>> {
    let ips = (gateway, 0)
        .to_socket_addrs()?
        .filter_map(|addr| match addr.ip() {
            IpAddr::V4(ip) => Some(ip),
            IpAddr::V6(_) => None,
        })
        .collect::<Vec<_>>();
    Ok(ips)
}

#[cfg(test)]
mod tests {
    use super::{gateway_ipv4s, vpnc_pid_from};
    use std::net::Ipv4Addr;
    use std::fs;
    use tempdir::TempDir;

    fn setup(pid_file: Option<&str>, comm: Option<&str>) -> TempDir {
        let dir = TempDir::new("vpnsky").unwrap();
        if let Some(content) = pid_file {
            fs::write(dir.path().join("vpnc.pid"), content).unwrap();
        }
        if let Some(name) = comm {
            fs::create_dir_all(dir.path().join("proc/4437")).unwrap();
            fs::write(dir.path().join("proc/4437/comm"), name).unwrap();
        }
        dir
    }

    fn check(dir: &TempDir) -> Option<u32> {
        vpnc_pid_from(&dir.path().join("vpnc.pid"), &dir.path().join("proc"))
    }

    #[test]
    fn running_vpnc_is_detected() {
        assert_eq!(check(&setup(Some("4437\n"), Some("vpnc\n"))), Some(4437));
    }

    #[test]
    fn stale_pid_file_is_not_running() {
        assert_eq!(check(&setup(Some("4437\n"), None)), None);
    }

    #[test]
    fn reused_pid_is_not_running() {
        assert_eq!(check(&setup(Some("4437\n"), Some("bash\n"))), None);
    }

    #[test]
    fn gateway_literal_ip_resolves_to_itself() {
        assert_eq!(
            gateway_ipv4s("62.176.249.5").unwrap(),
            vec![Ipv4Addr::new(62, 176, 249, 5)]
        );
    }

    #[test]
    fn missing_or_garbage_pid_file_is_not_running() {
        assert_eq!(check(&setup(None, Some("vpnc\n"))), None);
        assert_eq!(check(&setup(Some(""), Some("vpnc\n"))), None);
    }
}
