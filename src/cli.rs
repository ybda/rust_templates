use std::path::PathBuf;

use clap::{Parser, ValueHint};

#[derive(Parser)]
#[command(version, about, long_about = None, args_conflicts_with_subcommands = true, arg_required_else_help = true)]
pub struct Cli {
    /// Directory to get words from
    #[clap(value_hint = ValueHint::DirPath)]
    pub directory: PathBuf,

    /// Shortcut of --log-level=debug
    #[clap(short, long)]
    pub verbose: bool,

    /// Valid options are: error, warning, info, debug, trace [default: info]
    #[clap(short, long, conflicts_with = "verbose")]
    pub log_level: Option<String>,
}
