mod cli;
mod commands;
mod error;

use crate::cli::{Cli, Commands};
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Test {
            file_path,
            output_file_path,
        } => {
            let file_path = PathBuf::from(file_path);
            let output_file_path = PathBuf::from(output_file_path);

            commands::test::run(file_path, output_file_path)?;
        }
    }

    Ok(())
}
