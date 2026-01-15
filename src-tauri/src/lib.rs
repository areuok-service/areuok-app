//! areuok - Desktop check-in/habit tracking application with multi-device supervision.
//!
//! This is the main library module that exposes the Tauri application entry point
//! and organizes the codebase into logical modules.

mod api_client;
mod commands;
mod models;
mod remote_models;
mod services;
mod storage;

pub use commands::*;
pub use remote_models::*;

/// Run the Tauri application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    log::info!("Starting areuok application...");

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
            // Device commands (local)
            get_device_config,
            set_device_mode,
            update_device_name,
            set_device_imei,
            get_device_imei,
            // Supervision request commands (local)
            send_supervision_request,
            cancel_supervision_request,
            get_pending_supervision_requests,
            accept_supervision_request,
            reject_supervision_request,
            // Supervision relationship commands (local)
            remove_supervision_relationship,
            get_supervised_devices,
            get_supervisor_status,
            // Remote API commands
            device_register,
            device_get_info,
            device_update_name_api,
            device_signin_api,
            device_search,
            device_get_status,
            supervision_request_api,
            supervision_get_pending,
            supervision_accept_api,
            supervision_reject_api,
            supervision_list_api,
            supervision_remove_api,
            // Notification commands
            send_notification_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
