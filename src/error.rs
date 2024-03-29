use std::io::Write;

use thiserror::Error;

use crate::constants;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error(transparent)]
    Io(#[from] ::std::io::Error),

    #[error(transparent)]
    Fmt(#[from] ::std::fmt::Error),

    #[error("{0}")]
    Msg(String),
}

impl From<&'static str> for Error {
    fn from(s: &'static str) -> Self {
        Error::Msg(s.to_owned())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Msg(s)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn default_error_handler(error: &Error, output: &mut impl Write) {
    let err_prefix = nu_ansi_term::Color::Red.paint(constants::ERR_PREFIX);
    writeln!(output, "{}{}", err_prefix, error).ok();
}
