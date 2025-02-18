use chrono::Local;

pub fn get_current_time_formatted() -> String {
	return Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
