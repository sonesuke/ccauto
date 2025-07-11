use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Path to config YAML file
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,

    /// Enable debug logging for internal details
    #[arg(short, long, global = true)]
    pub debug: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Load and display rules
    Show(ShowArgs),
}

#[derive(Args, Debug)]
pub struct ShowArgs {
    /// Path to config YAML file
    #[arg(long, default_value = "config.yaml")]
    pub config: PathBuf,
}
