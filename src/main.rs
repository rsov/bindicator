use chrono::{Datelike, Local, TimeZone, Timelike};
use slint::{Timer, TimerMode};
use std::sync::Arc;
use wasm_bindgen::prelude::wasm_bindgen;

slint::include_modules!();

#[wasm_bindgen(main)]
pub fn main() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    let app = App::new().unwrap();
    let app_weak = app.as_weak();

    let app_arc = Arc::new(app_weak);

    let app_clock = Arc::clone(&app_arc);
    let timer = Timer::default();
    timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_millis(1000),
        move || {
            if let Some(app) = app_clock.upgrade() {
                let api = app.global::<Api>();
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
        },
    );

    let app_bin = Arc::clone(&app_arc);
    Timer::default().start(
        TimerMode::Repeated,
        std::time::Duration::from_secs(60 * 60),
        move || {
            if let Some(app) = app_bin.upgrade() {
                set_bins(app.global::<Api>());
            }
        },
    );

    // Set now instead of waiting for the timers to kick in
    let app_now = Arc::clone(&app_arc);
    if let Some(app) = app_now.upgrade() {
        set_bins(app.global::<Api>());
    }

    app.run().expect("AppWindow::run() failed");
}

pub fn set_bins(api: Api) {
    api.set_is_yellow_bin(is_yellow_bin());
    api.set_days_to_bin(get_days_to_bin());
}

// Yellow alternate every week
pub fn is_yellow_bin() -> bool {
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

pub fn get_days_to_bin() -> i32 {
    let known_yellow_bin_day = Local.with_ymd_and_hms(2024, 5, 13, 0, 0, 0).unwrap();
    let diff = Local::now() - known_yellow_bin_day;

    return (diff.num_days() % 7) as i32;
}
