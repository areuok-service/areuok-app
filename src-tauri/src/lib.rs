//! areuok - Desktop check-in/habit tracking application with multi-device supervision.
//!
//! This is the main library module that exposes the Tauri application entry point
//! and organizes the codebase into logical modules.

mod commands;
mod models;
mod services;
mod storage;

pub use commands::*;

/// Run the Tauri application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // Sign-in commands
            greet,
            load_signin_data,
            signin,
            signout,
            // Quote commands
            get_daily_quote,
            // Email config commands
            get_email_config,
            save_email_config_command,
            // Device commands
            get_device_config,
            set_device_mode,
            update_device_name,
            set_device_imei,
            get_device_imei,
            // Supervision request commands
            send_supervision_request,
            cancel_supervision_request,
            get_pending_supervision_requests,
            accept_supervision_request,
            reject_supervision_request,
            // Supervision relationship commands
            remove_supervision_relationship,
            get_supervised_devices,
            get_supervisor_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
