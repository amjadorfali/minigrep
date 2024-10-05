use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    println!("file contents are \n{contents}");

    Ok(())
}

#[derive(Debug)]
pub struct Config {
    pub search: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let search = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { search, file_path })
    }
}
