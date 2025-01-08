use chrono::Local;
use yew::{function_component, html, use_state, Html};
use yew_hooks::use_interval;

#[function_component]
pub fn ClockComponent() -> Html {
    let current_time = use_state(|| Local::now());

    {
        let state = current_time.clone();
        use_interval(
            move || {
                state.set(Local::now());
            },
            500,
        );
    }

    html! {
        <div class="fs-1 text-end fw-bold text-white">
            { format!("{}", current_time.format("%d %b %Y")) }
            <br/>
            { format!("{}", current_time.format("%H : %M : %S")) }
        </div>
    }
}
