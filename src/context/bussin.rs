use chrono::{DateTime, FixedOffset};
use gloo_console::log;
use serde_json::Value;
use std::rc::Rc;
use yew::{platform::spawn_local, prelude::*};

use crate::utils::fetch;

#[derive(Debug, PartialEq, Clone)]
pub struct BusCtx {
    pub is_loaded: bool,
    pub data: BusData,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct BusStopsStorage {
    pub bus_stops: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BusData {
    pub departures: Vec<Departure>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Departure {
    pub number: String,
    pub stop_name: String,
    pub departure_time: DateTime<FixedOffset>,
    pub is_cancelled: bool,
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

    // let bus_ctx = use_context::<BusContext>().unwrap();

    let update_every_millis = 1000 * 60 * 60;
    use_effect(move || {
        // let stops_to_load = LocalStorage::get::<BusStopsStorage>("bus_stops");

        // if stops_to_load.is_err() {
        //     log!("Could not load stops from storage");
        //     return;
        // }

        // let stops = stops_to_load.unwrap();
        // if stops.bus_stops.len() == 0 {
        //     return;
        // }

        // if stops.bus_stops.len() > 5 {
        //     log!("Probably loading too much stops for now??");
        //     return;
        // }

        spawn_local(async move {
            // let data = fetch_departures("G123123".to_string()).await;
            // data_clone.dispatcher(BusCtx {
            //     is_loaded: true,
            //     data: BusData { departures: data }
            // })
        });
    });

    html! {
        <ContextProvider<BusContext> context={data}>
            {props.children.clone()}
        </ContextProvider<BusContext>>
    }
}

// https://transportnsw.info/api/trip/v1/departure-list-request?name=G12312312&type=stop&depArrMacro=dep&depType=stopEvents&excludedModes=2,9,11,1,4,7

async fn fetch_departures(stop_number: String) -> Vec<Departure> {
    let params = [
        ["name", &stop_number.to_string()],
        ["depArrMacro", &"dep".to_string()],
        ["type", &"stop".to_string()],
        ["depType", &"stopEvents".to_string()],
        ["excludedModes", &"2,9,11,1,4,7".to_string()], // Only show busses for now
    ]
    .map(|x| x.join("="))
    .join("&");

    let url = "https://transportnsw.info/api/trip/v1/departure-list-request?".to_string() + &params;

    let response = fetch::<String>(url).await;
    log!(format!("{:?}", response));

    let data: Value = serde_json::from_str(&response).unwrap();

    let stop_events = data["stopEvents"].as_array().unwrap();
    let mut departures = Vec::new();

    for stop in stop_events {
        departures.push(Departure {
            departure_time: DateTime::parse_from_rfc3339(stop["departureTime"].as_str().unwrap())
                .unwrap(),
            number: stop["transportation"]["number"]
                .as_str()
                .unwrap()
                .to_string(),
            stop_name: stop["location"]["disassembledName"]
                .as_str()
                .unwrap()
                .to_string(),
            is_cancelled: stop["isCancelled"].as_bool().unwrap(),
        });
    }

    return departures;
}
