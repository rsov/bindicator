use gloo_net::http::Request;
use serde::{de::DeserializeOwned, Deserialize};
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[derive(Clone, PartialEq, Deserialize, Default)]
struct WeatherApiData {
    latitude: f32,
    daily: WeatherDaily,
}

#[derive(Clone, PartialEq, Deserialize, Default)]
struct WeatherDaily {
    sunrise: Vec<String>,
    sunset: Vec<String>,
    time: Vec<String>,
}

#[function_component]
pub fn WeatherComponent() -> Html {
    let geo_state = use_geolocation();
    let weather = use_state(|| WeatherApiData {
        ..Default::default()
    });

    let weather_clone = weather.clone();
    use_effect_update_with_deps(
        move |geo_state| {
            let params = [
                ["latitude", &geo_state.latitude.to_string()],
                ["longitude", &geo_state.longitude.to_string()],
                ["timezone", &"auto".to_string()],
                ["hourly", &["temperature_2m", "precipitation"].join(",")],
                ["daily", &["sunrise", "sunset"].join(",")],
            ]
            .map(|x| x.join("="))
            .join("&");

            let url = "https://api.open-meteo.com/v1/forecast?".to_string() + &params;

            spawn_local({
                async move {
                    let data = fetch::<WeatherApiData>(url).await;
                    weather_clone.set(data);
                }
            });

            || {}
        },
        geo_state.clone(),
    );

    html! {
        <div>
            <br/>
            <span>{"Sun rise: "}{weather.daily.sunrise.first()}</span>
            <br/>
            <span>{"Sun set: "}{weather.daily.sunset.first()}</span>
            <br/>
        </div>
    }
}

// Pretty generic, can be extracted
async fn fetch<T>(url: String) -> T
where
    T: DeserializeOwned,
{
    return Request::get(&url)
        .send()
        .await
        .unwrap()
        .json::<T>()
        .await
        .unwrap();
}
