use std::rc::Rc;

use gloo_console::log;
use serde::Deserialize;
use yew::{platform::spawn_local, prelude::*};
use yew_hooks::use_interval;

use crate::context::location::LocationContext;

use super::{super::utils::fetch, location::Coordinates};

// Easier to deal with a single 'variable'
#[derive(Debug, PartialEq, Clone)]
pub struct WeatherCtx {
    pub is_loaded: bool,
    pub weather: WeatherData,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
pub struct WeatherData {
    pub daily: WeatherDaily,
    pub hourly: WeatherHourly,
    pub utc_offset_seconds: i32,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
pub struct WeatherDaily {
    pub temperature_2m_max: Vec<f32>,
    pub temperature_2m_min: Vec<f32>,
    pub time: Vec<String>,
    pub precipitation_sum: Vec<f32>,
    pub precipitation_probability_max: Vec<i32>,
    pub weather_code: Vec<i32>,
    pub sunrise: Vec<String>,
    pub sunset: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
pub struct WeatherHourly {
    pub temperature_2m: Vec<f32>,
    pub precipitation: Vec<f32>,
    pub time: Vec<String>,
    pub uv_index: Vec<f32>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
struct WeatherApiData {
    daily: WeatherDaily,
    hourly: WeatherHourly,
    utc_offset_seconds: i32,
}

impl Reducible for WeatherCtx {
    type Action = WeatherData;

    fn reduce(self: Rc<Self>, data: Self::Action) -> Rc<Self> {
        WeatherCtx {
            is_loaded: true,
            weather: data,
        }
        .into()
    }
}

pub type WeatherContext = UseReducerHandle<WeatherCtx>;

#[derive(Properties, Debug, PartialEq)]
pub struct WeatherProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn WeatherProvider(props: &WeatherProviderProps) -> Html {
    let weather = use_reducer(|| WeatherCtx {
        is_loaded: false,
        weather: WeatherData {
            ..Default::default()
        },
    });

    let location_ctx = use_context::<LocationContext>().unwrap();

    let weather_clone = weather.clone();
    use_effect_with(location_ctx.coordinates.clone(), move |coordinates| {
        // Wait till we get data
        if coordinates.latitude == 0.0 {
            return;
        }

        let coordinates_clone = coordinates.clone();
        spawn_local(async move {
            let data = fetch_weather(coordinates_clone).await;
            weather_clone.dispatch(WeatherData {
                daily: data.daily,
                hourly: data.hourly,
                utc_offset_seconds: data.utc_offset_seconds,
            });
        });
    });

    let update_every_millis = 1000 * 60 * 60;
    let coordinates_clone1 = location_ctx.coordinates.clone();
    let weather_clone1 = weather.clone();
    use_interval(
        move || {
            log!("In use interval");
            // Wait till we get data
            if coordinates_clone1.latitude == 0.0 {
                return;
            }

            let coordinates_clone2 = coordinates_clone1.clone();
            let weather_clone2 = weather_clone1.clone();
            spawn_local(async move {
                let data = fetch_weather(coordinates_clone2).await;
                weather_clone2.dispatch(WeatherData {
                    daily: data.daily,
                    hourly: data.hourly,
                    utc_offset_seconds: data.utc_offset_seconds,
                });
            });
        },
        update_every_millis,
    );

    html! {
        <ContextProvider<WeatherContext> context={weather}>
            {props.children.clone()}
        </ContextProvider<WeatherContext>>
    }
}

async fn fetch_weather(coordinates: Coordinates) -> WeatherApiData {
    let params = [
        ["latitude", &coordinates.latitude.to_string()],
        ["longitude", &coordinates.longitude.to_string()],
        ["timezone", &"auto".to_string()],
        [
            "hourly",
            &["temperature_2m", "precipitation", "uv_index"].join(","),
        ],
        [
            "daily",
            &[
                "weather_code",
                "sunrise",
                "sunset",
                "temperature_2m_max",
                "temperature_2m_min",
                "precipitation_sum",
                "precipitation_probability_max",
            ]
            .join(","),
        ],
    ]
    .map(|x| x.join("="))
    .join("&");

    let url = "https://api.open-meteo.com/v1/forecast?".to_string() + &params;

    let data = fetch::<WeatherApiData>(url).await;
    log!(format!("{:?}", data));

    return data;
}
