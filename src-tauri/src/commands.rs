//! Tauri command handlers for the areuok application.
//!
//! This module contains all Tauri commands that can be invoked from the frontend.
//! Commands are organized into logical groups: sign-in, device, supervision, and utilities.

use chrono::{NaiveDate, Utc};
use uuid::Uuid;

use crate::models::{
    DeviceConfig, DeviceMode, DeviceStatus, EmailConfig, Quote, SigninData,
    SupervisionRelationship, SupervisionRequest, SupervisionRequestStatus, SupervisorStatus,
};
use crate::services::{fetch_hitokoto, send_signin_email};
use crate::storage;

// =============================================================================
// Utility Functions
// =============================================================================

/// Get today's date in YYYY-MM-DD format
fn get_today_date() -> String {
    Utc::now().format("%Y-%m-%d").to_string()
}

/// Check if the streak should continue based on previous sign-in data
fn should_continue_streak(data: &Option<SigninData>) -> bool {
    let Some(saved) = data else { return false };

    let Ok(last_date) = NaiveDate::parse_from_str(&saved.last_signin_date, "%Y-%m-%d") else {
        return false;
    };

    let today = Utc::now().date_naive();
    let yesterday = today - chrono::Duration::days(1);
    last_date == yesterday
}

/// Get a fallback quote when API fails
fn get_fallback_quote() -> Quote {
    Quote {
        text: "今天的签到已完成，继续加油！".to_string(),
        author: "系统".to_string(),
    }
}

// =============================================================================
// Sign-in Commands
// =============================================================================

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn load_signin_data() -> Result<Option<SigninData>, String> {
    storage::load_data().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn signin(name: String) -> Result<SigninData, String> {
    let saved_data = storage::load_data().map_err(|e| e.to_string())?;
    let today = get_today_date();

    let new_data = calculate_signin_data(&saved_data, &name, &today)?;
    storage::save_data(&new_data).map_err(|e| e.to_string())?;

    send_signin_notification(&name, new_data.streak).await;

    Ok(new_data)
}

/// Calculate new sign-in data based on existing data
fn calculate_signin_data(
    saved_data: &Option<SigninData>,
    name: &str,
    today: &str,
) -> Result<SigninData, String> {
    let (new_streak, mut signin_history) = match saved_data {
        Some(data) if data.last_signin_date == today => {
            return Ok(data.clone());
        }
        Some(data) if should_continue_streak(saved_data) => {
            (data.streak + 1, data.signin_history.clone())
        }
        _ => (1, vec![]),
    };

    if !signin_history.contains(&today.to_string()) {
        signin_history.push(today.to_string());
    }

    Ok(SigninData {
        name: name.to_string(),
        last_signin_date: today.to_string(),
        streak: new_streak,
        signin_history,
    })
}

/// Send email notification for sign-in (non-blocking)
async fn send_signin_notification(name: &str, streak: i32) {
    let email_config = match storage::load_email_config() {
        Ok(config) if config.enabled => config,
        _ => return,
    };

    let quote = fetch_hitokoto()
        .await
        .unwrap_or_else(|_| get_fallback_quote());

    if let Err(e) = send_signin_email(name, streak, &quote, &email_config) {
        eprintln!("Failed to send email: {}", e);
    }
}

#[tauri::command]
pub fn signout() -> Result<(), String> {
    storage::delete_data().map_err(|e| e.to_string())
}

// =============================================================================
// Quote Commands
// =============================================================================

#[tauri::command]
pub async fn get_daily_quote() -> Result<Quote, String> {
    fetch_hitokoto().await
}

// =============================================================================
// Email Config Commands
// =============================================================================

#[tauri::command]
pub fn get_email_config() -> Result<EmailConfig, String> {
    storage::load_email_config().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_email_config_command(config: EmailConfig) -> Result<(), String> {
    storage::save_email_config(&config).map_err(|e| e.to_string())
}

// =============================================================================
// Device Commands
// =============================================================================

#[tauri::command]
pub fn get_device_config() -> Result<DeviceConfig, String> {
    storage::load_or_create_device_config().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_device_mode(mode: DeviceMode) -> Result<DeviceConfig, String> {
    let mut config = storage::load_or_create_device_config().map_err(|e| e.to_string())?;
    config.device.mode = mode;
    storage::save_device_config(&config).map_err(|e| e.to_string())?;
    Ok(config)
}

#[tauri::command]
pub fn update_device_name(name: String) -> Result<DeviceConfig, String> {
    let mut config = storage::load_or_create_device_config().map_err(|e| e.to_string())?;
    config.device.device_name = name;
    storage::save_device_config(&config).map_err(|e| e.to_string())?;
    Ok(config)
}

#[tauri::command]
pub fn set_device_imei(imei: String) -> Result<DeviceConfig, String> {
    let mut config = storage::load_or_create_device_config().map_err(|e| e.to_string())?;
    config.device.imei = Some(imei);
    storage::save_device_config(&config).map_err(|e| e.to_string())?;
    Ok(config)
}

#[tauri::command]
pub async fn get_device_imei() -> Result<String, String> {
    let config = storage::load_or_create_device_config().map_err(|e| e.to_string())?;
    Ok(config.device.imei.unwrap_or(config.device.device_id))
}

// =============================================================================
// Supervision Request Commands
// =============================================================================

#[tauri::command]
pub fn send_supervision_request(target_device_id: String) -> Result<SupervisionRequest, String> {
    let mut config = storage::load_or_create_device_config().map_err(|e| e.to_string())?;

    if config.device.mode != DeviceMode::Supervisor {
        return Err("Only supervisor devices can send supervision requests".to_string());
    }

    let request = SupervisionRequest {
        request_id: Uuid::new_v4().to_string(),
        supervisor_device_id: config.device.device_id.clone(),
        supervisor_device_name: config.device.device_name.clone(),
        target_device_id,
        status: SupervisionRequestStatus::Pending,
        created_at: Utc::now().to_rfc3339(),
    };

    config.supervision_requests.push(request.clone());
    storage::save_device_config(&config).map_err(|e| e.to_string())?;

    Ok(request)
}

#[tauri::command]
pub fn cancel_supervision_request(request_id: String) -> Result<(), String> {
    let mut config = storage::load_or_create_device_config().map_err(|e| e.to_string())?;

    let request = config
        .supervision_requests
        .iter_mut()
        .find(|r| r.request_id == request_id)
        .ok_or("Request not found")?;

    request.status = SupervisionRequestStatus::Cancelled;
    storage::save_device_config(&config).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_pending_supervision_requests() -> Result<Vec<SupervisionRequest>, String> {
    let config = storage::load_or_create_device_config().map_err(|e| e.to_string())?;
    let my_device_id = &config.device.device_id;

    let pending = config
        .supervision_requests
        .iter()
        .filter(|r| {
            r.target_device_id == *my_device_id && r.status == SupervisionRequestStatus::Pending
        })
        .cloned()
        .collect();

    Ok(pending)
}

#[tauri::command]
pub fn accept_supervision_request(request_id: String) -> Result<SupervisionRelationship, String> {
    let mut config = storage::load_or_create_device_config().map_err(|e| e.to_string())?;

    let request = find_pending_request(&config, &request_id)?;
    validate_request_target(&config, &request)?;

    let relationship = create_relationship_from_request(&config, &request);
    config.supervision_relationships.push(relationship.clone());

    update_request_status(&mut config, &request_id, SupervisionRequestStatus::Accepted);
    storage::save_device_config(&config).map_err(|e| e.to_string())?;

    Ok(relationship)
}

/// Find a pending supervision request by ID
fn find_pending_request(
    config: &DeviceConfig,
    request_id: &str,
) -> Result<SupervisionRequest, String> {
    config
        .supervision_requests
        .iter()
        .find(|r| r.request_id == request_id && r.status == SupervisionRequestStatus::Pending)
        .cloned()
        .ok_or_else(|| "Request not found or already processed".to_string())
}

/// Validate that the request is for this device
fn validate_request_target(
    config: &DeviceConfig,
    request: &SupervisionRequest,
) -> Result<(), String> {
    if request.target_device_id != config.device.device_id {
        return Err("This request is not for this device".to_string());
    }
    Ok(())
}

/// Create a supervision relationship from an accepted request
fn create_relationship_from_request(
    config: &DeviceConfig,
    request: &SupervisionRequest,
) -> SupervisionRelationship {
    SupervisionRelationship {
        relationship_id: Uuid::new_v4().to_string(),
        supervisor_device_id: request.supervisor_device_id.clone(),
        supervisor_device_name: request.supervisor_device_name.clone(),
        supervised_device_id: config.device.device_id.clone(),
        supervised_device_name: config.device.device_name.clone(),
        established_at: Utc::now().to_rfc3339(),
        last_sync_at: Utc::now().to_rfc3339(),
    }
}

/// Update the status of a supervision request
fn update_request_status(
    config: &mut DeviceConfig,
    request_id: &str,
    status: SupervisionRequestStatus,
) {
    if let Some(r) = config
        .supervision_requests
        .iter_mut()
        .find(|r| r.request_id == request_id)
    {
        r.status = status;
    }
}

#[tauri::command]
pub fn reject_supervision_request(request_id: String) -> Result<(), String> {
    let mut config = storage::load_or_create_device_config().map_err(|e| e.to_string())?;

    let request = config
        .supervision_requests
        .iter_mut()
        .find(|r| r.request_id == request_id)
        .ok_or("Request not found")?;

    if request.target_device_id != config.device.device_id {
        return Err("This request is not for this device".to_string());
    }

    request.status = SupervisionRequestStatus::Rejected;
    storage::save_device_config(&config).map_err(|e| e.to_string())?;
    Ok(())
}

// =============================================================================
// Supervision Relationship Commands
// =============================================================================

#[tauri::command]
pub fn remove_supervision_relationship(relationship_id: String) -> Result<(), String> {
    let mut config = storage::load_or_create_device_config().map_err(|e| e.to_string())?;

    let initial_len = config.supervision_relationships.len();
    config
        .supervision_relationships
        .retain(|r| r.relationship_id != relationship_id);

    if config.supervision_relationships.len() < initial_len {
        storage::save_device_config(&config).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Relationship not found".to_string())
    }
}

#[tauri::command]
pub fn get_supervised_devices() -> Result<Vec<DeviceStatus>, String> {
    let config = storage::load_or_create_device_config().map_err(|e| e.to_string())?;
    let signin_data = storage::load_data().map_err(|e| e.to_string())?;
    let today = get_today_date();

    let statuses = config
        .supervision_relationships
        .iter()
        .filter(|r| r.supervisor_device_id == config.device.device_id)
        .map(|relationship| build_device_status(relationship, &signin_data, &today))
        .collect();

    Ok(statuses)
}

/// Build a device status from a supervision relationship
fn build_device_status(
    relationship: &SupervisionRelationship,
    signin_data: &Option<SigninData>,
    today: &str,
) -> DeviceStatus {
    let is_signed_in_today = signin_data
        .as_ref()
        .map(|d| d.last_signin_date == today)
        .unwrap_or(false);

    DeviceStatus {
        device_id: relationship.supervised_device_id.clone(),
        device_name: relationship.supervised_device_name.clone(),
        last_signin_date: signin_data
            .as_ref()
            .map(|d| d.last_signin_date.clone())
            .unwrap_or_default(),
        streak: signin_data.as_ref().map(|d| d.streak).unwrap_or(0),
        is_signed_in_today,
        last_sync_at: relationship.last_sync_at.clone(),
    }
}

#[tauri::command]
pub fn get_supervisor_status() -> Result<SupervisorStatus, String> {
    let config = storage::load_or_create_device_config().map_err(|e| e.to_string())?;
    let supervised_devices = get_supervised_devices()?;

    let pending_requests = config
        .supervision_requests
        .iter()
        .filter(|r| {
            r.supervisor_device_id == config.device.device_id
                && r.status == SupervisionRequestStatus::Pending
        })
        .cloned()
        .collect();

    Ok(SupervisorStatus {
        supervisor_device_id: config.device.device_id,
        supervised_devices,
        pending_requests,
    })
}
