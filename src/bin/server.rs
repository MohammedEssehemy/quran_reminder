//! A simple Rocket application, based on the example in [Getting Started][].
//!
//! [Getting Started]: https://rocket.rs/v0.5-rc/guide/getting-started/
extern crate job_scheduler;
use job_scheduler::{Job, JobScheduler};
use dotenv::dotenv;
use rocket::{get, launch, routes, time::Instant};
use std::thread;
use quran_reminder::{config::Config, run_once};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    thread::spawn(trigger_email_cron);
    rocket::build().mount("/", routes![index])
}

fn trigger_email_cron() {
    let config = Config::init();
    println!("{config:?}");
    let mut sched = JobScheduler::new();
    // every day at 6:10:10 GMT
    sched.add(Job::new("10 10 6 * * * *".parse().unwrap(), || {
        println!("running reminder job at: {:?}", Instant::now());
        if let Err(e) = run_once(&config) {
            eprintln!("reminder cron filed with error: {}", e);
        }
    }));
    loop {
        sched.tick();
        println!("will sleep for {:?} ", sched.time_till_next_job());
        thread::sleep(sched.time_till_next_job());
    }
}
