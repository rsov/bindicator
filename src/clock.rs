use chrono::{Datelike, Local, Timelike};

use crate::{Api, Date, Time};

pub fn set_time(api: Api) {
    let now = Local::now();
    let mut date = Date::default();
    date.year = now.year() as i32;
    date.month = now.month() as i32;
    date.day = now.day() as i32;
    api.set_current_date(date);

    let mut time = Time::default();
    time.hour = now.hour() as i32;
    time.minute = now.minute() as i32;
    time.second = now.second() as i32;
    api.set_current_time(time);
}
