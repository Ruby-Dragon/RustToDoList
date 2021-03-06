//GNU Public Licence v3, 2022, Ruby-Dragon
//
// This source is available for distribution and/or modification
// only under the terms of the RustToDoList Source Code License as
// published by Ruby-Dragon. All rights reserved.

use crate::task::*;
use crate::list::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use chrono;
use chrono::Datelike;
use std::io::Read;

//write a list to a file in a machine readable format
pub fn write_list_to_file(task_list : TaskList, filename : &str){

		let mut file = std::fs::File::create(&filename)
				.expect("Failed to create file.");

		let mut final_file_output : String = String::new();
		
		for task in &task_list.task_vec{
				final_file_output = format!("{}{}\t{}\t{}\n", final_file_output, task.is_complete, task.name, task.date);
		}

		file.write_all(final_file_output
				.as_bytes())
				.expect("Failed to write to file.");
}

//read a list from a file and turn into objects
pub fn read_list_from_file(filename : &str) -> TaskList{

		//open the file
		let mut file = std::fs::File::open(&filename)
				.expect("File cannot be opened, file may not exist.");

		//whole text of a file
		let mut whole_file_text : String = String::new();

		file
				.read_to_string(&mut whole_file_text)
				.expect("Error in reading data, file may be corrupted or incorrectly encoded.");

		//split at new lines
		let split = whole_file_text.lines();

		let lines_vec : Vec<&str> = split.collect();

		//the list to be returned
		let mut final_task_list : crate::list::TaskList = crate::list::TaskList {last_update_date : chrono::NaiveDate::from_ymd(chrono::Local::now().year(), chrono::Local::now().month(), chrono::Local::now().day() ), task_vec : Vec::new()};

		//go through each line and get task object in it
		for line in &lines_vec{
			let split_two = line.split("\t");
			let line_vec : Vec<&str> = split_two.collect();

			if line_vec.len() >= 3{
				let comp : bool = line_vec[0].parse::<bool>()
					.unwrap();

				let task_name : String = line_vec[1]
						.to_string();

				let task_date : chrono::NaiveDate = chrono::NaiveDate::parse_from_str(&line_vec[2], "%Y-%m-%d").unwrap();

				let current_task : crate::task::Task = crate::task::Task {name : task_name, is_complete : comp, date : task_date};
				if !(current_task.is_complete && current_task.date < final_task_list.last_update_date)
				{
					final_task_list.add(current_task);
				}
			}
		}

		return final_task_list;
}