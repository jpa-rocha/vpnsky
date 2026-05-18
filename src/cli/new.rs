use vpnsky::config::get_options;
use vpnsky::errors::{AppErrors, FolderCreationError};
use vpnsky::{create_folder, create_input_file};

use clap::Args;

#[derive(Args, Debug)]
pub struct NewCmd {
    #[arg(short, long)]
    year: u16,
}

impl NewCmd {
    todo!();
}
