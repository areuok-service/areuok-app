//! Storage operations for persistent data.
//!
//! This module handles all file I/O operations for storing and loading
//! application data, configurations, and device settings.

use std::fs;
use std::io;
use std::path::PathBuf;

use uuid::Uuid;

use crate::models::{DeviceConfig, EmailConfig, SigninData};

/// Get the application data directory path
fn get_app_dir() -> io::Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Config directory not found"))?;
    let app_dir = config_dir.join("areuok");
    fs::create_dir_all(&app_dir)?;
    Ok(app_dir)
}

/// Get the path to the sign-in data file
pub fn get_data_file_path() -> io::Result<PathBuf> {
    Ok(get_app_dir()?.join("data.json"))
}

/// Get the path to the email config file
pub fn get_email_config_path() -> io::Result<PathBuf> {
    Ok(get_app_dir()?.join("email_config.json"))
}

/// Get the path to the device config file
pub fn get_device_config_path() -> io::Result<PathBuf> {
    Ok(get_app_dir()?.join("device_config.json"))
}

/// Load sign-in data from storage
pub fn load_data() -> io::Result<Option<SigninData>> {
    let path = get_data_file_path()?;

    if path.exists() {
        let contents = fs::read_to_string(&path)?;
        let data: SigninData = serde_json::from_str(&contents)?;
        Ok(Some(data))
    } else {
        Ok(None)
    }
}

/// Save sign-in data to storage
pub fn save_data(data: &SigninData) -> io::Result<()> {
    let path = get_data_file_path()?;
    let json = serde_json::to_string_pretty(data)?;
    fs::write(&path, json)?;
    Ok(())
}

/// Delete sign-in data from storage
pub fn delete_data() -> io::Result<()> {
    let path = get_data_file_path()?;
    if path.exists() {
        fs::remove_file(&path)?;
    }
    Ok(())
}

/// Load email configuration from storage
pub fn load_email_config() -> io::Result<EmailConfig> {
    let path = get_email_config_path()?;

    if path.exists() {
        let contents = fs::read_to_string(&path)?;
        let config: EmailConfig = serde_json::from_str(&contents)?;
        Ok(config)
    } else {
        Ok(EmailConfig::default())
    }
}

/// Save email configuration to storage
pub fn save_email_config(config: &EmailConfig) -> io::Result<()> {
    let path = get_email_config_path()?;
    let json = serde_json::to_string_pretty(config)?;
    fs::write(&path, json)?;
    Ok(())
}

/// Load or create device configuration
pub fn load_or_create_device_config() -> io::Result<DeviceConfig> {
    let path = get_device_config_path()?;

    if path.exists() {
        let contents = fs::read_to_string(&path)?;
        let config: DeviceConfig = serde_json::from_str(&contents)?;
        Ok(config)
    } else {
        let device_id = Uuid::new_v4().to_string();
        let config = DeviceConfig::new(device_id);
        save_device_config(&config)?;
        Ok(config)
    }
}

/// Save device configuration to storage
pub fn save_device_config(config: &DeviceConfig) -> io::Result<()> {
    let path = get_device_config_path()?;
    let json = serde_json::to_string_pretty(config)?;
    fs::write(&path, json)?;
    Ok(())
}
