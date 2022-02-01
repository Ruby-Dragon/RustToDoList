use std::env;
mod args;
mod task;
use chrono;

fn main() {
	  let mut args: Vec<String> = env::args().collect();
		args.remove(0);
		let mut cstyleint:i32 = 20;
    println!("Hello, world! {}", cstyleint);

		let result = args::return_string(23);
		println!("{}", result);

    let thing = task::Task {name : "urmom".to_string(), is_complete: false, date : chrono::Utc::now()};

		println!("{}", thing.name);

		for arg in args
		{
			println!("{}", arg);
		}
}
