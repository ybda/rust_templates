use std::path::PathBuf;

use clap::{Parser, ValueHint};

use crate::error::Result;
use crate::logging;

#[derive(Parser)]
#[command(version, about, long_about = None, args_conflicts_with_subcommands = true, arg_required_else_help = true)]
pub struct Cli {
    /// Directory to get words from
    #[clap(value_hint = ValueHint::DirPath)]
    pub directory: PathBuf,

    /// Shortcut of --log-level=debug
    #[clap(short, long)]
    verbose: bool,

    /// Valid options are: error, warning, info, debug, trace [default: info]
    #[clap(short, long, conflicts_with = "verbose")]
    log_level: Option<String>,
}

impl Cli {
    pub fn new_with_logging() -> Result<Self> {
        let cli = Cli::parse();
        logging::configure(cli.verbose, cli.log_level.as_ref().map(|x| x.as_str()))?;
        Ok(cli)
    }
}
