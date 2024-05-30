use crate::backend;

#[tauri::command]
pub fn get_customers(token: &str) -> Option<String> {
    match backend::get(&format!("customer?token={}", token)) {
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
