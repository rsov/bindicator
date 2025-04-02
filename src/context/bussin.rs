use std::rc::Rc;

use gloo_console::log;
use serde::Deserialize;
use yew::{platform::spawn_local, prelude::*};
use yew_hooks::use_interval;

use crate::context::location::LocationContext;

#[derive(Debug, PartialEq, Clone)]
pub struct BusCtx {
    pub is_loaded: bool,
    pub data: BusData,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
pub struct BusData {
    pub utc_offset_seconds: i32,
}

impl Reducible for BusCtx {
    type Action = BusData;

    fn reduce(self: Rc<Self>, data: Self::Action) -> Rc<Self> {
        BusCtx {
            is_loaded: true,
            data: data,
        }
        .into()
    }
}

pub type BusContext = UseReducerHandle<BusCtx>;

#[derive(Properties, Debug, PartialEq)]
pub struct BusProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn BusProvider(props: &BusProviderProps) -> Html {
    let data = use_reducer(|| BusCtx {
        is_loaded: false,
        data: BusData {
            ..Default::default()
        },
    });

    let location_ctx = use_context::<LocationContext>().unwrap();

    let data_clone = data.clone();
    use_effect_with(location_ctx.coordinates.clone(), move |coordinates| {
        // Wait till we get data
        if coordinates.latitude == 0.0 {
            return;
        }

        let coordinates_clone = coordinates.clone();
        spawn_local(async move {
            // let data = fetch_weather(coordinates_clone).await;
            // data_clone.dispatch(BusData {
            //     utc_offset_seconds: data.utc_offset_seconds,
            // });
        });
    });

    let update_every_millis = 1000 * 60 * 60;
    let coordinates_clone1 = location_ctx.coordinates.clone();
    let data_clone1 = data.clone();
    use_interval(
        move || {
            log!("In use interval");
            // Wait till we get data
            if coordinates_clone1.latitude == 0.0 {
                return;
            }

            let coordinates_clone2 = coordinates_clone1.clone();
            let weather_clone2 = data_clone1.clone();
            spawn_local(async move {
                // let data = fetch_weather(coordinates_clone2).await;
                // weather_clone2.dispatch(BusData {
                //     utc_offset_seconds: data.utc_offset_seconds,
                // });
            });
        },
        update_every_millis,
    );

    html! {
        <ContextProvider<BusContext> context={data}>
            {props.children.clone()}
        </ContextProvider<BusContext>>
    }
}
