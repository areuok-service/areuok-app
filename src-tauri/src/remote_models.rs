//! Remote API models for server communication.
//!
//! This module contains data structures that match the server's API responses.
//! Server: http://20.41.108.70

use serde::{Deserialize, Serialize};

/// Device mode from server
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DeviceMode {
    Signin,
    Supervisor,
}

/// Device information from server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub device_id: String,
    pub device_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imei: Option<String>,
    pub mode: DeviceMode,
    pub created_at: String,
    pub last_seen_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_updated_at: Option<String>,
}

/// Sign-in data from server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigninRecord {
    pub device_id: String,
    pub date: String,
    pub streak: i32,
}

/// Device status for supervisors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceStatus {
    pub device_id: String,
    pub device_name: String,
    pub mode: DeviceMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_signin: Option<String>,
    pub streak: i32,
}

/// Supervision request status from server
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SupervisionStatus {
    Pending,
    Accepted,
    Rejected,
}

/// Supervision request from server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupervisionRequest {
    pub request_id: String,
    pub supervisor_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supervisor_name: Option<String>,
    pub target_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_name: Option<String>,
    pub status: SupervisionStatus,
    pub created_at: String,
}

/// Supervision relationship from server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupervisionRelation {
    pub relation_id: String,
    pub supervisor_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supervisor_name: Option<String>,
    pub target_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_name: Option<String>,
    pub created_at: String,
}

/// Sign-in response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigninResponse {
    pub streak: i32,
}

/// Device search response
pub type DeviceSearchResponse = Vec<Device>;

/// Supervision list response
pub type SupervisionListResponse = Vec<SupervisionRelation>;

/// Pending requests response
pub type PendingRequestsResponse = Vec<SupervisionRequest>;
