use crate::backend;
use serde_json::json;

#[tauri::command]
pub fn login(username: &str, password: &str) -> Option<String> {
    let payload = json!({
      "username": username,
      "password": password
    });

    let response = backend::send_request("user/login", &payload);

    if response.status().is_success() {
        Some(
            response
                .text()
                .expect("Failed to get the response body as text."),
        )
    } else {
        None
    }
}

#[tauri::command]
pub fn register(username: &str, password: &str) -> Option<String> {
    let payload = json!({
      "username": username,
      "password": password
    });

    let response = backend::send_request("user/register", &payload);

    if response.status().is_success() {
        Some(
            response
                .text()
                .expect("Failed to get the response body as text."),
        )
    } else {
        None
    }
}
