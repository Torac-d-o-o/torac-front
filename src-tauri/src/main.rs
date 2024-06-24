// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod backend;
mod commands;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::user::user_login,
            commands::user::user_register,
            commands::order::order_register,
            commands::order::update_order,
            commands::order::get_orders,
            commands::customer::get_customers,
            commands::customer::customer_register,
            commands::production::register_production,
            commands::production::get_production,
            commands::mixer::get_mixers,
            commands::mixer::register_mixer,
            commands::mixer::update_mixer,
            commands::decanter::register_decanter,
            commands::decanter::get_decanter
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
