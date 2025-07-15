use slint::{Timer, TimerMode};
use std::sync::Arc;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{bins::set_bins, clock::set_time, location::set_location};

mod bins;
mod clock;
mod location;
mod weather;

slint::include_modules!();

#[wasm_bindgen(main)]
pub async fn main() {
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
                set_time(app.global::<Api>());
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
        set_location(app.global::<Api>()).await;
    }

    app.run().expect("AppWindow::run() failed");
}
