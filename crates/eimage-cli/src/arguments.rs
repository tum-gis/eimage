use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None, propagate_version = true)]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Tests
    Test {
        /// Path to an example image
        #[clap(long)]
        file_path: String,

        /// Path to an example image
        #[clap(long)]
        output_file_path: String,
    },
}
