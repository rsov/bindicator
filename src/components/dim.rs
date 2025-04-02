use chrono::{DateTime, Local, Timelike};
use yew::{function_component, html, use_state, Html};
use yew_hooks::use_interval;

const REFRESH_MILLIS: u32 = 900_000; // Every 15 minutes

pub fn should_dim() -> bool {
    let current: DateTime<Local> = Local::now();

    let daylight_hours = 7..20;

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
            REFRESH_MILLIS,
        );
    }

    // Altering global state is bad, mmkkkkay?
    html! {
        if *is_dim {
            <style>
                { "body {opacity: 0.3; background-color: black}" }
            </style>
        }
    }
}
