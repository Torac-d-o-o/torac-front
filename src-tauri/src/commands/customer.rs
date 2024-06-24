use serde::{Deserialize, Serialize};

use crate::backend;


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerRegistration<'a> {
    pub name_surname: &'a str,
    pub address: &'a str,
    pub postal_code: i32,
    pub phone_number: &'a str,
}

#[tauri::command]
pub fn customer_register(data: CustomerRegistration, token: &str) -> Option<String> {
    match backend::post(&format!("customer?token={}", token), &data) {
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
pub fn get_customers(token: &str, customer_name: Option<&str>, address: Option<&str>) -> Option<String> {
    println!("Message from Rust before formating get_customers:");
    let mut url = format!("customer?token={}", token);
    
    if let Some(customer_name) = customer_name {
        url.push_str(&format!("&customerName={}", customer_name));
    }
    if let Some(address) = address {
        url.push_str(&format!("&address={}", address))
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
