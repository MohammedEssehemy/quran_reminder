extern crate dotenv;
use dotenv::dotenv;
use std::process;
use quran_reminder::{config::Config, run};


fn main() {
    dotenv().ok();
    let config = Config::init();
    println!("{config:?}");
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
    println!("done successfully");
}
