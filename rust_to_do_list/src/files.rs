use crate::task::*;
use crate::list::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn write_list_to_file(mut task_list : TaskList, filename : String)
{
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