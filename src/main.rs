mod components;
use components::bin::BinComponent;
use components::clock::ClockComponent;

use yew::{function_component, html, Html};

#[function_component]
pub fn App() -> Html {
    html! {
        <div style="padding: 8px;display: flex;justify-content:space-between">
            <BinComponent/>
            <ClockComponent/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
