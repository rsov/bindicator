use chrono::{DateTime, Local, Timelike};
use yew::{function_component, html, use_state, Html};
use yew_hooks::use_interval;

const REFRESH_HOURS: u32 = 1;

pub fn should_dim() -> bool {
    let current: DateTime<Local> = Local::now();

    let daylight_hours = 6..20;

    return !daylight_hours.contains(&current.hour());
}

#[function_component]
pub fn DimComponent() -> Html {
    let is_dim = use_state(|| should_dim());

    {
        let state = is_dim.clone();
        use_interval(
            move || {
                state.set(should_dim());
            },
            REFRESH_HOURS * 3_600_000,
        );
    }

    // Altering global state is bad, mmkkkkay?
    html! {
        if *is_dim {
            <style>
                { "body {opacity: 0.5; background-color: black}" }
            </style>
        }
    }
}
