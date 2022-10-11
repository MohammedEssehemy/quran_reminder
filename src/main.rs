use dotenv::dotenv;
use std::process;
use quran_reminder::{Config, run_once};


fn main() {
    dotenv().ok();
    let config = Config::init();
    println!("{config:#?}");
    if let Err(e) = run_once(&config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
    println!("done successfully");
}
