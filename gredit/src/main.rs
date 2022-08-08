

use std::error::Error;
use std::env::args;
use gredit::config;
use std::process;

use gredit::build_log_config;

#[macro_use]
extern crate simple_log;


fn main() -> Result<(), Box<dyn Error>> {
    build_log_config()?;
    let config = config::Config::new(args()).unwrap_or_else( |e|{
        eprintln!("{e}");
        process::exit(1);
    });

    gredit::run(config)?;

    Ok(())
}
