mod start;
mod stop;

use clap::{Args as ClapArgs, Parser, Subcommand};
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
}

fn check_vpn_pid() -> bool {
    return std::path::Path::new("/var/run/vpnc.pid").exists();
}
