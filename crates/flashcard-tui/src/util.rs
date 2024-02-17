//! some good 'ol hardworking functions. Can't live without them
//! won't live without them.
use crate::preamble::*;

pub fn error_handling_setup() -> Result<()> {
    color_eyre::install()?;
    Ok(())
}
