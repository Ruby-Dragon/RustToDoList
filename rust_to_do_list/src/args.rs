//GNU Public Licence v3, 2022, Ruby-Dragon
//
// This source is available for distribution and/or modification
// only under the terms of the RustToDoList Source Code License as
// published by Ruby-Dragon. All rights reserved.

use crate::list::*;
use crate::task::*;
use crate::files::*;
use std::path::Path;

//the name of the file lists are stored, const because it never changes
pub const LIST_FILE_NAME : &str= "storedtodolist.lst";

//parse the arguments passed in
pub fn parse_args(args : &Vec<String>)
{
		//Current list opened
		let mut open_list : crate::list::TaskList;
		//check if file exists
		if Path::new(LIST_FILE_NAME).exists(){
			open_list = crate::files::read_list_from_file(LIST_FILE_NAME);
		} else {
			let _file = std::fs::File::create(LIST_FILE_NAME)
				.expect("Failed to create file.");
			open_list = crate::files::read_list_from_file(LIST_FILE_NAME);
		}
		
		//execute arguments
		if args.len() > 0{
				//if there are any arguments
				if args[0] == "add"{
					if args.len() > 2{
						//make a new task
						let new_task = crate::task::Task {name : args[1].clone(), is_complete : false, date : chrono::NaiveDate::parse_from_str(&args[2], "%Y-%m-%d")
								.unwrap()};

						//add task to list
						open_list.add(new_task);
					}
					else{
						//if user used it wrong
						println!("add requires two arguments.")
					}
				}
				else if args[0] == "comp"{
					if args.len() > 1{
						//index number of task to be completed
						let number = args[1].parse::<usize>().expect("Comp requires an number greater than zero");
						//complete task if it exists
						if number <= open_list.task_vec.len() && number > 0 {
							open_list.complete_task(number -1);
						}
						else{
							//if user uses it wrong
							println!("There is no task at number: {}.", number);
						}
					}
					else{
						//if user uses it wrong
						println!("comp requires an argument.")
					}
				}
				else if args[0] == "rm"{
					if args.len() > 1
					{
						//index of task to be removed
						let number = args[1].parse::<usize>().expect("rm requires an number greater than zero");
						if number <= open_list.task_vec.len() && number > 0 {
							//remove the task at index -1 if it exists
							open_list.remove(number -1);
						}
						else{
							//if user uses it wrong
							println!("There is no task at number: {}.", number);
						}
					}
					else{
						//if the user uses it wrong
						println!("rm requires an argument.");
					}
				}
				//print the list and save it in the file
				print!("{}", open_list.to_string());
				crate::files::write_list_to_file(open_list, LIST_FILE_NAME);
		}
		else{
				//print the list and save it in the file
				print!("{}", open_list.to_string());
				crate::files::write_list_to_file(open_list, LIST_FILE_NAME);
		}
}