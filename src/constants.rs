use shadow_rs::formatcp;

const CARGO_PKG_NAME_ENV: &str = env!("CARGO_PKG_NAME");

pub const ERR_PREFIX: &str = formatcp!("[{} error]: ", CARGO_PKG_NAME_ENV);
pub const WARN_PREFIX: &str = formatcp!("[{} warn]: ", CARGO_PKG_NAME_ENV);
pub const TRACE_PREFIX: &str = formatcp!("[{} trace]: ", CARGO_PKG_NAME_ENV);
