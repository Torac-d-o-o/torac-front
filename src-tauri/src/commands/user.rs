use crate::backend;
use serde_json::json;

#[tauri::command]
pub fn user_login(username: &str, password: &str) -> Option<String> {
    let payload = json!({
      "username": username,
      "password": password
    });

    match backend::post("user/login", &payload) {
        Err(_) => None,
        Ok(response) => {
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
    }
}

#[tauri::command]
pub fn user_register(username: &str, password: &str) -> Option<String> {
    let payload = json!({
      "username": username,
      "password": password
    });

    match backend::post("user/register", &payload) {
        Err(_) => None,
        Ok(response) => {
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
    }
}
