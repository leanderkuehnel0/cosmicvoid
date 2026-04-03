use chrono::{Local, Datelike, Timelike};

fn main() {
    let now = Local::now();
    let list = [(now.ordinal0(), 365), (now.hour(), 24), (now.minute(), 60), (now.second(), 60)];
    let mut total = 1;
    let mut end = 0.0;
    for e in list {
    	total *= e.1;
    	end += e.0 as f64 / total as f64;
    }
    println!("{}", end);
}
