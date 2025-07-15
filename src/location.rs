use serde::{Deserialize, Serialize};
use web_sys::Window;

use crate::{Api, Coordinates};

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
struct GeoLocationApiData {
    latitude: f32,
    longitude: f32,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default, Serialize)]
struct LocationStorage {
    latitude: f32,
    longitude: f32,
}

const STORAGE_KEY: &str = "coordinates";

async fn fetch_location() -> GeoLocationApiData {
    let url = "https://ipwho.is/";

    let data = reqwest::get(url)
        .await
        .unwrap()
        .json::<GeoLocationApiData>()
        .await
        .unwrap();

    return data;
}

fn fetch_from_local(key: &str) -> Option<Coordinates> {
    let window: Window = web_sys::window().unwrap();
    let maybe_storage = window.local_storage().unwrap();
    if let Some(storage) = maybe_storage {
        let maybe_data = storage.get_item(key).unwrap();
        if let Some(data) = maybe_data {
            let coordinates_result: Result<LocationStorage, serde_json::Error> =
                serde_json::from_str(&data);

            if coordinates_result.is_ok() {
                let location_storage = coordinates_result.unwrap();
                return Some(Coordinates {
                    latitude: location_storage.latitude,
                    longitude: location_storage.longitude,
                });
            }
        }
    }
    None
}

fn store_into_local(key: &str, coordinates: Coordinates) {
    let window: Window = web_sys::window().unwrap();
    let maybe_storage = window.local_storage().unwrap();
    if let Some(storage) = maybe_storage {
        let location_storage = LocationStorage {
            latitude: coordinates.latitude,
            longitude: coordinates.longitude,
        };

        let data_string = serde_json::to_string(&location_storage).unwrap();
        storage.set_item(key, &data_string).unwrap();
    }
}

// TODO: Read from storage later on
pub async fn set_location(api: Api<'_>) {
    let from_location = fetch_from_local(STORAGE_KEY);

    if let Some(local) = from_location {
        api.set_coordinates(local);
        return;
    }

    let data = fetch_location().await;
    let coordinates = Coordinates {
        latitude: data.latitude,
        longitude: data.longitude,
    };

    store_into_local(STORAGE_KEY, coordinates.clone());

    api.set_coordinates(coordinates);
}
