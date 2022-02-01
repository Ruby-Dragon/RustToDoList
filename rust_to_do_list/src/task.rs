use chrono;
use ansi_term;

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
				final_str = format!("{}{},\tDue: {}", final_str, self.name, self.date,);
			}
			else
			{
				final_str = format!("{}{},\tDue: {}", final_str, self.name, self.date,);
			}
			
			return final_str;
	  }
}