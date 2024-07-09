mod arguments;
mod commands;

use crate::arguments::{Arguments, Commands};
use clap::Parser;
use std::path::PathBuf;

fn main() {
    tracing_subscriber::fmt::init();
    let arguments = Arguments::parse();

    match &arguments.command {
        Commands::Test {
            file_path,
            output_file_path,
        } => {
            let file_path = PathBuf::from(file_path);
            let output_file_path = PathBuf::from(output_file_path);

            commands::test::run(file_path, output_file_path);
        }
    }
}
