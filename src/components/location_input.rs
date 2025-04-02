use gloo_storage::{LocalStorage, Storage};
use web_sys::{wasm_bindgen::JsCast, EventTarget, FormData, HtmlFormElement};
use yew::{function_component, html, use_context, use_effect_with, Callback, Html, SubmitEvent};

use crate::context::location::{Coordinates, LocationContext};

#[function_component]
pub fn LocationInput() -> Html {
    // Should replace this with an address lookup API but I'm lazy A.F.

    let location_ctx = use_context::<LocationContext>().unwrap();

    let location_ctx_effect_clone = location_ctx.clone();
    use_effect_with(location_ctx.coordinates.clone(), move |_| {
        let current_coordinates_result = LocalStorage::get::<Coordinates>("coordinates");

        if current_coordinates_result.is_ok() {
            location_ctx_effect_clone.dispatch(current_coordinates_result.unwrap());
        }
    });

    let location_ctx_submit_clone = location_ctx.clone();
    let form_onsubmit = {
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let target: Option<EventTarget> = event.target();
            let form = target.and_then(|t| t.dyn_into::<HtmlFormElement>().ok());

            if let Some(form) = form {
                let form_data = FormData::new_with_form(&form).unwrap();

                // I miss JS sometimes
                let coordinates = Coordinates {
                    latitude: form_data
                        .get("lat")
                        .as_string()
                        .unwrap()
                        .parse::<f32>()
                        .unwrap(),
                    longitude: form_data
                        .get("lon")
                        .as_string()
                        .unwrap()
                        .parse::<f32>()
                        .unwrap(),
                };

                LocalStorage::set("coordinates", coordinates.clone()).unwrap();
                location_ctx_submit_clone.dispatch(coordinates.clone());
            }
        })
    };

    let location_ctx_onclick_clone = location_ctx.clone();
    let clear_onclick = {
        Callback::from(move |_| {
            LocalStorage::clear();

            location_ctx_onclick_clone.dispatch(Coordinates {
                ..Default::default()
            });
        })
    };

    let current_coordinates = location_ctx.coordinates.clone();

    html! {
        <div>
            {
                if current_coordinates.latitude != 0.0 {
                  html!{
                        <div class="d-flex gap-5">
                            <div>
                                {"Saved location"}<br/>
                                {"Latitude: "} {current_coordinates.latitude}<br/>
                                {"Longitude: "} {current_coordinates.longitude}
                            </div>

                            <button onclick={clear_onclick}>
                                    {"Clear"}
                            </button>
                        </div>
                    }
                } else {
                  html!{  <div>{"No stored data"}</div>}
                }
            }


            <form class="d-flex flex-column gap-3 mt-2" onsubmit={ form_onsubmit }>

                <div class="input-group">
                    <div class="input-group-prepend">
                        <span class="input-group-text">{"Latitude"}</span>
                    </div>
                    <input type="number" name="lat" id="lat" class="form-control" placeholder="00.00" required={true} step="any" />
                </div>

                <div class="input-group">
                    <div class="input-group-prepend">
                        <span class="input-group-text">{"Longitude"}</span>
                    </div>
                    <input type="number" name="lon" id="lon" class="form-control" placeholder="00.00" required={true} step="any" />
                </div>

                <button class="btn btn-primary">{"Save"}</button>
            </form>
        </div>
    }
}
