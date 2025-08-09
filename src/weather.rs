use chrono::{DateTime, Datelike, Local, Timelike};
use serde::Deserialize;
use slint::VecModel;

use crate::{
    Api, Coordinates, Date, Time, WeatherCurrent, WeatherDaily, WeatherHourly, WeatherType,
};

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
    pub precipitation_probability: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
pub struct WeatherApiCurrent {
    pub temperature_2m: f32,
    pub precipitation: f32,
    pub weather_code: i32,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
struct WeatherApiData {
    current: WeatherApiCurrent,
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
            "current",
            &["temperature_2m", "weather_code", "precipitation"].join(","),
        ],
        [
            "hourly",
            &[
                "temperature_2m",
                "precipitation",
                "uv_index",
                "precipitation_probability",
            ]
            .join(","),
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

fn code_to_type(code: i32) -> WeatherType {
    return match code {
        0 | 1 => WeatherType::Sunny,
        2 | 3 => WeatherType::Cloudy,
        45 | 48 => WeatherType::Fog,
        51 | 53 | 55 => WeatherType::Sprinkle,
        56 | 57 => WeatherType::Snow,
        61 | 63 | 65 => WeatherType::RainDrop,
        66 | 67 => WeatherType::RainMix,
        71 | 73 | 75 => WeatherType::SnowflakeCold,
        77 => WeatherType::SnowWind,
        80 | 81 | 82 => WeatherType::Rain,
        85 | 86 => WeatherType::SnowThunderstorm,
        95 | 96 | 99 => WeatherType::Thunderstorm,
        _ => WeatherType::Other,
    };
}

pub async fn set_weather(api: Api<'_>) {
    let coordinates = api.get_coordinates();
    if coordinates.latitude == 0.0 || coordinates.longitude == 0.0 {
        return;
    }

    let api_data = fetch_weather(coordinates).await;

    let offset_sec = api_data.utc_offset_seconds / 60 / 60;
    let offset_hours = format!("+{offset_sec}:00");

    api.set_weather_current(WeatherCurrent {
        temperature: api_data.current.temperature_2m as i32,
        precipitation: api_data.current.precipitation as i32,
        weather_type: code_to_type(api_data.current.weather_code),
    });

    let daily = api_data.daily.clone();
    let hourly = api_data.hourly.clone();

    let mut weather_daily: Vec<WeatherDaily> = vec![];

    daily.time.iter().enumerate().for_each(|(i, time)| {
        let api_date = DateTime::parse_from_rfc3339(&format!("{time}T00:00:00{offset_hours}"));

        let mut date = Date::default();
        let mut week_day: i32 = 0;
        if let Ok(d) = api_date {
            date.year = d.year() as i32;
            date.month = d.month() as i32;
            date.day = d.day() as i32;
            week_day = d.weekday().num_days_from_monday() as i32;
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

        let api_sunset =
            DateTime::parse_from_rfc3339(&format!("{}:00{offset_hours}", api_data.daily.sunset[i]));

        let mut sunset = Time::default();

        if let Ok(t) = api_sunset {
            sunset.hour = t.hour() as i32;
            sunset.minute = t.minute() as i32;
            sunset.second = t.second() as i32;
        }

        weather_daily.push(WeatherDaily {
            weather_type: code_to_type(daily.weather_code[i]),
            week_day: week_day,
            temperature_max: daily.temperature_2m_max[i] as i32,
            temperature_min: daily.temperature_2m_min[i] as i32,
            precipitation_sum: daily.precipitation_sum[i] as i32,
            precipitation_probability_max: daily.precipitation_probability_max[i],
            date: date,
            sunrise: sunrise,
            sunset: sunset,
        });
    });

    api.set_weather_daily(VecModel::from_slice(&weather_daily));

    let mut weather_hourly: Vec<WeatherHourly> = vec![];
    hourly.time.iter().enumerate().for_each(|(i, time)| {
        // Time example: "2025-08-09T00:00"
        let api_date = DateTime::parse_from_rfc3339(&format!("{time}:00{offset_hours}"));
        let current_time = Local::now();

        if api_date.is_ok() && api_date.unwrap() >= current_time {
            let mut date = Date::default();
            let mut time = Time::default();
            if let Ok(d) = api_date {
                date.year = d.year() as i32;
                date.month = d.month() as i32;
                date.day = d.day() as i32;
                time.hour = d.hour() as i32;
                time.minute = d.minute() as i32;
            }

            weather_hourly.push(WeatherHourly {
                temperature: hourly.temperature_2m[i] as i32,
                precipitation_mm: hourly.precipitation[i],
                date: date,
                time: time,
            });
        }
    });
    api.set_weather_hourly(VecModel::from_slice(&weather_hourly));

    let now = Local::now();
    let mut time = Time::default();
    time.hour = now.hour() as i32;
    time.minute = now.minute() as i32;
    time.second = now.second() as i32;
    api.set_weather_updated_at(time);
}
