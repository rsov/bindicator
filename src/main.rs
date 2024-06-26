mod components;
use components::bin::BinComponent;
use components::clock::ClockComponent;
use components::weather::WeatherComponent;

use yew::{function_component, html, Html};

#[function_component]
pub fn App() -> Html {
    html! {
        <div id="app" class="d-flex flex-column justify-content-between p-2" style="overflow: hidden;">
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
