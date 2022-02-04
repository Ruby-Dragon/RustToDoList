use crate::list::*;
use crate::task::*;
use crate::files::*;

pub const LIST_FILE_NAME : &str= "storedtodolist.lst";

pub fn parse_args(args : &Vec<String>)
{

		let mut open_list : crate::list::TaskList = crate::files::read_list_from_file(LIST_FILE_NAME);

		if args.len() > 0{
				//if there are any arguments
				if args[0] == "add"{
					if args.len() > 2{
						let new_task = crate::task::Task {name : args[1].clone(), is_complete : false, date : chrono::NaiveDate::parse_from_str(&args[2], "%Y-%m-%d")
								.unwrap()};

						open_list.add(new_task);
					}
					else{
						println!("add requires two arguments.")
					}
				}
				else if args[0] == "comp"{
					if args.len() > 1{
						
					}
					else{
						println!("comp requires an argument.")
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