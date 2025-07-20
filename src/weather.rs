use chrono::{DateTime, Datelike, Timelike};
use serde::Deserialize;
use slint::VecModel;

use crate::{Api, Coordinates, Date, Time, WeatherDaily};

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
pub struct WeatherApiDaily {
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
pub struct WeatherApiHourly {
    pub temperature_2m: Vec<f32>,
    pub precipitation: Vec<f32>,
    pub time: Vec<String>,
    pub uv_index: Vec<f32>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
struct WeatherApiData {
    daily: WeatherApiDaily,
    hourly: WeatherApiHourly,
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
    let coordinates = api.get_coordinates();
    if coordinates.latitude == 0.0 || coordinates.longitude == 0.0 {
        return;
    }

    let api_data = fetch_weather(coordinates).await;

    let offset_sec = api_data.utc_offset_seconds / 60 / 60;
    let offset_hours = format!("+{offset_sec}:00");

    let daily = api_data.daily.clone();

    let mut weather_daily: Vec<WeatherDaily> = vec![];

    api_data
        .daily
        .time
        .iter()
        .enumerate()
        .for_each(|(i, time)| {
            let api_date = DateTime::parse_from_rfc3339(&format!("{time}T00:00:00{offset_hours}"));

            let mut date = Date::default();
            if let Ok(d) = api_date {
                date.year = d.year() as i32;
                date.month = d.month() as i32;
                date.day = d.day() as i32;
            }

            let mut sunrise = Time::default();
            let api_sunrise = DateTime::parse_from_rfc3339(&format!(
                "{}:00{offset_hours}",
                api_data.daily.sunrise[i]
            ));
            if let Ok(t) = api_sunrise {
                sunrise.hour = t.hour() as i32;
                sunrise.minute = t.minute() as i32;
                sunrise.second = t.second() as i32;
            }

            let api_sunset = DateTime::parse_from_rfc3339(&format!(
                "{}:00{offset_hours}",
                api_data.daily.sunset[i]
            ));

            let mut sunset = Time::default();

            if let Ok(t) = api_sunset {
                sunset.hour = t.hour() as i32;
                sunset.minute = t.minute() as i32;
                sunset.second = t.second() as i32;
            }

            weather_daily.push(WeatherDaily {
                weather_code: daily.weather_code[i],
                temperature_max: daily.temperature_2m_max[i],
                temperature_min: daily.temperature_2m_min[i],
                precipitation_sum: daily.precipitation_sum[i],
                precipitation_probability_max: daily.precipitation_probability_max[i],
                date: date,
                sunrise: sunrise,
                sunset: sunset,
            });
        });

    api.set_weather_daily(VecModel::from_slice(&weather_daily));
}
