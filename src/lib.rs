
use std::error::Error;
use std::{fs, };



pub fn run(config:Config) -> Result<(), Box<dyn Error>>{
 let contents:String = fs::read_to_string(config.filename) ?;

    for line in search(&config.query, &contents){
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query:&str, content: &'a str ) -> Vec<&'a str>{
    let mut results: Vec<&str> = Vec:: new();
    for line in content.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

pub struct Config {
    pub query : String,
    pub filename : String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str>
{
    if args.len() <3 {
        return Err("Not enough arguments");
    }
    let query :String = args[1].clone();
    let filename :String = args[2].clone();

    Ok(Config { query: query, filename: filename })
}
}


