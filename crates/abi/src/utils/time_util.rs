use chrono::{DateTime, Local};

pub fn now() -> DateTime<Local>  {
	Local::now()
}

pub fn get_current_time_formatted() -> String {
	return Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
