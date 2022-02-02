use chrono;
use crate::task::*;

pub struct TaskList
{
		pub last_update_date : chrono::NaiveDate,
		pub task_vec : Vec<crate::task::Task>
}

impl TaskList
{
	pub fn to_string(&mut self) -> String
	{
		let mut final_str : String = "".to_string();

		for i in 0..self.task_vec.len()
		{
			let mut next_task = self
					.task_vec[i]
					.clone();
			
			final_str = format!("{}{}\n", final_str, next_task.to_string());
		}

		return final_str;
	}

	pub fn add(&mut self, task_to_add: crate::task::Task)
	{
		self
				.task_vec
				.push(task_to_add);
	}

	pub fn remove(&mut self, index : usize)
	{
		self
				.task_vec
				.remove(index);
	}
}