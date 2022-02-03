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

		//all test code and will be removed
    let mut thing = task::Task {name : "urmom".to_string(), is_complete: false, date : chrono::NaiveDate::parse_from_str("2022-02-01", "%Y-%m-%d").unwrap()};

		println!("{}", thing.to_string());

		let mut thing_list = list::TaskList {last_update_date : chrono::NaiveDate::parse_from_str("2022-02-01", "%Y-%m-%d").unwrap(), task_vec : vec![thing.clone()]};

		thing_list.add(thing.clone());
		thing_list.remove(0);

		thing_list.complete_task(0);

		println!("{}", thing_list.to_string());

		files::write_list_to_file(thing_list, args::list_file_name);

		let mut another_list = files::read_list_from_file(args::list_file_name);

		println!("{}", another_list.to_string());

		args::parse_args(&args);

		println!(" ");

		for arg in args
		{
			println!("{}", arg);
		}
}
