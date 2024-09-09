mod components;
use chrono::{DateTime, Local, NaiveTime};
use components::bin::BinComponent;
use components::clock::ClockComponent;
use components::dim::DimComponent;
use components::weather::WeatherComponent;
use web_sys::window;

use yew::{function_component, html, Html};
use yew_hooks::use_timeout;

#[function_component]
pub fn App() -> Html {
    let current: DateTime<Local> = Local::now();
    let date = current.date_naive();
    let midnight = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let midnight = date.and_time(midnight).and_local_timezone(Local).unwrap();

    // Reload the whole page at midnight so it can apply new code from github (if any)
    use_timeout(
        || {
            if let Some(window) = window() {
                window.location().reload().unwrap();
            }
        },
        midnight.signed_duration_since(current).num_milliseconds() as _,
    );

    html! {
            <div id="app" class="d-flex flex-column justify-content-between p-2" style="overflow: hidden;">
                <DimComponent/>
                <div class="d-flex justify-content-between">
                    <BinComponent/>
                    <ClockComponent/>
                </div>
                <WeatherComponent/>
            </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
