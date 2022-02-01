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

    let mut thing = task::Task {name : "urmom".to_string(), is_complete: false, date : chrono::Local::now()};

		println!("{} , {} , {}", thing.get_name(), thing.get_complete(), thing.get_date();

		for arg in args
		{
			println!("{}", arg);
		}
}
