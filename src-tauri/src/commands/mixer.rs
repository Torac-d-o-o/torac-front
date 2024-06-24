use serde::{Deserialize, Serialize};

use crate::backend;

use reqwest::Client;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MixerData {
    pub id: Option<u64>,
    pub entered_at: Option<u64>,
    pub exited_at: Option<u64>,
    pub approximated_pasta_weight: Option<f64>,
    pub production_id: Option<u64>,
    pub machine_id: Option<u64>,
    pub mixer_status: Option<String>,
    pub order_id: Option<u64>,
}

#[tauri::command]
pub fn get_mixers(token: &str, data: MixerData) -> Option<String> {
    let mut url = format!("mixer?token={}", token);

    if let Some(id) = data.id {
        url.push_str(&format!("&id={}", id));
    }
    if let Some(machine_id) = data.machine_id {
        url.push_str(&format!("&machineId={}", machine_id));
    }
    if let Some(production_id) = data.production_id {
        url.push_str(&format!("&productionId={}", production_id));
    }
    if let Some(mixer_status) = &data.mixer_status {
        url.push_str(&format!("&mixerStatus={}", mixer_status));
    }

    println!("URL Get Mixer data: {}", url);
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
pub fn register_mixer(token: &str, data: MixerData) -> Option<String> {
    match backend::post(&format!("mixer?token={}", token), &data) {
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
pub async fn update_mixer(machine_id: u64, status: &str, token: &str) -> Result<(), String> {
    let url = format!("http://localhost:3000/mixer/{}/status/{}?token={}", machine_id, status, token);

    // Send the PATCH request
    let client = Client::new();
    let response = match client.patch(&url).send().await {
        Ok(resp) => resp,
        Err(e) => return Err(format!("Request failed: {:?}", e)),
    };

    // Check the response status
    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("HTTP error: {}", response.status()))
    }
}
