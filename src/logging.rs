use std::io::Write;

use env_logger::{Builder, Target};
use log::{Level, LevelFilter};

use crate::constants;
use crate::error::{Error, Result};

pub fn configure(verbose: bool, log_level: Option<&str>) -> Result<()> {
    let log_level = log_level_from_cli(verbose, log_level)?;
    configure_logger(log_level);
    Ok(())
}

fn log_level_from_cli(verbose: bool, log_level: Option<&str>) -> Result<LevelFilter> {
    if verbose {
        return Ok(LevelFilter::Debug);
    }

    let log_level = match log_level {
        None => return Ok(LevelFilter::Info),
        Some(s) => s,
    };

    return Ok(match log_level {
        "error" => LevelFilter::Error,
        "warning" | "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => return Err(Error::from("Invalid log level was provided.")),
    });
}

fn configure_logger(log_level: LevelFilter) {
    let mut logger = Builder::new();

    // Make all logs write into stdout
    logger.target(Target::Stdout);

    logger.filter(None, log_level);

    let warn_prefix = nu_ansi_term::Color::Yellow.paint(constants::WARN_PREFIX);
    let trace_prefix = nu_ansi_term::Color::DarkGray.paint(constants::TRACE_PREFIX);

    logger.format(move |buf, record| {
        // Using error! macro is not expected. It's useless because of the crate::error
        // module
        debug_assert_ne!(record.level(), Level::Error);

        match record.level() {
            Level::Trace => {
                writeln!(buf, "{}{}", &trace_prefix, record.args())?;
                return Ok(());
            }
            Level::Warn => {
                writeln!(buf, "{}{}", &warn_prefix, record.args())?;
                return Ok(());
            }
            _ => {
                writeln!(buf, "{}", record.args())?;
                return Ok(());
            }
        }
    });

    logger.init();
}
