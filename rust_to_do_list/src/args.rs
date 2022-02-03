use crate::list::*;
use crate::task::*;
use crate::files::*;

pub fn parse_args(args : &Vec<String>)
{

		const list_file_name : &str= "storedtodolist.lst";

		if args.len() > 0{
				//if there are any arguments
				println!("{}", args[0]);
		}
		else{
				//print list here println!()
		}
}