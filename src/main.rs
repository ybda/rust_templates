mod cli;
mod constants;
mod error;
mod logging;
use std::process;

use error::{Error, Result};
use log::{debug, info, trace, warn};

use crate::cli::Cli;

fn main() {
    run().unwrap_or_else(|e| {
        error::default_error_handler(&e, &mut std::io::stderr().lock());
        process::exit(1);
    })
}

fn run() -> Result<()> {
    let cli = Cli::new_with_logging()?;

    warn!("warn");
    info!("info");
    debug!("debug");
    trace!("trace");

    info!("dir is `{}`", cli.directory.to_string_lossy());
    info!("maxlevel is `{}`", log::max_level());

    Err(Error::from("test error"))
    //Ok(())
}
