use gloo_net::http::Request;
use serde::de::DeserializeOwned;

// Pretty generic, can be extracted
pub async fn fetch<T>(url: String) -> T
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
