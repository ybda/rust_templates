mod error;
use std::process;

use error::{Result,Error};

fn main() {
    run().unwrap_or_else(|e| {
        error::default_error_handler(&e, &mut std::io::stderr().lock());
        process::exit(1);
    })
}

fn run() -> Result<()> {
    println!("Hello world!");

    Err(Error::from("Test error"))

    //Ok(())
}
