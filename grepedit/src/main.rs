


use std::error::Error;

use grepedit::build_log_config;
#[macro_use]
extern crate simple_log;



fn main() -> Result<(), Box<dyn Error>> {
    build_log_config()?;

    info!("started app");

    grepedit::run()?;

    Ok(())
}
