use chrono::{NaiveDate, Utc};
use lettre::message::{header::ContentType, Mailbox};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SigninData {
    name: String,
    last_signin_date: String,
    streak: i32,
    signin_history: Vec<String>,
}

/// 设备模式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum DeviceMode {
    /// 签到模式：被监督的设备
    Signin,
    /// 监督模式：监督其他设备
    Supervisor,
}

/// 设备信息
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DeviceInfo {
    device_id: String,
    device_name: String,
    imei: Option<String>,
    mode: DeviceMode,
    created_at: String,
}

/// 监督请求状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum SupervisionRequestStatus {
    Pending,
    Accepted,
    Rejected,
    Cancelled,
}

/// 监督请求
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SupervisionRequest {
    request_id: String,
    supervisor_device_id: String,
    supervisor_device_name: String,
    target_device_id: String,
    status: SupervisionRequestStatus,
    created_at: String,
}

/// 监督关系
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SupervisionRelationship {
    relationship_id: String,
    supervisor_device_id: String,
    supervisor_device_name: String,
    supervised_device_id: String,
    supervised_device_name: String,
    established_at: String,
    last_sync_at: String,
}

/// 设备状态（用于监督者查看被监督设备的状态）
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DeviceStatus {
    device_id: String,
    device_name: String,
    last_signin_date: String,
    streak: i32,
    is_signed_in_today: bool,
    last_sync_at: String,
}

/// 监督者状态
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SupervisorStatus {
    supervisor_device_id: String,
    supervised_devices: Vec<DeviceStatus>,
    pending_requests: Vec<SupervisionRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Quote {
    text: String,
    author: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EmailConfig {
    enabled: bool,
    to_email: String,
    smtp_server: String,
    smtp_port: u16,
    smtp_username: String,
    smtp_password: String,
    from_email: String,
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

/// 设备配置，包含设备信息和监督关系
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DeviceConfig {
    device: DeviceInfo,
    supervision_requests: Vec<SupervisionRequest>,
    supervision_relationships: Vec<SupervisionRelationship>,
}

#[derive(Debug, Deserialize)]
struct HitokotoResponse {
    hitokoto: String,
    from: String,
    from_who: Option<String>,
}

fn get_data_file_path() -> io::Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Config directory not found"))?;
    let app_dir = config_dir.join("areuok");
    fs::create_dir_all(&app_dir)?;
    Ok(app_dir.join("data.json"))
}

fn get_email_config_path() -> io::Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Config directory not found"))?;
    let app_dir = config_dir.join("areuok");
    fs::create_dir_all(&app_dir)?;
    Ok(app_dir.join("email_config.json"))
}

fn get_device_config_path() -> io::Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Config directory not found"))?;
    let app_dir = config_dir.join("areuok");
    fs::create_dir_all(&app_dir)?;
    Ok(app_dir.join("device_config.json"))
}

fn load_data() -> io::Result<Option<SigninData>> {
    let path = get_data_file_path()?;

    if path.exists() {
        let contents = fs::read_to_string(&path)?;
        let data: SigninData = serde_json::from_str(&contents)?;
        Ok(Some(data))
    } else {
        Ok(None)
    }
}

fn save_data(data: &SigninData) -> io::Result<()> {
    let path = get_data_file_path()?;
    let json = serde_json::to_string_pretty(data)?;
    fs::write(&path, json)?;
    Ok(())
}

fn load_email_config() -> io::Result<EmailConfig> {
    let path = get_email_config_path()?;

    if path.exists() {
        let contents = fs::read_to_string(&path)?;
        let config: EmailConfig = serde_json::from_str(&contents)?;
        Ok(config)
    } else {
        Ok(EmailConfig::default())
    }
}

fn save_email_config(config: &EmailConfig) -> io::Result<()> {
    let path = get_email_config_path()?;
    let json = serde_json::to_string_pretty(config)?;
    fs::write(&path, json)?;
    Ok(())
}

fn load_or_create_device_config() -> io::Result<DeviceConfig> {
    let path = get_device_config_path()?;

    if path.exists() {
        let contents = fs::read_to_string(&path)?;
        let config: DeviceConfig = serde_json::from_str(&contents)?;
        Ok(config)
    } else {
        let device_id = Uuid::new_v4().to_string();
        let config = DeviceConfig {
            device: DeviceInfo {
                device_id: device_id.clone(),
                device_name: format!("设备-{}", &device_id[..8]),
                imei: None,
                mode: DeviceMode::Signin,
                created_at: Utc::now().to_rfc3339(),
            },
            supervision_requests: vec![],
            supervision_relationships: vec![],
        };
        save_device_config(&config)?;
        Ok(config)
    }
}

fn save_device_config(config: &DeviceConfig) -> io::Result<()> {
    let path = get_device_config_path()?;
    let json = serde_json::to_string_pretty(config)?;
    fs::write(&path, json)?;
    Ok(())
}

fn get_today_date() -> String {
    Utc::now().format("%Y-%m-%d").to_string()
}

fn should_continue_streak(data: &Option<SigninData>) -> bool {
    if let Some(saved) = data {
        if let Ok(last_date) = NaiveDate::parse_from_str(&saved.last_signin_date, "%Y-%m-%d") {
            let today = Utc::now().date_naive();
            let yesterday = today - chrono::Duration::days(1);
            return last_date == yesterday;
        }
    }
    false
}

async fn fetch_hitokoto() -> Result<Quote, String> {
    let client = reqwest::Client::new();
    // 使用免费的 hitokoto.cn API，无需注册
    let url = "https://v1.hitokoto.cn/";

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    let hitokoto_response: HitokotoResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let author = hitokoto_response
        .from_who
        .unwrap_or_else(|| hitokoto_response.from.clone());

    Ok(Quote {
        text: hitokoto_response.hitokoto,
        author,
    })
}

fn send_signin_email(
    name: &str,
    streak: i32,
    quote: &Quote,
    config: &EmailConfig,
) -> Result<(), String> {
    if !config.enabled || config.to_email.is_empty() {
        return Ok(());
    }

    let from = config
        .from_email
        .parse::<Mailbox>()
        .map_err(|e| format!("Invalid from email: {}", e))?;
    let to = config
        .to_email
        .parse::<Mailbox>()
        .map_err(|e| format!("Invalid to email: {}", e))?;

    let subject = format!("🔥 {} 签到成功！连续签到 {} 天", name, streak);
    let body = format!(
        "Hi {},\n\n\
        恭喜你今天成功签到！🎉\n\n\
        当前连续签到天数：{} 天 🔥\n\n\
        每日一言：\n\
        \"{}\"\n\
        - {}\n\n\
        继续保持，加油！💪\n\n\
        --\n\
        Are You OK?",
        name, streak, quote.text, quote.author
    );

    let email = Message::builder()
        .from(from)
        .to(to)
        .subject(&subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body)
        .map_err(|e| format!("Failed to build email: {}", e))?;

    let credentials = Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());

    let mailer = SmtpTransport::starttls_relay(&config.smtp_server)
        .map_err(|e| format!("Failed to create SMTP relay: {}", e))?
        .port(config.smtp_port)
        .credentials(credentials)
        .build();

    mailer
        .send(&email)
        .map_err(|e| format!("Failed to send email: {}", e))?;

    Ok(())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn load_signin_data() -> Result<Option<SigninData>, String> {
    load_data().map_err(|e| e.to_string())
}

#[tauri::command]
async fn signin(name: String) -> Result<SigninData, String> {
    let saved_data = load_data().map_err(|e| e.to_string())?;
    let today = get_today_date();
    let mut new_streak = 0;
    let mut signin_history: Vec<String> = vec![];

    if let Some(ref data) = saved_data {
        if should_continue_streak(&saved_data) {
            new_streak = data.streak + 1;
            signin_history = data.signin_history.clone();
        } else if data.last_signin_date == today {
            return Ok(data.clone());
        } else {
            new_streak = 1;
            signin_history = vec![];
        }
    } else {
        new_streak = 1;
    }

    if !signin_history.contains(&today) {
        signin_history.push(today.clone());
    }

    let new_data = SigninData {
        name: name.clone(),
        last_signin_date: today,
        streak: new_streak,
        signin_history,
    };

    save_data(&new_data).map_err(|e| e.to_string())?;

    let email_config = load_email_config().map_err(|e| e.to_string())?;

    if email_config.enabled {
        let quote = fetch_hitokoto().await.unwrap_or_else(|_| Quote {
            text: "今天的签到已完成，继续加油！".to_string(),
            author: "系统".to_string(),
        });

        send_signin_email(&name, new_streak, &quote, &email_config).map_err(|e| {
            eprintln!("Failed to send email: {}", e);
            e
        })?;
    }

    Ok(new_data)
}

#[tauri::command]
fn signout() -> Result<(), String> {
    let path = get_data_file_path().map_err(|e| e.to_string())?;
    if path.exists() {
        fs::remove_file(&path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn get_daily_quote() -> Result<Quote, String> {
    fetch_hitokoto().await
}

#[tauri::command]
fn get_email_config() -> Result<EmailConfig, String> {
    load_email_config().map_err(|e| e.to_string())
}

#[tauri::command]
fn save_email_config_command(config: EmailConfig) -> Result<(), String> {
    save_email_config(&config).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_device_config() -> Result<DeviceConfig, String> {
    load_or_create_device_config().map_err(|e| e.to_string())
}

#[tauri::command]
fn set_device_mode(mode: DeviceMode) -> Result<DeviceConfig, String> {
    let mut config = load_or_create_device_config().map_err(|e| e.to_string())?;
    config.device.mode = mode;
    save_device_config(&config).map_err(|e| e.to_string())?;
    Ok(config)
}

#[tauri::command]
fn update_device_name(name: String) -> Result<DeviceConfig, String> {
    let mut config = load_or_create_device_config().map_err(|e| e.to_string())?;
    config.device.device_name = name;
    save_device_config(&config).map_err(|e| e.to_string())?;
    Ok(config)
}

#[tauri::command]
fn set_device_imei(imei: String) -> Result<DeviceConfig, String> {
    let mut config = load_or_create_device_config().map_err(|e| e.to_string())?;
    config.device.imei = Some(imei);
    save_device_config(&config).map_err(|e| e.to_string())?;
    Ok(config)
}

#[tauri::command]
fn send_supervision_request(target_device_id: String) -> Result<SupervisionRequest, String> {
    let mut config = load_or_create_device_config().map_err(|e| e.to_string())?;

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
    save_device_config(&config).map_err(|e| e.to_string())?;

    Ok(request)
}

#[tauri::command]
fn cancel_supervision_request(request_id: String) -> Result<(), String> {
    let mut config = load_or_create_device_config().map_err(|e| e.to_string())?;

    if let Some(request) = config
        .supervision_requests
        .iter_mut()
        .find(|r| r.request_id == request_id)
    {
        request.status = SupervisionRequestStatus::Cancelled;
        save_device_config(&config).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Request not found".to_string())
    }
}

#[tauri::command]
fn get_pending_supervision_requests() -> Result<Vec<SupervisionRequest>, String> {
    let config = load_or_create_device_config().map_err(|e| e.to_string())?;

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
fn accept_supervision_request(request_id: String) -> Result<SupervisionRelationship, String> {
    let mut config = load_or_create_device_config().map_err(|e| e.to_string())?;

    let request = config
        .supervision_requests
        .iter()
        .find(|r| r.request_id == request_id && r.status == SupervisionRequestStatus::Pending)
        .cloned()
        .ok_or("Request not found or already processed")?;

    if request.target_device_id != config.device.device_id {
        return Err("This request is not for this device".to_string());
    }

    let relationship = SupervisionRelationship {
        relationship_id: Uuid::new_v4().to_string(),
        supervisor_device_id: request.supervisor_device_id.clone(),
        supervisor_device_name: request.supervisor_device_name.clone(),
        supervised_device_id: config.device.device_id.clone(),
        supervised_device_name: config.device.device_name.clone(),
        established_at: Utc::now().to_rfc3339(),
        last_sync_at: Utc::now().to_rfc3339(),
    };

    config.supervision_relationships.push(relationship.clone());

    if let Some(r) = config
        .supervision_requests
        .iter_mut()
        .find(|r| r.request_id == request_id)
    {
        r.status = SupervisionRequestStatus::Accepted;
    }

    save_device_config(&config).map_err(|e| e.to_string())?;

    Ok(relationship)
}

#[tauri::command]
fn reject_supervision_request(request_id: String) -> Result<(), String> {
    let mut config = load_or_create_device_config().map_err(|e| e.to_string())?;

    if let Some(request) = config
        .supervision_requests
        .iter_mut()
        .find(|r| r.request_id == request_id)
    {
        if request.target_device_id != config.device.device_id {
            return Err("This request is not for this device".to_string());
        }
        request.status = SupervisionRequestStatus::Rejected;
        save_device_config(&config).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Request not found".to_string())
    }
}

#[tauri::command]
fn remove_supervision_relationship(relationship_id: String) -> Result<(), String> {
    let mut config = load_or_create_device_config().map_err(|e| e.to_string())?;

    let initial_len = config.supervision_relationships.len();
    config
        .supervision_relationships
        .retain(|r| r.relationship_id != relationship_id);

    if config.supervision_relationships.len() < initial_len {
        save_device_config(&config).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Relationship not found".to_string())
    }
}

#[tauri::command]
fn get_supervised_devices() -> Result<Vec<DeviceStatus>, String> {
    let config = load_or_create_device_config().map_err(|e| e.to_string())?;
    let signin_data = load_data().map_err(|e| e.to_string())?;

    let today = get_today_date();
    let mut statuses = vec![];

    for relationship in &config.supervision_relationships {
        if relationship.supervisor_device_id == config.device.device_id {
            let is_signed_in_today = if let Some(ref data) = signin_data {
                data.last_signin_date == today
            } else {
                false
            };

            let status = DeviceStatus {
                device_id: relationship.supervised_device_id.clone(),
                device_name: relationship.supervised_device_name.clone(),
                last_signin_date: signin_data
                    .as_ref()
                    .map(|d| d.last_signin_date.clone())
                    .unwrap_or_default(),
                streak: signin_data.as_ref().map(|d| d.streak).unwrap_or(0),
                is_signed_in_today,
                last_sync_at: relationship.last_sync_at.clone(),
            };

            statuses.push(status);
        }
    }

    Ok(statuses)
}

#[tauri::command]
fn get_supervisor_status() -> Result<SupervisorStatus, String> {
    let config = load_or_create_device_config().map_err(|e| e.to_string())?;

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

#[tauri::command]
async fn get_device_imei() -> Result<String, String> {
    let config = load_or_create_device_config().map_err(|e| e.to_string())?;

    if let Some(imei) = config.device.imei {
        Ok(imei)
    } else {
        Ok(config.device.device_id)
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            load_signin_data,
            signin,
            signout,
            get_daily_quote,
            get_email_config,
            save_email_config_command,
            get_device_config,
            set_device_mode,
            update_device_name,
            send_supervision_request,
            cancel_supervision_request,
            get_pending_supervision_requests,
            accept_supervision_request,
            reject_supervision_request,
            remove_supervision_relationship,
            get_supervised_devices,
            get_supervisor_status,
            get_device_imei,
            set_device_imei
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
