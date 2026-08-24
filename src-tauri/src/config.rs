use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

const CONFIG_FILENAME: &str = "config.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub download_dir: String,
    pub auto_retry: bool,
    #[serde(default = "default_style")]
    pub window_style: String,
    #[serde(default = "default_theme")]
    pub theme_mode: String,
}

fn default_style() -> String {
    "default".to_string()
}

fn default_theme() -> String {
    "system".to_string()
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            download_dir: String::new(),
            auto_retry: true,
            window_style: default_style(),
            theme_mode: default_theme(),
        }
    }
}

fn config_path() -> PathBuf {
    let base = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join("depotgamedl").join(CONFIG_FILENAME)
}

#[tauri::command]
pub fn load_config() -> AppConfig {
    let path = config_path();
    let mut config = if path.exists() {
        let content = std::fs::read_to_string(&path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        AppConfig::default()
    };
    if config.window_style == "mica" {
        config.window_style = "mica_alt".to_string();
    }
    config
}

#[tauri::command]
pub fn save_config(config: AppConfig) {
    let path = config_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    std::fs::write(&path, serde_json::to_string_pretty(&config).unwrap()).ok();
}

#[tauri::command]
pub fn get_app_meta() -> serde_json::Value {
    let repository = env!("CARGO_PKG_REPOSITORY");
    let author = repository
        .split_once("github.com/")
        .and_then(|(_, rest)| rest.split('/').next())
        .unwrap_or("")
        .to_string();
    serde_json::json!({
        "name": env!("CARGO_PKG_NAME"),
        "display_name": env!("CARGO_PKG_DESCRIPTION"),
        "version": env!("CARGO_PKG_VERSION"),
        "repository": repository,
        "author": author,
    })
}

#[tauri::command]
pub fn get_default_download_dir() -> String {
    if PathBuf::from("D:\\").exists() {
        "D:\\".to_string()
    } else {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("C:\\"));
        home.join("Saved Games").to_string_lossy().to_string()
    }
}

#[tauri::command]
pub fn set_window_style(app: AppHandle, style: String, dark: bool) {
    apply_style(&app, &style, dark)
}

pub fn apply_style(app: &AppHandle, style: &str, dark: bool) {
    let Some(window) = app.get_webview_window("main") else {
        return;
    };
    let result = match style {
        "mica_alt" => window_vibrancy::apply_tabbed(&window, Some(dark)),
        "acrylic" => {
            let tint = if dark {
                (40, 40, 40, 220)
            } else {
                (255, 255, 255, 220)
            };
            window_vibrancy::apply_acrylic(&window, Some(tint))
        }
        _ => {
            let _ = window_vibrancy::clear_mica(&window);
            let _ = window_vibrancy::clear_tabbed(&window);
            window_vibrancy::clear_acrylic(&window)
        }
    };
    drop(result);
}
