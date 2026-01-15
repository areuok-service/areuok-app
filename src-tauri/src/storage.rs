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
    log::debug!("Attempting to load sign-in data");
    let path = get_data_file_path()?;

    if path.exists() {
        let contents = fs::read_to_string(&path)?;
        let data: SigninData = serde_json::from_str(&contents)?;
        log::info!("Successfully loaded sign-in data: name={}, streak={}", data.name, data.streak);
        Ok(Some(data))
    } else {
        log::info!("No existing sign-in data found, returning None");
        Ok(None)
    }
}

/// Save sign-in data to storage
pub fn save_data(data: &SigninData) -> io::Result<()> {
    log::debug!("Saving sign-in data: name={}, streak={}", data.name, data.streak);
    let path = get_data_file_path()?;
    let json = serde_json::to_string_pretty(data)?;
    fs::write(&path, json)?;
    log::info!("Successfully saved sign-in data to {:?}", path);
    Ok(())
}

/// Delete sign-in data from storage
pub fn delete_data() -> io::Result<()> {
    log::debug!("Attempting to delete sign-in data");
    let path = get_data_file_path()?;
    if path.exists() {
        fs::remove_file(&path)?;
        log::info!("Successfully deleted sign-in data");
    } else {
        log::warn!("No sign-in data file found to delete");
    }
    Ok(())
}

/// Load email configuration from storage
pub fn load_email_config() -> io::Result<EmailConfig> {
    log::debug!("Attempting to load email configuration");
    let path = get_email_config_path()?;

    if path.exists() {
        let contents = fs::read_to_string(&path)?;
        let config: EmailConfig = serde_json::from_str(&contents)?;
        log::info!("Successfully loaded email configuration: enabled={}", config.enabled);
        Ok(config)
    } else {
        log::info!("No existing email configuration found, returning default");
        Ok(EmailConfig::default())
    }
}

/// Save email configuration to storage
pub fn save_email_config(config: &EmailConfig) -> io::Result<()> {
    log::debug!("Saving email configuration: enabled={}", config.enabled);
    let path = get_email_config_path()?;
    let json = serde_json::to_string_pretty(config)?;
    fs::write(&path, json)?;
    log::info!("Successfully saved email configuration to {:?}", path);
    Ok(())
}

/// Load or create device configuration
pub fn load_or_create_device_config() -> io::Result<DeviceConfig> {
    log::debug!("Attempting to load device configuration");
    let path = get_device_config_path()?;

    if path.exists() {
        let contents = fs::read_to_string(&path)?;
        let config: DeviceConfig = serde_json::from_str(&contents)?;
        log::info!(
            "Successfully loaded device configuration: device_id={}, device_name={}, mode={:?}",
            config.device.device_id,
            config.device.device_name,
            config.device.mode
        );
        Ok(config)
    } else {
        log::info!("No existing device configuration found, creating new one");
        let device_id = Uuid::new_v4().to_string();
        let config = DeviceConfig::new(device_id.clone());
        save_device_config(&config)?;
        log::info!("Created new device with device_id={}", device_id);
        Ok(config)
    }
}

/// Save device configuration to storage
pub fn save_device_config(config: &DeviceConfig) -> io::Result<()> {
    log::debug!(
        "Saving device configuration: device_id={}, device_name={}",
        config.device.device_id,
        config.device.device_name
    );
    let path = get_device_config_path()?;
    let json = serde_json::to_string_pretty(config)?;
    fs::write(&path, json)?;
    log::info!("Successfully saved device configuration to {:?}", path);
    Ok(())
}
