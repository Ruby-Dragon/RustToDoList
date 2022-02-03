use crate::list::*;
use crate::task::*;
use crate::files::*;

pub const LIST_FILE_NAME : &str= "storedtodolist.lst";

pub fn parse_args(args : &Vec<String>)
{

		let mut open_list : crate::list::TaskList = crate::files::read_list_from_file(LIST_FILE_NAME);

		if args.len() > 0{
				//if there are any arguments
				if (args[0] == "add")
				{

				}
				else if (args[0] == "comp")
				{

				}
				
				println!("{}", open_list.to_string());
		}
		else{
				println!("{}", open_list.to_string());
		}
}