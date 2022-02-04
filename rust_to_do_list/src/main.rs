use std::env;
mod args;
mod task;
use chrono;
mod list;
mod files;

fn main() {
		//get cli args, and remove exec name
	  let mut args: Vec<String> = env::args().collect();
		args.remove(0);

		args::parse_args(&args);
		
		/*
		for arg in args
		{
			println!("{}", arg);
		}
		*/
}
