use chrono::{DateTime, Local};
use yew::{function_component, html, Html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct DailyComponentProps {
    pub weather_code: i32,
    pub date: DateTime<Local>,
    pub temp_min: f32,
    pub temp_max: f32,
    pub precipitation_sum: f32,
    pub precipitation_probability_max: i32,
    pub sunrise: DateTime<Local>,
    pub sunset: DateTime<Local>,
}

#[function_component]
pub fn DailyComponent(props: &DailyComponentProps) -> Html {
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
