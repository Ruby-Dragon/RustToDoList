use std::env;
mod args;
mod task;
use chrono;

fn main() {
		//get cli args, and remove exec name
	  let mut args: Vec<String> = env::args().collect();
		args.remove(0);

    println!("Hello, world!");

    let mut thing = task::Task {name : "urmom".to_string(), is_complete: false, date : chrono::NaiveDate::parse_from_str("2022-02-01", "%Y-%m-%d").unwrap()};

		thing.is_complete = true;

		println!("{}", thing.to_string());

		for arg in args
		{
			println!("{}", arg);
		}
}
