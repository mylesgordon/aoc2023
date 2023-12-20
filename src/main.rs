use std::env;

mod day;
mod day1;
mod day2;

pub use day::*;

fn get_day(day: &str) -> Box<dyn Solve> {
    match day {
        "1" => Box::new(day1::DayOne::new()),
        "2" => Box::new(day2::DayTwo::new()),
        _ => unimplemented!("Day {} is not available", day)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("You must provide one argument - the day to be solved");
    }

    let day = get_day(&args[1]);
    println!("{}", day.solve());
}
