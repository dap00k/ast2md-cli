//! Command line subcommands and options
//!
//! Options:
//! - level: Select the log level
//!
//! Subcommands:
//! - manual: Use the manual parser
//! - generated: Use the pest generated parser
//!
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub struct CliArgs {
    /// Verbosity level
    #[clap(short, long, value_enum, value_name = "LEVEL", default_value = "warn")]
    pub level: log::LevelFilter,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Use manual parser
    Manual {
        /// Path to dump-ast file
        filename: Option<PathBuf>,
    },
    /// Use generated parser (pest)
    Generated {
        /// Path to dump-ast file
        filename: Option<PathBuf>,
    },
}
