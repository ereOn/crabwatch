//! A command-line utility to manipulate GPIO chips.

use anyhow::Result;
use clap::{Parser, Subcommand};

use crabwatch_libgpiod_sys::api_version;

#[derive(Parser)]
pub struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    ApiVersion,
}

fn main() -> Result<()> {
    let args = Args::try_parse()?;

    match args.command {
        Command::ApiVersion => {
            println!("libgpiod API version: {}", api_version());
        }
    }

    Ok(())
}
