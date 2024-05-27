mod components;
use components::bin::Bin;

use yew::{function_component, html, Html};

#[function_component]
pub fn App() -> Html {
    html! {
        <div style="padding: 8px;">
            <Bin/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
