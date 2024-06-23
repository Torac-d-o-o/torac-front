use serde::{Deserialize, Serialize};

use crate::backend;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecanterRegister<'a>{
    id: Option<u64>,
    entered_at: u64,
    exited_at: Option<u64>,
    machine_id: Option<&'a str>,
    production: u64,
    mixer: &'a str
}

#[tauri::command]
pub fn get_decanter(token: &str, id: Option<u64>, entered_at: Option<u64>, exited_at: Option<u64>, machine_id: Option<&str>, production: Option<u64>, mixer: Option<&str>) -> Option<String> {
    let mut url = format!("decanter?token={}", token);

    if let Some(id) = id {
        url.push_str(&format!("&id={}", id));
    }
    if let Some(machine_id) = machine_id {
        url.push_str(&format!("&machineId={}", machine_id));
    }
    if let Some(production) = production {
        url.push_str(&format!("&production={}", production));
    }
    if let Some(mixer) = mixer {
        url.push_str(&format!("&mixer={}", mixer));
    }
    if let Some(entered_at) = entered_at {
        url.push_str(&format!("&enteredAt={}", entered_at));
    }
    if let Some(exited_at) = exited_at {
        url.push_str(&format!("&exitedAt={}", exited_at));
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
pub fn register_decanter(token: &str, data: DecanterRegister) -> Option<String> {
    match backend::post(&format!("decanter?token={}", token), &data) {
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