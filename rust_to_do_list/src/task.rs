use chrono;

pub struct Task
{
	  pub name : String,
	  pub is_complete: bool,
	  pub date : chrono::DateTime<chrono::Local>
}

impl Task
{
    pub fn get_name(&mut self) -> String
		{
			return self.name.clone();
		}

		pub fn get_complete(&mut self) -> bool
		{
			return self.is_complete;
		}

		pub fn get_date(&mut self) -> chrono::DateTime<chrono::Local>
		{
			return self.date;
		}
}