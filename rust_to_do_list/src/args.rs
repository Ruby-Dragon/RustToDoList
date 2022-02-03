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
		}
		else{
				println!("{}", open_list.to_string());
		}
}