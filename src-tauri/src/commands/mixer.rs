use serde::{Deserialize, Serialize};

use crate::backend;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MixerRegistration {
    pub id: Option<u64>,
    pub entered_at: Option<u64>,
    pub exited_at: Option<u64>,
    pub approximated_pasta_weight: Option<f64>,
    pub production_id: Option<u64>,
    pub machine_id: Option<u64>,
    pub status: Option<String>,
    pub order_id: Option<u64>,
}

#[tauri::command]
pub fn get_mixers(token: &str, data: MixerRegistration) -> Option<String> {
    let mut url = format!("mixer?token={}", token);

    if let Some(id) = data.id {
        url.push_str(&format!("&id={}", id));
    }
    if let Some(machine_id) = data.machine_id {
        url.push_str(&format!("&machine_id={}", machine_id));
    }
    if let Some(production_id) = data.production_id {
        url.push_str(&format!("&production_id={}", production_id));
    }
    if let Some(status) = &data.status {
        url.push_str(&format!("&status={}", status));
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


#[tauri::command]
pub fn register_mixer(token: &str, data: MixerRegistration) -> Option<String> {
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

/*#[tauri::command]
pub fn update_mixer(token: &str, data: MixerRegistration) ->Option<String> {

}*/
