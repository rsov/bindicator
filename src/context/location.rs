use std::rc::Rc;

use gloo_console::log;
use gloo_storage::{LocalStorage, Storage};
use serde::Deserialize;
use yew::{platform::spawn_local, prelude::*};

use super::super::utils::fetch;

// Easier to deal with a single 'variable'
#[derive(Debug, PartialEq, Clone)]
pub struct LocationCtx {
    pub coordinates: Coordinates,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Default)]
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
        log!(format!("Reducing: {:?}", data));
        LocationCtx {
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
        coordinates: Coordinates {
            latitude: 0.0,
            longitude: 0.0,
        },
    });

    let location_clone = location.clone();
    use_effect_with(location.clone(), move |_| {
        // Only get location once, not sure why I need these checks
        if location_clone.coordinates.latitude != 0.0 {
            return;
        }

        let current_coordinates_result = LocalStorage::get::<Coordinates>("coordinates");

        if current_coordinates_result.is_ok() {
            let data = current_coordinates_result.unwrap();
            location_clone.dispatch(Coordinates {
                latitude: data.latitude,
                longitude: data.longitude,
            });
        } else {
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
        }
    });

    html! {
        <ContextProvider<LocationContext> context={location}>
            {props.children.clone()}
        </ContextProvider<LocationContext>>
    }
}
