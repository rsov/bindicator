use serde::Deserialize;

use crate::{Api, Coordinates};

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

    let data = reqwest::get(url)
        .await
        .unwrap()
        .json::<WeatherApiData>()
        .await
        .unwrap();

    return data;
}

pub async fn set_weather(api: Api<'_>) {
    // TODO: Move the wether types into slint UI
}
