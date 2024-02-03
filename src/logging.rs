use std::io::Write;

use env_logger::{Builder, Target};
use log::{debug, Level, LevelFilter};

use crate::cli::Cli;
use crate::constants;
use crate::error::{Error, Result};

pub fn configure(cli: &Cli) -> Result<()> {
    let log_level = log_level_from_cli(&cli)?;
    configure_logger(log_level);
    Ok(())
}

fn log_level_from_cli(cli: &Cli) -> Result<LevelFilter> {
    if cli.verbose {
        return Ok(LevelFilter::Debug);
    }

    let log_level = match &cli.log_level {
        None => return Ok(LevelFilter::Info),
        Some(s) => s,
    };

    return Ok(match log_level.as_str() {
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

        if record.level() == Level::Warn {
            writeln!(buf, "{}{}", &warn_prefix, record.args())?;
            return Ok(());
        }
        if record.level() == Level::Trace {
            writeln!(buf, "{}{}", &trace_prefix, record.args())?;
            return Ok(());
        }
        writeln!(buf, "{}", record.args())?;

        Ok(())
    });

    logger.init();
}
