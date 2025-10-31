mod cli;
mod commands;
mod error;

use crate::cli::{Cli, Commands};
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Test {
            file_path,
            output_file_path,
        } => {
            commands::test::run(file_path, output_file_path)?;
        }
    }

    Ok(())
}
