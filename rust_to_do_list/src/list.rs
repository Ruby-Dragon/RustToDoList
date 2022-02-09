//GNU Public Licence v3, 2022, Ruby-Dragon
//
// This source is available for distribution and/or modification
// only under the terms of the RustToDoList Source Code License as
// published by Ruby-Dragon. All rights reserved.

use chrono;
use crate::task::*;

pub struct TaskList
{
		//date last updated, not used all that much atm
		pub last_update_date : chrono::NaiveDate,
		//vector of tasks
		pub task_vec : Vec<crate::task::Task>
}

impl TaskList
{
	//to string, call to string on tasks in vector
	pub fn to_string(&mut self) -> String
	{
		let mut final_str : String = String::new();

		for i in 0..self.task_vec.len()
		{
			let mut next_task = self
					.task_vec[i]
					.clone();
			
			final_str = format!("{}{}. {}\n", final_str, i + 1, next_task.to_string());
		}

		return final_str;
	}

	//add task to list
	pub fn add(&mut self, task_to_add: crate::task::Task)
	{
		self
				.task_vec
				.push(task_to_add);
	}

	//remove task from list
	pub fn remove(&mut self, index : usize)
	{
		self
				.task_vec
				.remove(index);
	}

	//complete the task
	pub fn complete_task(&mut self, index : usize)
	{
		self
				.task_vec[index]
				.is_complete = true;
	}
}