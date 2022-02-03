use crate::task::*;
use crate::list::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use chrono;
use std::io::Read;

pub fn write_list_to_file(mut task_list : TaskList, filename : String){

		let mut file = std::fs::File::create(&filename)
				.expect("Failed to create file.");

		let mut final_file_output : String = String::new();
		
		for task in &task_list.task_vec{
				final_file_output = format!("{}\t{}\t{}\n", task.is_complete, task.name, task.date);
		}

		file.write_all(final_file_output
				.as_bytes())
				.expect("Failed to write to file.");
}

pub fn read_list_from_file(filename : String) -> TaskList{

		let mut file = std::fs::File::open(&filename)
				.expect("File cannot be opened, file may not exist.");

		let mut whole_file_text : String = String::new();

		file
				.read_to_string(&mut whole_file_text)
				.expect("Error in reading data, file may be corrupted or incorrectly encoded.");

		let mut split = whole_file_text.lines();

		let lines_vec : Vec<&str> = split.collect();

		let mut final_task_list : crate::list::TaskList = crate::list::TaskList {last_update_date : chrono::NaiveDate::parse_from_str("2022-02-01", "%Y-%m-%d").unwrap(), task_vec : Vec::new()};

		for line in &lines_vec{
			let mut split_two = line.split("\t");
			let line_vec : Vec<&str> = split_two.collect();

			let comp : bool = line_vec[0].parse::<bool>()
					.unwrap();

			let task_name : String = line_vec[1]
					.to_string();

			let task_date : chrono::NaiveDate = chrono::NaiveDate::parse_from_str(&line_vec[2], "%Y-%m-%d").unwrap();

			let current_task : crate::task::Task = crate::task::Task {name : task_name, is_complete : comp, date : task_date};
			final_task_list.add(current_task);
		}

		return final_task_list;
}