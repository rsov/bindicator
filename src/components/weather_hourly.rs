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
use yew::{function_component, html, use_effect_with, Html, Properties};
use yew_hooks::use_async;

use crate::context::weather::WeatherHourly;

#[derive(Clone, PartialEq, Properties)]
pub struct HourlyComponentProps {
    pub data: WeatherHourly,
    pub offset_hours: String,
}

#[function_component]
pub fn HourlyComponent(props: &HourlyComponentProps) -> Html {
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
                    .y_axis_index(1)
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
