use clap::{Parser, Subcommand, ValueHint};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None, propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Tests
    Test {
        /// Path to an example image
        #[clap(long, value_hint = ValueHint::FilePath)]
        file_path: PathBuf,

        /// Path to an example image
        #[clap(long, value_hint = ValueHint::FilePath)]
        output_file_path: PathBuf,
    },
}
