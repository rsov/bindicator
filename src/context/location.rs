use std::rc::Rc;

use gloo_console::log;
use serde::Deserialize;
use yew::{platform::spawn_local, prelude::*};

use super::super::utils::fetch;

// Easier to deal with a single 'variable'
#[derive(Debug, PartialEq, Clone)]
pub struct LocationCtx {
    pub is_loaded: bool,
    pub coordinates: Coordinates,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Coordinates {
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
struct GeoLocationApiData {
    latitude: f32,
    longitude: f32,
}

impl Reducible for LocationCtx {
    type Action = Coordinates;

    fn reduce(self: Rc<Self>, data: Self::Action) -> Rc<Self> {
        LocationCtx {
            is_loaded: true,
            coordinates: Coordinates {
                longitude: data.longitude,
                latitude: data.latitude,
            },
        }
        .into()
    }
}

pub type LocationContext = UseReducerHandle<LocationCtx>;

#[derive(Properties, Debug, PartialEq)]
pub struct LocationProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn LocationProvider(props: &LocationProviderProps) -> Html {
    let location = use_reducer(|| LocationCtx {
        is_loaded: false,
        coordinates: Coordinates {
            latitude: 0.0,
            longitude: 0.0,
        },
    });

    let location_clone = location.clone();
    use_effect(move || {
        // Only get location once, not sure why I need these checks
        if location_clone.is_loaded == true {
            return;
        }

        spawn_local({
            async move {
                let url = String::from("https://freeipapi.com/api/json");
                let data = fetch::<GeoLocationApiData>(url).await;

                log!(format!("{:?}", data));

                location_clone.dispatch(Coordinates {
                    latitude: data.latitude,
                    longitude: data.longitude,
                });
            }
        });
    });

    html! {
        <ContextProvider<LocationContext> context={location}>
            {props.children.clone()}
        </ContextProvider<LocationContext>>
    }
}
