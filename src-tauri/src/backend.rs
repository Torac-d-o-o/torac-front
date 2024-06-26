use reqwest::blocking::Response;
use serde::Serialize;
use serde_json::json;

/// The URL of the back-end API. This has to be changed to the correct URL before publishing.
pub const BASE_URL: &str = "http://167.86.127.215:3000";

/// Sends a POST request to the back-end API.
pub fn post<T: Serialize + ?Sized>(
    endpoint: &str,
    payload: &T,
) -> Result<Response, reqwest::Error> {
    let payload = json!(payload);

    let client = reqwest::blocking::Client::new();

    client
        .post(format!("{}/{}", BASE_URL, endpoint))
        .json(&payload)
        .send()
}

/// Sends a GET request to the back-end API.
pub fn get(endpoint: &str) -> Result<Response, reqwest::Error> {
    let client = reqwest::blocking::Client::new();

    client.get(format!("{}/{}", BASE_URL, endpoint)).send()
}
