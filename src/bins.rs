use chrono::{Local, TimeZone};

use crate::Api;

// Yellow alternate every week
fn is_yellow_bin() -> bool {
    let known_yellow_bin_day = Local.with_ymd_and_hms(2024, 5, 13, 0, 0, 0).unwrap();
    let diff = Local::now() - known_yellow_bin_day;

    // I threw 💩 until it sorta worked
    // Good luck

    let wat = diff.num_days() % 14;

    if wat != 0 && wat <= 7 {
        return false;
    }
    return true;
}

fn get_days_to_bin() -> i32 {
    let known_yellow_bin_day = Local.with_ymd_and_hms(2024, 5, 13, 0, 0, 0).unwrap();
    let diff = Local::now() - known_yellow_bin_day;

    return (7 - diff.num_days() % 7) as i32;
}

pub fn set_bins(api: Api) {
    api.set_is_yellow_bin(is_yellow_bin());
    api.set_days_to_bin(get_days_to_bin());
}
