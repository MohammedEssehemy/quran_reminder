use dotenv::dotenv;
use humantime::{format_duration, format_rfc3339};
use job_scheduler::{Job, JobScheduler};
use quran_reminder::{run_once, Config};
use rocket::{get, launch, routes};
use std::{env, panic, process, thread, time::SystemTime};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    panic_handler();
    dotenv().ok();
    thread::spawn(|| {
        println!("spawning another thread for the cronjob \"trigger_reminder_cron\"");
        trigger_reminder_cron();
    });
    rocket::build().mount("/", routes![index])
}

fn trigger_reminder_cron() {
    let config = Config::init();
    println!("{config:#?}");
    let mut sched = JobScheduler::new();
    // every day at 6:10:10 GMT
    let cron_pattern = env::var("CRON_PATTERN").unwrap_or("10 10 6 * * * *".into());
    println!("cron_pattern: {cron_pattern:#?}");
    sched.add(Job::new(cron_pattern.parse().unwrap(), || {
        println!(
            "running reminder job at: {}",
            format_rfc3339(SystemTime::now())
        );
        if let Err(e) = run_once(&config) {
            eprintln!("reminder cron filed with error: {}", e);
        }
    }));
    loop {
        sched.tick();
        let remaining_time = sched.time_till_next_job();
        println!("will sleep for {}", format_duration(remaining_time));
        thread::sleep(remaining_time);
    }
}

fn panic_handler() {
    // https://stackoverflow.com/questions/35988775/how-can-i-cause-a-panic-on-a-thread-to-immediately-end-the-main-thread
    // take_hook() returns the default hook in case when a custom one is not set
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // invoke the default handler and exit the process
        orig_hook(panic_info);
        println!("one thread panicked, closing the main thread");
        process::exit(1);
    }));
}
