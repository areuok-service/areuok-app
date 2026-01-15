//! HTTP client for remote API calls.
//!
//! This module provides functions to call the remote server API.
//! Server: http://20.41.108.70

use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::remote_models::*;

const API_BASE_URL: &str = "http://localhost:3000";

/// Create HTTP client instance
fn create_client() -> Result<Client, String> {
    Client::builder().build().map_err(|e| {
        log::error!("Failed to create HTTP client: {}", e);
        format!("Failed to create HTTP client: {}", e)
    })
}

/// Generic API request function
async fn api_request<T: DeserializeOwned>(
    method: reqwest::Method,
    endpoint: &str,
    body: Option<impl Serialize>,
) -> Result<T, String> {
    let client = create_client()?;
    let url = format!("{}{}", API_BASE_URL, endpoint);

    log::debug!("{} {} - Starting API request", method, endpoint);

    let mut request = client.request(method.clone(), &url);

    if let Some(b) = body {
        request = request.json(&b);
    }

    let response = request.send().await.map_err(|e| {
        log::error!("API request failed for {} {}: {}", method, endpoint, e);
        format!("Request failed: {}", e)
    })?;

    let status = response.status();

    if !status.is_success() {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        log::error!(
            "API request returned error status {} for {} {}: {}",
            status,
            method,
            endpoint,
            error_text
        );
        return Err(format!("API error {}: {}", status, error_text));
    }

    log::debug!("API request succeeded: {} {} (status: {})", method, endpoint, status);

    let response_text = response.text().await.map_err(|e| {
        log::error!("Failed to read API response body for {} {}: {}", method, endpoint, e);
        format!("Failed to read response: {}", e)
    })?;

    log::trace!("API response body: {}", response_text);

    serde_json::from_str::<T>(&response_text).map_err(|e| {
        log::error!(
            "Failed to parse API response for {} {}: {}. Response body: {}",
            method,
            endpoint,
            e,
            response_text
        );
        format!("Failed to parse response: {}. Response: {}", e, response_text)
    })
}

// =============================================================================
// Device APIs
// =============================================================================

/// Register or update device
pub async fn register_device(
    device_name: &str,
    imei: Option<&str>,
    mode: DeviceMode,
) -> Result<Device, String> {
    log::info!("Registering device: {} (mode: {:?})", device_name, mode);
    #[derive(Serialize)]
    struct RequestBody {
        device_name: String,
        imei: Option<String>,
        mode: DeviceMode,
    }

    let body = RequestBody {
        device_name: device_name.to_string(),
        imei: imei.map(|s| s.to_string()),
        mode,
    };

    api_request(reqwest::Method::POST, "/devices/register", Some(body)).await
}

/// Get device info
pub async fn get_device(device_id: &str) -> Result<Device, String> {
    log::info!("Getting device info: {}", device_id);
    let endpoint = format!("/devices/{}", device_id);
    api_request(reqwest::Method::GET, &endpoint, None::<()>).await
}

/// Update device name
pub async fn update_device_name(device_id: &str, new_name: &str) -> Result<Device, String> {
    log::info!("Updating device name: {} -> {}", device_id, new_name);
    #[derive(Serialize)]
    struct RequestBody {
        device_name: String,
    }

    let body = RequestBody {
        device_name: new_name.to_string(),
    };

    let endpoint = format!("/devices/{}/name", device_id);
    api_request(reqwest::Method::PATCH, &endpoint, Some(body)).await
}

/// Sign in for a device
pub async fn device_signin(device_id: &str) -> Result<SigninResponse, String> {
    log::info!("Device sign-in: {}", device_id);
    let endpoint = format!("/devices/{}/signin", device_id);
    api_request(reqwest::Method::POST, &endpoint, None::<()>).await
}

/// Get device status
pub async fn get_device_status(device_id: &str) -> Result<DeviceStatus, String> {
    log::info!("Getting device status: {}", device_id);
    let endpoint = format!("/devices/{}/status", device_id);
    api_request(reqwest::Method::GET, &endpoint, None::<()>).await
}

/// Search devices
pub async fn search_devices(query: &str) -> Result<DeviceSearchResponse, String> {
    log::info!("Searching devices with query: {}", query);
    let endpoint = format!("/search/devices?q={}", urlencoding::encode(query));
    api_request(reqwest::Method::GET, &endpoint, None::<()>).await
}

// =============================================================================
// Supervision APIs
// =============================================================================

/// Send supervision request
pub async fn send_supervision_request_api(
    supervisor_id: &str,
    target_id: &str,
) -> Result<SupervisionRequest, String> {
    log::info!("Sending supervision request via API: {} -> {}", supervisor_id, target_id);
    #[derive(Serialize)]
    struct RequestBody {
        supervisor_id: String,
        target_id: String,
    }

    let body = RequestBody {
        supervisor_id: supervisor_id.to_string(),
        target_id: target_id.to_string(),
    };

    api_request(reqwest::Method::POST, "/supervision/request", Some(body)).await
}

/// Get pending supervision requests
pub async fn get_pending_requests(device_id: &str) -> Result<PendingRequestsResponse, String> {
    log::info!("Getting pending supervision requests via API for {}", device_id);
    let endpoint = format!("/supervision/pending/{}", device_id);
    api_request(reqwest::Method::GET, &endpoint, None::<()>).await
}

/// Accept supervision request
pub async fn accept_supervision_request_api(
    supervisor_id: &str,
    target_id: &str,
) -> Result<(), String> {
    log::info!("Accepting supervision request via API: {} -> {}", supervisor_id, target_id);
    #[derive(Serialize)]
    struct RequestBody {
        supervisor_id: String,
        target_id: String,
    }

    let body = RequestBody {
        supervisor_id: supervisor_id.to_string(),
        target_id: target_id.to_string(),
    };

    api_request(reqwest::Method::POST, "/supervision/accept", Some(body)).await
}

/// Reject supervision request
pub async fn reject_supervision_request_api(
    supervisor_id: &str,
    target_id: &str,
) -> Result<(), String> {
    log::info!("Rejecting supervision request via API: {} -> {}", supervisor_id, target_id);
    #[derive(Serialize)]
    struct RequestBody {
        supervisor_id: String,
        target_id: String,
    }

    let body = RequestBody {
        supervisor_id: supervisor_id.to_string(),
        target_id: target_id.to_string(),
    };

    api_request(reqwest::Method::POST, "/supervision/reject", Some(body)).await
}

/// Get supervision relationship list
pub async fn get_supervision_list(device_id: &str) -> Result<SupervisionListResponse, String> {
    log::info!("Getting supervision list via API for {}", device_id);
    let endpoint = format!("/supervision/list/{}", device_id);
    api_request(reqwest::Method::GET, &endpoint, None::<()>).await
}

/// Remove supervision relationship
pub async fn remove_supervision_relationship_api(relation_id: &str) -> Result<(), String> {
    log::info!("Removing supervision relationship via API: {}", relation_id);
    let endpoint = format!("/supervision/{}", relation_id);
    api_request(reqwest::Method::DELETE, &endpoint, None::<()>).await
}
