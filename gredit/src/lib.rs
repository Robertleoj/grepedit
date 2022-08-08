
use simple_log::LogConfigBuilder;
use std::error::Error;
pub mod app;

pub mod searcher;
pub mod config;

use config::Config;
use searcher::search_file;

use std::io;

#[macro_use]
extern crate simple_log;


pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {

    let result = search_file(
        &config.file_path,
        &config.pattern
    ).unwrap();

    let mut app = app::App::new(
        vec![result]
    );

    app.run()?;

    Ok(())
}


pub fn build_log_config() -> Result<(), Box<dyn Error>>{
    let config = LogConfigBuilder::builder()
        .path("./project.log")
        .size(1 * 100)
        .roll_count(10)
        .time_format("%Y-%m-%d %H:%M:%S.%f") //E.g:%H:%M:%S.%f
        .level("debug")
        .output_file()
        .build();

    simple_log::new(config)?;
    Ok(())
}

