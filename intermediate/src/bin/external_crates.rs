use std::time::Duration;
use humantime::format_duration;
use chrono::prelude::*;



fn main () {
    let d = Duration::from_secs(1200);
    println!("{}", format_duration(d));
    let local: DateTime<Local> = Local::now();
    println!("{}", local.format("%a %b %e %T %Y").to_string());
}