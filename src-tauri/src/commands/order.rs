use serde::{Deserialize, Serialize};

use crate::backend;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderRegistration<'a> {
    pub bottles_description: &'a str,
    pub box_ids: Vec<u32>,
    pub customer_id: u32,
    pub oil_filtering: bool,
    pub oil_water_separation: bool,
    pub olive_amount: u32,
    pub received_at: u64,
    pub status: &'a str,
    pub worker_username: &'a str,
}

#[tauri::command]
pub fn order_register(data: OrderRegistration, token: &str) -> Option<String> {
    match backend::post(&format!("order?token={}", token), &data) {
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
pub fn get_orders(token: &str, status: Option<&str>, worker_name: Option<&str>) -> Option<String> {
    let mut url = format!("order?token={}", token);
    
    // Add status to the query if provided
    if let Some(status) = status {
        url.push_str(&format!("&status={}", status));
    }
    if let Some(worker_name) = worker_name {
        url.push_str(&format!("&workerName={}", worker_name))
    }

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
