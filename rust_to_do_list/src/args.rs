use crate::list::*;
use core::task::*;
use crate::files::*;
use std::path::Path;

//GNU Public Licence v3, 2022, Ruby-Dragon
//
// This source is available for distribution and/or modification
// only under the terms of the RustToDoList Source Code License as
// published by Ruby-Dragon. All rights reserved.

pub const LIST_FILE_NAME : &str= "storedtodolist.lst";

pub fn parse_args(args : &Vec<String>)
{
		let mut open_list : crate::list::TaskList;
		if Path::new(LIST_FILE_NAME).exists(){
			open_list = crate::files::read_list_from_file(LIST_FILE_NAME);
		} else {
			let _file = std::fs::File::create(LIST_FILE_NAME)
				.expect("Failed to create file.");
			open_list = crate::files::read_list_from_file(LIST_FILE_NAME);
		}
		
		if args.len() > 0{
				//if there are any arguments
				if args[0] == "add"{
					if args.len() > 2{
						let new_task = core::task::Task {name : args[1].clone(), is_complete : false, date : chrono::NaiveDate::parse_from_str(&args[2], "%Y-%m-%d")
								.unwrap()};

						open_list.add(new_task);
					}
					else{
						println!("add requires two arguments.")
					}
				}
				else if args[0] == "comp"{
					if args.len() > 1{
						let number = args[1].parse::<usize>().expect("Comp requires an number greater than zero");
						if number <= open_list.task_vec.len() && number > 0 {
							open_list.complete_task(number -1);
						}
						else{
							println!("There is no task at number: {}.", number);
						}
					}
					else{
						println!("comp requires an argument.")
					}
				}
				else if args[0] == "rm"{
					if args.len() > 1
					{
						let number = args[1].parse::<usize>().expect("rm requires an number greater than zero");
						if number <= open_list.task_vec.len() && number > 0 {
							open_list.remove(number -1);
						}
						else{
							println!("There is no task at number: {}.", number);
						}
					}
					else{
						println!("rm requires an argument.");
					}
				}
				
				println!("{}", open_list.to_string());
				crate::files::write_list_to_file(open_list, LIST_FILE_NAME);
		}
		else{
				println!("{}", open_list.to_string());
				crate::files::write_list_to_file(open_list, LIST_FILE_NAME);
		}
}