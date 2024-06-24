mod components;
use components::bin::BinComponent;
use components::clock::ClockComponent;
use components::weather::WeatherComponent;

use yew::{function_component, html, Html};

#[function_component]
pub fn App() -> Html {
    html! {
        <div>
            <div class="p-2 d-flex justify-content-between">
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
