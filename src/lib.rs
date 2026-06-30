use chrono::prelude::*;

pub trait CustomTime {
    fn new() -> Option<DateTime<Local>>;
    fn get_string_format(custom_time: &DateTime<Local>) -> String;
}
impl CustomTime for DateTime<Local> {
    // set time for network header
    fn new() -> Option<DateTime<Local>>{
        Option::from(Local::now())
    }

    // getter String format
    fn get_string_format(data_time: &DateTime<Local>) -> String {
        data_time.to_string()
    }

    // @TODO handling DateTime<Local> String
}

fn main() {
    let opt_system_time= <DateTime<Local> as CustomTime>::new();
    let system_time= match opt_system_time {
        Some(time) => time,
        None => panic!("[Err] can't get local time")
    };

    let string_time= <DateTime<Local> as CustomTime>::get_string_format(&system_time);
}
