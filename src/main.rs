use std::env;
use std::process;

use minigrep::{Config, run};
// use log::{debug};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.get_query());
    println!("In file {}", config.get_file_path());
    // debug!("{:?}", args);
    // dbg!(&args);
    
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }   
}

