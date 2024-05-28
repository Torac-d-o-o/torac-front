use reqwest::blocking::Response;
use serde::Serialize;
use serde_json::json;

/// The URL of the back-end API. This has to be changed to the correct URL before publishing.
pub const BASE_URL: &str = "http://localhost:3000";

/// Sends a request to the back-end API.
pub fn send_request<T: Serialize + ?Sized>(endpoint: &str, payload: &T) -> Response {
    let payload = json!(payload);

    let client = reqwest::blocking::Client::new();

    client
        .post(format!("{}/{}", BASE_URL, endpoint))
        .json(&payload)
        .send()
        .expect("Failed to send request.")
}
