use std::env;

mod day;
mod day1;

pub use day::*;

fn get_day(day: &str) -> impl Solve {
    match day {
        "1" => day1::DayOne::new(),
        _ => unimplemented!("Day {} is not available", day)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("You must provide only one argument - the day to be solved");
    }

    let day = get_day(&args[1]);
    println!("{}", day.solve());
}
