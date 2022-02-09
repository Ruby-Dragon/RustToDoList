//GNU Public Licence v3, 2022, Ruby-Dragon
//
// This source is available for distribution and/or modification
// only under the terms of the RustToDoList Source Code License as
// published by Ruby-Dragon. All rights reserved.

use chrono;
use ansi_term;

//let cloning happen on task - probably not neccessary after testing
#[derive(Clone)]
pub struct Task
{
		//name of task
	  pub name : String,
		//completeness status
	  pub is_complete: bool,
		//date it is supposed to be completed by
	  pub date : chrono::NaiveDate
}

//implement
impl Task
{
		//to string method
	  pub fn to_string(&mut self) -> String
	  {
			//final string to be returned
			let mut final_str : String = "".to_string();
			//if it is complete, add strikethrough
			if self.is_complete
			{
				final_str = format!("{}{},\tDue: {}", ansi_term::Style::new()
						.strikethrough()
						.paint(final_str), self.name, self.date);
				
				final_str = format!("{}", ansi_term::Style::new()
						.strikethrough()
						.paint(final_str));
			}
			else //no strikethrough
			{
				final_str = format!("{}{},\tDue: {}", final_str, self.name, self.date,);
			}
			
			return final_str; //return the final string
	  }
}