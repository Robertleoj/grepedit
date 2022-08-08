
use std::env::Args;
use std::error::Error;

pub struct Config {
    pub file_path: String,
    pub pattern: String
}

impl Config {
    pub fn new(mut args: Args) 
        // -> Result<Self, Box<dyn Error>>
        -> Result<Self, &'static str>
    {
        args.next().unwrap();

        let pattern = match args.next() {
            None => return Err("No pattern provided"),
            Some(arg) => arg
        };

        let filename = match args.next() {
            None => return Err("No filename provided"),
            Some(arg) => arg
        };

        Ok(Config {
            file_path: filename,
            pattern: pattern
        })
    }
}