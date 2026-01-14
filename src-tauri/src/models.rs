//! Data models for the areuok application.
//!
//! This module contains all shared data structures used across the application.

use chrono::Utc;
use serde::{Deserialize, Serialize};

/// User sign-in data containing streak information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigninData {
    pub name: String,
    pub last_signin_date: String,
    pub streak: i32,
    pub signin_history: Vec<String>,
}

/// Device operating mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DeviceMode {
    /// Sign-in mode: device being supervised
    Signin,
    /// Supervisor mode: device supervising others
    Supervisor,
}

/// Device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub device_id: String,
    pub device_name: String,
    pub imei: Option<String>,
    pub mode: DeviceMode,
    pub created_at: String,
}

/// Supervision request status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SupervisionRequestStatus {
    Pending,
    Accepted,
    Rejected,
    Cancelled,
}

/// Supervision request between devices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupervisionRequest {
    pub request_id: String,
    pub supervisor_device_id: String,
    pub supervisor_device_name: String,
    pub target_device_id: String,
    pub status: SupervisionRequestStatus,
    pub created_at: String,
}

/// Established supervision relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupervisionRelationship {
    pub relationship_id: String,
    pub supervisor_device_id: String,
    pub supervisor_device_name: String,
    pub supervised_device_id: String,
    pub supervised_device_name: String,
    pub established_at: String,
    pub last_sync_at: String,
}

/// Device status for supervisors to view supervised devices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceStatus {
    pub device_id: String,
    pub device_name: String,
    pub last_signin_date: String,
    pub streak: i32,
    pub is_signed_in_today: bool,
    pub last_sync_at: String,
}

/// Supervisor status containing supervised devices and pending requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupervisorStatus {
    pub supervisor_device_id: String,
    pub supervised_devices: Vec<DeviceStatus>,
    pub pending_requests: Vec<SupervisionRequest>,
}

/// Daily inspirational quote
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub text: String,
    pub author: String,
}

/// Email configuration for notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    pub enabled: bool,
    pub to_email: String,
    pub smtp_server: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_email: String,
}

impl Default for EmailConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            to_email: String::new(),
            smtp_server: "smtp.gmail.com".to_string(),
            smtp_port: 587,
            smtp_username: String::new(),
            smtp_password: String::new(),
            from_email: String::new(),
        }
    }
}

/// Device configuration including device info and supervision data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConfig {
    pub device: DeviceInfo,
    pub supervision_requests: Vec<SupervisionRequest>,
    pub supervision_relationships: Vec<SupervisionRelationship>,
}

impl DeviceConfig {
    /// Create a new device config with a generated device ID
    pub fn new(device_id: String) -> Self {
        Self {
            device: DeviceInfo {
                device_id: device_id.clone(),
                device_name: format!("设备-{}", &device_id[..8]),
                imei: None,
                mode: DeviceMode::Signin,
                created_at: Utc::now().to_rfc3339(),
            },
            supervision_requests: vec![],
            supervision_relationships: vec![],
        }
    }
}

/// Response from hitokoto.cn API
#[derive(Debug, Deserialize)]
pub struct HitokotoResponse {
    pub hitokoto: String,
    pub from: String,
    pub from_who: Option<String>,
}
