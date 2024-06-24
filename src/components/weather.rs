use chrono::{DateTime, Local};
use gloo_console::log;
use gloo_net::http::Request;
use serde::{de::DeserializeOwned, Deserialize};
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
struct WeatherApiData {
    latitude: f32,
    daily: WeatherDaily,
    hourly: WeatherHourly,
    utc_offset_seconds: i32,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
struct WeatherDaily {
    temperature_2m_max: Vec<f32>,
    temperature_2m_min: Vec<f32>,
    time: Vec<String>,
    precipitation_sum: Vec<f32>,
    weather_code: Vec<i32>,
    sunrise: Vec<String>,
    sunset: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
struct WeatherHourly {
    temperature_2m: Vec<f32>,
    precipitation: Vec<f32>,
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
                [
                    "daily",
                    &[
                        "weather_code",
                        "sunrise",
                        "sunset",
                        "temperature_2m_max",
                        "temperature_2m_min",
                        "precipitation_sum",
                    ]
                    .join(","),
                ],
            ]
            .map(|x| x.join("="))
            .join("&");

            let url = "https://api.open-meteo.com/v1/forecast?".to_string() + &params;

            spawn_local({
                async move {
                    let data = fetch::<WeatherApiData>(url).await;
                    log!(format!("{:?}", data));
                    weather_clone.set(data);
                }
            });

            || {}
        },
        geo_state.clone(),
    );

    let offset_sec = weather.utc_offset_seconds / 60 / 60;
    let offset_hours = format!("+{offset_sec}:00");

    let current_time = Local::now();

    html! {
        <>
            <div style="display: flex; gap: 8px; overflow: scroll">
            {
                weather.hourly.time.clone().iter().enumerate().map(|(i, time)| {
                    let temp = weather.hourly.temperature_2m.clone()[i];
                    let precipitation = weather.hourly.precipitation.clone()[i];

                    let date = DateTime::parse_from_rfc3339(&format!("{time}:00{offset_hours}"));

                    if date.is_ok() && date.unwrap() >= current_time {
                        let props = HourlyComponentProps {
                            date: date.unwrap().to_owned().into(),
                            temp: temp.to_owned(),
                            precipitation: precipitation.to_owned(),
                        };
                        html!{
                            <HourlyComponent ..props.clone() />
                        }
                    } else {
                        html!{}
                    }

                }).collect::<Html>()
            }
            </div>

            <div class="card-group p-2">
            {
                weather.daily.time.clone().iter().enumerate().map(|(i, time)| {
                    let temp_max = weather.daily.temperature_2m_max.clone()[i];
                    let temp_min = weather.daily.temperature_2m_min.clone()[i];
                    let precipitation = weather.daily.precipitation_sum.clone()[i];
                    let code = weather.daily.weather_code.clone()[i];

                    let date = DateTime::parse_from_rfc3339(&format!("{time}T00:00:00{offset_hours}"));
                    let sunrise = DateTime::parse_from_rfc3339(&format!("{}:00{offset_hours}", weather.daily.sunrise.clone()[i]));
                    let sunset = DateTime::parse_from_rfc3339(&format!("{}:00{offset_hours}", weather.daily.sunset.clone()[i]));

                    if date.is_ok() {
                        let props = DailyComponentProps {
                            weather_code: code.to_owned(),
                            temp_max: temp_max.to_owned(),
                            temp_min: temp_min.to_owned(),
                            precipitation_sum: precipitation.to_owned(),
                            date: date.unwrap().to_owned().into(),
                            sunrise: sunrise.unwrap().to_owned().into(),
                            sunset: sunset.unwrap().to_owned().into(),
                        };
                        html!{
                            <DailyComponent ..props.clone() />
                        }
                    } else {
                        html!{}
                    }

                }).collect::<Html>()
            }
            </div>
        </>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct HourlyComponentProps {
    date: DateTime<Local>,
    temp: f32,
    precipitation: f32,
}

#[function_component]
fn HourlyComponent(props: &HourlyComponentProps) -> Html {
    html! {
    <div>
        {format!("{:.0} °C", props.temp)}
        <br/>
        { format!("{}", props.date.format("%H:%M")) }<br/>
        if props.precipitation > 0.0 {
            {props.precipitation}{" mm"}<br/>
        }
    </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct DailyComponentProps {
    weather_code: i32,
    date: DateTime<Local>,
    temp_min: f32,
    temp_max: f32,
    precipitation_sum: f32,
    sunrise: DateTime<Local>,
    sunset: DateTime<Local>,
}

#[function_component]
fn DailyComponent(props: &DailyComponentProps) -> Html {
    html! {
    <div class="card">
        <div class="card-header text-center">
            { format!("{}", props.date.format("%a")) }
        </div>
        <div class="card-body p-1 d-flex flex-column align-items-center gap-2">
            <CodeIconComponent code={props.weather_code} />
            <div class="text-nowrap">
                {format!("{:.0}", props.temp_min)}
                {" - "}
                {format!("{:.0}", props.temp_max)}
                {" ºC"}
            </div>
            <div class="text-nowrap">
                { format!("{}", props.sunrise.format("%H:%M")) }
                {" - "}
                { format!("{}", props.sunset.format("%H:%M")) }
            </div>
            if props.precipitation_sum > 0.0 {
                <div>{props.precipitation_sum}{" mm"}</div>
            }
        </div>
    </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct CodeIconProps {
    code: i32,
}

#[function_component]
fn CodeIconComponent(props: &CodeIconProps) -> Html {
    let class = format!("wi wi-wmo4680-{}", props.code);
    html! {
        <div class="display-2">
            <i class={class}></i>
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
