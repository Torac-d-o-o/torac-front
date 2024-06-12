use serde::{Deserialize, Serialize};

use crate::backend;


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductionRegistration<'a> {
    pub entered_at: Option<u64>,
    pub exited_at: Option<u64>,
    pub status:Option<&'a str>,
    pub control_panel_user:Option<&'a str>,
    pub olive_washing_user:Option<&'a str>,
    pub oil_water_separation_user:Option<&'a str>,
    pub order: u64
}


#[tauri::command]
pub fn get_production(token: &str, status: Option<&str>, order: Option<u64>) -> Option<String> {
    let mut url = format!("production?token={}", token);

    if let Some(status) = status {
        url.push_str(&format!("&status={}", status));
    }
    if let Some(order) = order {
        url.push_str(&format!("&order={}", order))
    }
    
    println!("URL: {}", url);

    match backend::get(&url) {
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
pub fn register_production(data: ProductionRegistration, token: &str) -> Option<String> {
    match backend::post(&format!("production?token={}", token), &data) {
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