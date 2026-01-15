//! Tauri command handlers for the areuok application.
//!
//! This module contains all Tauri commands that can be invoked from the frontend.
//! Commands are organized into logical groups: sign-in, device, supervision, and utilities.

use chrono::{NaiveDate, Utc};
use uuid::Uuid;
use tauri_plugin_notification::NotificationExt;

use crate::api_client::{
    accept_supervision_request_api, device_signin, get_device, get_device_status,
    get_pending_requests, get_supervision_list, register_device, reject_supervision_request_api,
    remove_supervision_relationship_api, search_devices, send_supervision_request_api,
    update_device_name as update_device_name_api,
};
use crate::models::{
    DeviceConfig, DeviceMode, DeviceStatus, EmailConfig, Quote, SigninData,
    SupervisionRelationship, SupervisionRequest, SupervisionRequestStatus, SupervisorStatus,
};
use crate::remote_models::{
    Device as RemoteDevice, DeviceMode as RemoteDeviceMode, DeviceStatus as RemoteDeviceStatus,
    SigninResponse, SupervisionRelation, SupervisionRequest as RemoteSupervisionRequest,
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
    log::info!("Loading sign-in data");
    storage::load_data().map_err(|e| {
        log::error!("Failed to load sign-in data: {}", e);
        e.to_string()
    })
}

#[tauri::command]
pub async fn signin(name: String) -> Result<SigninData, String> {
    log::info!("Sign-in requested for user: {}", name);
    let saved_data = storage::load_data().map_err(|e| {
        log::error!("Failed to load sign-in data: {}", e);
        e.to_string()
    })?;
    let today = get_today_date();

    let new_data = calculate_signin_data(&saved_data, &name, &today)?;
    storage::save_data(&new_data).map_err(|e| {
        log::error!("Failed to save sign-in data: {}", e);
        e.to_string()
    })?;

    send_signin_notification(&name, new_data.streak).await;

    log::info!("User {} signed in successfully. New streak: {} days", name, new_data.streak);
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
            log::info!("User {} already signed in today", name);
            return Ok(data.clone());
        }
        Some(data) if should_continue_streak(saved_data) => {
            log::debug!("Continuing streak for user {}", name);
            (data.streak + 1, data.signin_history.clone())
        }
        _ => {
            log::debug!("Starting new streak for user {}", name);
            (1, vec![])
        }
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
    log::debug!("Preparing sign-in notification for {}", name);
    let email_config = match storage::load_email_config() {
        Ok(config) if config.enabled => config,
        Ok(_config) => {
            log::debug!("Email notification disabled for {}", name);
            return;
        }
        Err(e) => {
            log::warn!("Failed to load email config: {}", e);
            return;
        }
    };

    let quote = fetch_hitokoto().await.unwrap_or_else(|e| {
        log::warn!("Failed to fetch quote, using fallback: {}", e);
        get_fallback_quote()
    });

    if let Err(e) = send_signin_email(name, streak, &quote, &email_config) {
        log::error!("Failed to send email notification: {}", e);
    }
}

#[tauri::command]
pub fn signout() -> Result<(), String> {
    log::info!("User signed out, clearing all sign-in data");
    storage::delete_data().map_err(|e| {
        log::error!("Failed to delete sign-in data: {}", e);
        e.to_string()
    })
}

// =============================================================================
// Quote Commands
// =============================================================================

#[tauri::command]
pub async fn get_daily_quote() -> Result<Quote, String> {
    log::info!("Fetching daily quote");
    fetch_hitokoto().await.map_err(|e| {
        log::error!("Failed to fetch daily quote: {}", e);
        e
    })
}

// =============================================================================
// Email Config Commands
// =============================================================================

#[tauri::command]
pub fn get_email_config() -> Result<EmailConfig, String> {
    log::info!("Getting email configuration");
    storage::load_email_config().map_err(|e| {
        log::error!("Failed to load email config: {}", e);
        e.to_string()
    })
}

#[tauri::command]
pub fn save_email_config_command(config: EmailConfig) -> Result<(), String> {
    log::info!("Saving email configuration: enabled={}", config.enabled);
    storage::save_email_config(&config).map_err(|e| {
        log::error!("Failed to save email config: {}", e);
        e.to_string()
    })
}

// =============================================================================
// Device Commands
// =============================================================================

#[tauri::command]
pub fn get_device_config() -> Result<DeviceConfig, String> {
    log::info!("Getting device configuration");
    storage::load_or_create_device_config().map_err(|e| {
        log::error!("Failed to load device config: {}", e);
        e.to_string()
    })
}

#[tauri::command]
pub fn set_device_mode(mode: DeviceMode) -> Result<DeviceConfig, String> {
    log::info!("Setting device mode to {:?}", mode);
    let mut config = storage::load_or_create_device_config().map_err(|e| {
        log::error!("Failed to load device config: {}", e);
        e.to_string()
    })?;
    config.device.mode = mode;
    storage::save_device_config(&config).map_err(|e| {
        log::error!("Failed to save device config: {}", e);
        e.to_string()
    })?;
    Ok(config)
}

#[tauri::command]
pub fn update_device_name(name: String) -> Result<DeviceConfig, String> {
    log::info!("Updating device name to {}", name);
    let mut config = storage::load_or_create_device_config().map_err(|e| {
        log::error!("Failed to load device config: {}", e);
        e.to_string()
    })?;
    config.device.device_name = name.clone();
    storage::save_device_config(&config).map_err(|e| {
        log::error!("Failed to save device config: {}", e);
        e.to_string()
    })?;
    log::info!("Device name updated successfully to {}", name);
    Ok(config)
}

#[tauri::command]
pub fn set_device_imei(imei: String) -> Result<DeviceConfig, String> {
    log::info!("Setting device IMEI");
    let mut config = storage::load_or_create_device_config().map_err(|e| {
        log::error!("Failed to load device config: {}", e);
        e.to_string()
    })?;
    config.device.imei = Some(imei.clone());
    storage::save_device_config(&config).map_err(|e| {
        log::error!("Failed to save device config: {}", e);
        e.to_string()
    })?;
    log::info!("Device IMEI set successfully");
    Ok(config)
}

#[tauri::command]
pub async fn get_device_imei() -> Result<String, String> {
    log::info!("Getting device IMEI");
    let config = storage::load_or_create_device_config().map_err(|e| {
        log::error!("Failed to load device config: {}", e);
        e.to_string()
    })?;
    Ok(config.device.imei.unwrap_or(config.device.device_id))
}

// =============================================================================
// Supervision Request Commands
// =============================================================================

#[tauri::command]
pub fn send_supervision_request(target_device_id: String) -> Result<SupervisionRequest, String> {
    log::info!("Sending supervision request to device {}", target_device_id);
    let mut config = storage::load_or_create_device_config().map_err(|e| {
        log::error!("Failed to load device config: {}", e);
        e.to_string()
    })?;

    if config.device.mode != DeviceMode::Supervisor {
        log::warn!(
            "Non-supervisor device {} attempted to send supervision request",
            config.device.device_id
        );
        return Err("Only supervisor devices can send supervision requests".to_string());
    }

    let request = SupervisionRequest {
        request_id: Uuid::new_v4().to_string(),
        supervisor_device_id: config.device.device_id.clone(),
        supervisor_device_name: config.device.device_name.clone(),
        target_device_id: target_device_id.clone(),
        status: SupervisionRequestStatus::Pending,
        created_at: Utc::now().to_rfc3339(),
    };

    log::info!(
        "Created supervision request: {} -> {}",
        config.device.device_id,
        target_device_id
    );

    config.supervision_requests.push(request.clone());
    storage::save_device_config(&config).map_err(|e| {
        log::error!("Failed to save device config: {}", e);
        e.to_string()
    })?;

    Ok(request)
}

#[tauri::command]
pub fn cancel_supervision_request(request_id: String) -> Result<(), String> {
    log::info!("Cancelling supervision request {}", request_id);
    let mut config = storage::load_or_create_device_config().map_err(|e| {
        log::error!("Failed to load device config: {}", e);
        e.to_string()
    })?;

    let request = config
        .supervision_requests
        .iter_mut()
        .find(|r| r.request_id == request_id)
        .ok_or_else(|| {
            log::warn!("Supervision request {} not found", request_id);
            "Request not found".to_string()
        })?;

    request.status = SupervisionRequestStatus::Cancelled;
    storage::save_device_config(&config).map_err(|e| {
        log::error!("Failed to save device config: {}", e);
        e.to_string()
    })?;
    log::info!("Supervision request {} cancelled successfully", request_id);
    Ok(())
}

#[tauri::command]
pub fn get_pending_supervision_requests() -> Result<Vec<SupervisionRequest>, String> {
    log::info!("Getting pending supervision requests");
    let config = storage::load_or_create_device_config().map_err(|e| {
        log::error!("Failed to load device config: {}", e);
        e.to_string()
    })?;
    let my_device_id = &config.device.device_id;

    let pending: Vec<SupervisionRequest> = config
        .supervision_requests
        .iter()
        .filter(|r| {
            r.target_device_id == *my_device_id && r.status == SupervisionRequestStatus::Pending
        })
        .cloned()
        .collect();

    log::info!("Found {} pending supervision requests", pending.len());
    Ok(pending)
}

#[tauri::command]
pub fn accept_supervision_request(request_id: String) -> Result<SupervisionRelationship, String> {
    log::info!("Accepting supervision request {}", request_id);
    let mut config = storage::load_or_create_device_config().map_err(|e| {
        log::error!("Failed to load device config: {}", e);
        e.to_string()
    })?;

    let request = find_pending_request(&config, &request_id)?;
    validate_request_target(&config, &request)?;

    let relationship = create_relationship_from_request(&config, &request);
    log::info!(
        "Creating supervision relationship: {} supervised by {}",
        config.device.device_id,
        request.supervisor_device_id
    );

    config.supervision_relationships.push(relationship.clone());

    update_request_status(&mut config, &request_id, SupervisionRequestStatus::Accepted);
    storage::save_device_config(&config).map_err(|e| {
        log::error!("Failed to save device config: {}", e);
        e.to_string()
    })?;

    log::info!("Supervision request {} accepted successfully", request_id);
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
        .ok_or_else(|| {
            log::warn!("Pending supervision request {} not found", request_id);
            "Request not found or already processed".to_string()
        })
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
    log::info!("Rejecting supervision request {}", request_id);
    let mut config = storage::load_or_create_device_config().map_err(|e| {
        log::error!("Failed to load device config: {}", e);
        e.to_string()
    })?;

    let request = config
        .supervision_requests
        .iter_mut()
        .find(|r| r.request_id == request_id)
        .ok_or_else(|| {
            log::warn!("Supervision request {} not found", request_id);
            "Request not found".to_string()
        })?;

    if request.target_device_id != config.device.device_id {
        log::warn!("Supervision request {} not for this device", request_id);
        return Err("This request is not for this device".to_string());
    }

    request.status = SupervisionRequestStatus::Rejected;
    storage::save_device_config(&config).map_err(|e| {
        log::error!("Failed to save device config: {}", e);
        e.to_string()
    })?;
    log::info!("Supervision request {} rejected successfully", request_id);
    Ok(())
}

// =============================================================================
// Supervision Relationship Commands
// =============================================================================

#[tauri::command]
pub fn remove_supervision_relationship(relationship_id: String) -> Result<(), String> {
    log::info!("Removing supervision relationship {}", relationship_id);
    let mut config = storage::load_or_create_device_config().map_err(|e| {
        log::error!("Failed to load device config: {}", e);
        e.to_string()
    })?;

    let initial_len = config.supervision_relationships.len();
    config
        .supervision_relationships
        .retain(|r| r.relationship_id != relationship_id);

    if config.supervision_relationships.len() < initial_len {
        storage::save_device_config(&config).map_err(|e| {
            log::error!("Failed to save device config: {}", e);
            e.to_string()
        })?;
        log::info!("Supervision relationship {} removed successfully", relationship_id);
        Ok(())
    } else {
        log::warn!("Supervision relationship {} not found", relationship_id);
        Err("Relationship not found".to_string())
    }
}

#[tauri::command]
pub fn get_supervised_devices() -> Result<Vec<DeviceStatus>, String> {
    log::info!("Getting supervised devices");
    let config = storage::load_or_create_device_config().map_err(|e| {
        log::error!("Failed to load device config: {}", e);
        e.to_string()
    })?;
    let signin_data = storage::load_data().map_err(|e| {
        log::error!("Failed to load sign-in data: {}", e);
        e.to_string()
    })?;
    let today = get_today_date();

    let statuses: Vec<DeviceStatus> = config
        .supervision_relationships
        .iter()
        .filter(|r| r.supervisor_device_id == config.device.device_id)
        .map(|relationship| build_device_status(relationship, &signin_data, &today))
        .collect();

    log::info!("Found {} supervised devices", statuses.len());
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
    log::info!("Getting supervisor status");
    let config = storage::load_or_create_device_config().map_err(|e| {
        log::error!("Failed to load device config: {}", e);
        e.to_string()
    })?;
    let supervised_devices = get_supervised_devices()?;

    let pending_requests: Vec<SupervisionRequest> = config
        .supervision_requests
        .iter()
        .filter(|r| {
            r.supervisor_device_id == config.device.device_id
                && r.status == SupervisionRequestStatus::Pending
        })
        .cloned()
        .collect();

    log::info!(
        "Supervisor status: {} supervised devices, {} pending requests",
        supervised_devices.len(),
        pending_requests.len()
    );

    Ok(SupervisorStatus {
        supervisor_device_id: config.device.device_id,
        supervised_devices,
        pending_requests,
    })
}

// =============================================================================
// Remote API Commands
// =============================================================================

#[tauri::command]
pub async fn device_register(
    device_name: String,
    imei: Option<String>,
    mode: String,
) -> Result<RemoteDevice, String> {
    log::info!("Registering remote device: {} (mode: {})", device_name, mode);
    let remote_mode = match mode.as_str() {
        "signin" => RemoteDeviceMode::Signin,
        "supervisor" => RemoteDeviceMode::Supervisor,
        _ => {
            log::warn!("Invalid device mode: {}", mode);
            return Err("Invalid device mode".to_string());
        }
    };

    register_device(&device_name, imei.as_deref(), remote_mode).await
}

#[tauri::command]
pub async fn device_get_info(device_id: String) -> Result<RemoteDevice, String> {
    log::info!("Getting remote device info for {}", device_id);
    get_device(&device_id).await
}

#[tauri::command]
pub async fn device_update_name_api(
    device_id: String,
    new_name: String,
) -> Result<RemoteDevice, String> {
    log::info!("Updating remote device name: {} -> {}", device_id, new_name);
    update_device_name_api(&device_id, &new_name).await
}

#[tauri::command]
pub async fn device_signin_api(device_id: String) -> Result<SigninResponse, String> {
    log::info!("Remote device sign-in for {}", device_id);
    device_signin(&device_id).await
}

#[tauri::command]
pub async fn device_search(query: String) -> Result<Vec<RemoteDevice>, String> {
    log::info!("Searching remote devices with query: {}", query);
    search_devices(&query).await
}

#[tauri::command]
pub async fn supervision_request_api(
    supervisor_id: String,
    target_id: String,
) -> Result<RemoteSupervisionRequest, String> {
    log::info!("Sending remote supervision request: {} -> {}", supervisor_id, target_id);
    send_supervision_request_api(&supervisor_id, &target_id).await
}

#[tauri::command]
pub async fn supervision_get_pending(
    device_id: String,
) -> Result<Vec<RemoteSupervisionRequest>, String> {
    log::info!("Getting pending supervision requests for remote device {}", device_id);
    get_pending_requests(&device_id).await
}

#[tauri::command]
pub async fn supervision_accept_api(
    supervisor_id: String,
    target_id: String,
) -> Result<(), String> {
    log::info!("Accepting remote supervision request: {} -> {}", supervisor_id, target_id);
    accept_supervision_request_api(&supervisor_id, &target_id).await
}

#[tauri::command]
pub async fn supervision_reject_api(
    supervisor_id: String,
    target_id: String,
) -> Result<(), String> {
    log::info!("Rejecting remote supervision request: {} -> {}", supervisor_id, target_id);
    reject_supervision_request_api(&supervisor_id, &target_id).await
}

#[tauri::command]
pub async fn supervision_list_api(device_id: String) -> Result<Vec<SupervisionRelation>, String> {
    log::info!("Getting supervision list for remote device {}", device_id);
    get_supervision_list(&device_id).await
}

#[tauri::command]
pub async fn supervision_remove_api(relation_id: String) -> Result<(), String> {
    log::info!("Removing remote supervision relationship {}", relation_id);
    remove_supervision_relationship_api(&relation_id).await
}

#[tauri::command]
pub async fn device_get_status(device_id: String) -> Result<RemoteDeviceStatus, String> {
    log::info!("Getting remote device status for {}", device_id);
    get_device_status(&device_id).await
}

// =============================================================================
// Notification Commands
// =============================================================================

#[tauri::command]
pub async fn send_notification_command(
    app: tauri::AppHandle,
    title: String,
    body: String,
) -> Result<(), String> {
    log::info!("Sending notification: {} - {}", title, body);

    app.notification()
        .builder()
        .title(&title)
        .body(&body)
        .show()
        .map_err(|e| {
            log::error!("Failed to show notification: {}", e);
            e.to_string()
        })
}
