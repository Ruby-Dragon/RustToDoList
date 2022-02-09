//GNU Public Licence v3, 2022, Ruby-Dragon
//
// This source is available for distribution and/or modification
// only under the terms of the RustToDoList Source Code License as
// published by Ruby-Dragon. All rights reserved.

use chrono;
use ansi_term;

//let cloning happen on task - probably not neccessary
#[derive(Clone)]
pub struct Task
{
	  pub name : String,
	  pub is_complete: bool,
	  pub date : chrono::NaiveDate
}

impl Task
{
	  pub fn to_string(&mut self) -> String
	  {
			let mut final_str : String = "".to_string();
			if self.is_complete
			{
				final_str = format!("{}{},\tDue: {}", ansi_term::Style::new()
						.strikethrough()
						.paint(final_str), self.name, self.date);
				
				final_str = format!("{}", ansi_term::Style::new()
						.strikethrough()
						.paint(final_str));
			}
			else
			{
				final_str = format!("{}{},\tDue: {}", final_str, self.name, self.date,);
			}
			
			return final_str;
	  }
}