use crate::task::*;
use crate::list::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn write_list_to_file(mut task_list : TaskList, filename : String)
{
		let mut file = std::fs::File::create(&filename).unwrap();
		file.write_all(task_list.to_string().as_bytes());
}