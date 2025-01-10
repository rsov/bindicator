mod components;
use components::carousel::Carousel;
use components::clock::ClockComponent;
use components::dim::DimComponent;
use components::location_input::LocationInput;
use components::weather::WeatherComponent;
use components::{bin::BinComponent, carousel::CarouselItem};

mod context;
use context::{location::LocationProvider, weather::WeatherProvider};

mod utils;

use yew::{function_component, html, Html};

#[function_component]
pub fn App() -> Html {
    html! {
        <div id="app" class="d-flex flex-column justify-content-between p-2" style="overflow: hidden;">
            <DimComponent/>
            <div class="d-flex justify-content-between">
                <BinComponent/>
                <ClockComponent/>
            </div>
            <LocationProvider>

                <Carousel>

                    <CarouselItem active={true}>
                        <WeatherProvider>
                            <WeatherComponent/>
                        </WeatherProvider>
                    </CarouselItem>

                    <CarouselItem active={false}>
                        <LocationInput />
                    </CarouselItem>

                </Carousel>

            </LocationProvider>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
