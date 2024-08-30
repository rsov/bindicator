use charming::{
    component::{Axis, Grid, Legend},
    element::{
        AxisLabel, AxisTick, AxisType, ItemStyle, LineStyle, MarkArea, MarkAreaData, SplitLine,
        TextStyle,
    },
    series::Line,
    Chart, WasmRenderer,
};
use chrono::{DateTime, Local};
use gloo_console::log;
use gloo_net::http::Request;
use serde::{de::DeserializeOwned, Deserialize};
use yew::{platform::spawn_local, prelude::*};
use yew_hooks::prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
struct GeoLocationApiData {
    latitude: f32,
    longitude: f32,
}

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
    precipitation_probability_max: Vec<i32>,
    weather_code: Vec<i32>,
    sunrise: Vec<String>,
    sunset: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
struct WeatherHourly {
    temperature_2m: Vec<f32>,
    precipitation: Vec<f32>,
    time: Vec<String>,
    uv_index: Vec<f32>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
struct GeoState {
    loading: bool,
    latitude: f32,
    longitude: f32,
}

#[function_component]
pub fn WeatherComponent() -> Html {
    let update_every_millis = 1000 * 60 * 60;
    let trigger = use_force_update();

    let weather = use_state(|| WeatherApiData {
        ..Default::default()
    });

    let weather_clone1 = weather.clone();
    use_interval(
        move || {
            weather_clone1.set((|| WeatherApiData {
                ..Default::default()
            })());

            trigger.force_update()
        },
        update_every_millis,
    );

    let weather_clone = weather.clone();

    use_effect(move || {
        if weather_clone.latitude != 0.0 {
            return;
        }

        spawn_local({
            async move {
                let url = String::from("https://freeipapi.com/api/json");
                let data = fetch::<GeoLocationApiData>(url).await;

                log!(format!("{:?}", data));

                let params = [
                    ["latitude", &data.latitude.to_string()],
                    ["longitude", &data.longitude.to_string()],
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

                spawn_local({
                    async move {
                        let data = fetch::<WeatherApiData>(url).await;
                        log!(format!("{:?}", data));
                        weather_clone.set(data);
                    }
                });
            }
        });
    });

    let offset_sec = weather.utc_offset_seconds / 60 / 60;
    let offset_hours = format!("+{offset_sec}:00");

    html! {
        <>
            <HourlyComponent data={weather.hourly.clone()} offset_hours={offset_hours.clone()} />

            <div class="card-group text-white">
            {
                weather.daily.time.clone().iter().enumerate().map(|(i, time)| {
                    let temp_max = weather.daily.temperature_2m_max.clone()[i];
                    let temp_min = weather.daily.temperature_2m_min.clone()[i];
                    let precipitation = weather.daily.precipitation_sum.clone()[i];
                    let precipitation_probability_max = weather.daily.precipitation_probability_max.clone()[i];
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
                            precipitation_probability_max: precipitation_probability_max.to_owned(),
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
    data: WeatherHourly,
    offset_hours: String,
}

#[function_component]
fn HourlyComponent(props: &HourlyComponentProps) -> Html {
    let current_time = Local::now();

    let mut time = Vec::new();
    let mut temp = Vec::new();
    let mut rain = Vec::new();
    let mut uv: Vec<f32> = Vec::new();

    let offset_hours = props.offset_hours.clone();

    for (i, time_stamp) in props.data.time.clone().iter().enumerate() {
        if time.len() > 48 {
            break;
        }

        // let precipitation = weather.hourly.precipitation.clone()[i];

        let date = DateTime::parse_from_rfc3339(&format!("{time_stamp}:00{offset_hours}"));

        if date.is_ok() && date.unwrap() >= current_time {
            time.push(format!("{}", date.unwrap().format("%H:%M")));
            temp.push(props.data.temperature_2m[i]);
            rain.push(props.data.precipitation[i]);
            uv.push(props.data.uv_index[i]);
        } else {
        }
    }

    let f = use_async::<_, _, ()>({
        let chart = Chart::new()
            .legend(
                Legend::new()
                    .data(vec!["Temperature", "Precipitation", "UV"])
                    .text_style(TextStyle::new().color("white")),
            )
            .x_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .data(time.clone())
                    .axis_tick(AxisTick::new().show(false))
                    .axis_label(AxisLabel::new().color("white")),
            )
            .y_axis(
                // Temp Lines
                Axis::new()
                    .type_(AxisType::Value)
                    .axis_label(AxisLabel::new().color("white"))
                    // Doesn't work https://github.com/yuankunzhang/charming/pull/67
                    // .axis_label(AxisLabel::new().formatter("{value} °C"))
                    .split_line(SplitLine::new().line_style(LineStyle::new().color("grey"))),
            )
            .y_axis(
                // Lines
                Axis::new()
                    .type_(AxisType::Value)
                    .axis_label(AxisLabel::new().color("orange"))
                    .split_line(SplitLine::new().line_style(LineStyle::new().opacity(0)))
                    .max(11),
            )
            .series(
                Line::new()
                    .name("Temperature")
                    .data(temp.clone())
                    .show_symbol(false)
                    .item_style(ItemStyle::new().color("white"))
                    .line_style(LineStyle::new().width(5).color("white"))
                    .mark_area(
                        MarkArea::new()
                            .item_style(ItemStyle::new().color("grey"))
                            .data(vec![(
                                MarkAreaData::new().x_axis("23:00"),
                                MarkAreaData::new().x_axis("01:00"),
                            )]),
                    ),
            )
            .series(
                Line::new()
                    .name("Precipitation")
                    .data(rain.clone())
                    .show_symbol(false)
                    .item_style(ItemStyle::new().color("blue"))
                    .line_style(LineStyle::new().width(3).color("blue")),
            )
            .series(
                Line::new()
                    .name("UV")
                    .data(uv.clone())
                    .y_axis_index(1)
                    .show_symbol(false)
                    .item_style(ItemStyle::new().color("orange"))
                    .line_style(LineStyle::new().width(3).color("orange")),
            )
            .grid(Grid::new().top(24).left(24).right(24).bottom(20));

        let renderer = WasmRenderer::new(780, 170);

        async move {
            renderer.render("chart", &chart).unwrap();
            Ok(())
        }
    });

    use_effect_with(time.clone(), move |_| {
        f.run();
        || ()
    });

    html! {
        <div id="chart"></div>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct DailyComponentProps {
    weather_code: i32,
    date: DateTime<Local>,
    temp_min: f32,
    temp_max: f32,
    precipitation_sum: f32,
    precipitation_probability_max: i32,
    sunrise: DateTime<Local>,
    sunset: DateTime<Local>,
}

#[function_component]
fn DailyComponent(props: &DailyComponentProps) -> Html {
    html! {
    <div class="card">
        <div class="card-header text-center p-0 text-white">
            { format!("{}", props.date.format("%a")) }
        </div>
        <div class="card-body d-flex flex-column align-items-center gap-1 p-0">
            <CodeIconComponent code={props.weather_code} />
            <div class="text-nowrap text-white fw-bold fs-5">
                {format!("{:.0} - {:.0}  ºC", props.temp_max, props.temp_min)}
            </div>
            <div class="text-nowrap text-white fw-bold">
                { format!("{} - {}", props.sunrise.format("%H:%M"), props.sunset.format("%H:%M")) }
            </div>
            if props.precipitation_sum > 0.0 {
                <div class="text-white fw-bold">
                    {format!("{}mm {}%", props.precipitation_sum, props.precipitation_probability_max)}
                </div>
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
    let class = match props.code {
        0 | 1 => "wi-day-sunny",
        2 | 3 => "wi-day-cloudy",
        45 | 48 => "wi-fog",
        51 | 53 | 55 => "wi-sprinkle",
        56 | 57 => "wi-snow",
        61 | 63 | 65 => "wi-raindrop",
        66 | 67 => "wi-rain-mix",
        71 | 73 | 75 => "wi-snowflake-cold",
        77 => "wi-snow-wind",
        80 | 81 | 82 => "wi-rain",
        85 | 86 => "wi-day-snow-thunderstorm",
        96 | 99 => "wi-day-thunderstorm",
        95 => "wi-day-thunderstorm",
        _ => "wi-meteor", // he-he-he-he
    };

    let icon_class = format!("wi {} text-white", class);
    html! {
        <div class="display-3">
            <i class={icon_class}></i>
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
