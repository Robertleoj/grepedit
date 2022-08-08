
use simple_log::LogConfigBuilder;
use std::error::Error;
pub mod app;

use std::io;

#[macro_use]
extern crate simple_log;

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


pub fn run() -> Result<(), Box<dyn Error>> {
    let mut app = app::App::new(vec![
        app::MatchItem{
            location: app::Location{
                line_nr: 10, 
                file_path: "hello.py".to_string()
            }, 
            match_text: "def a():".to_string()
        },
        app::MatchItem{
            location: app::Location{
                line_nr: 20, 
                file_path: "index.html".to_string()
            }, 
            match_text: "<def>".to_string()
        },
    ]);

    app.run()?;

    Ok(())
}



