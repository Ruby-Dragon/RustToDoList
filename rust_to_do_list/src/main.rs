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

    println!("Hello, world!");

    let mut thing = task::Task {name : "urmom".to_string(), is_complete: false, date : chrono::NaiveDate::parse_from_str("2022-02-01", "%Y-%m-%d").unwrap()};

		thing.is_complete = true;

		println!("{}", thing.to_string());

		let mut thing_list = list::TaskList {last_update_date : chrono::NaiveDate::parse_from_str("2022-02-01", "%Y-%m-%d").unwrap(), task_vec : vec![thing.clone()]};

		thing_list.add(thing.clone());
		thing_list.remove(0);

		println!("{}", thing_list.to_string());

		files::write_list_to_file(thing_list, "test.txt".to_string());

		for arg in args
		{
			println!("{}", arg);
		}
}
