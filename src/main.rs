mod cli;
mod tools;
mod core;
mod enki_vm;

use cli::matches::match_command;
use clap::Parser;
use cli::args::Args;

fn main() -> Result<(), Box<dyn std::error::Error>>{

    let cmd = Args::parse();

    match_command(&cmd.command)?;

    Ok(())

}
