use crate::downloader::depot_path;
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Read;
use std::time::Duration;
use tauri::AppHandle;
use zip::ZipArchive;

static APP_DETAILS_API: &str = "https://store.steampowered.com/api/appdetails?appids=";

async fn fetch_game_name(appid: u32) -> Option<String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .ok()?;
    let url = format!("{}{}", APP_DETAILS_API, appid);
    let Ok(resp) = client.get(&url).send().await else {
        return None;
    };
    if !resp.status().is_success() {
        return None;
    }
    let Ok(json) = resp.json::<serde_json::Value>().await else {
        return None;
    };
    json.get(appid.to_string())
        .and_then(|v| v.get("data"))
        .and_then(|d| d.get("name"))
        .and_then(|n| n.as_str())
        .map(|name| name.to_string())
}

#[tauri::command]
pub async fn fetch_game_name_cmd(appid: u32) -> Option<String> {
    fetch_game_name(appid).await
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DepotInfo {
    pub id: u32,
    pub sha: String,
    #[serde(default)]
    pub name: Option<String>,
}

fn line_comment(rest_of_line: &str) -> Option<String> {
    let text = rest_of_line.split("--").nth(1)?.trim();
    if text.is_empty() {
        None
    } else {
        Some(text.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameInfo {
    pub appid: Option<u32>,
    pub depots: Vec<DepotInfo>,
    pub dlc_depots: Vec<u32>,
    pub tokens: HashMap<u32, String>,
    pub manifest_ids: HashMap<u32, u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportResult {
    pub success: bool,
    pub game_info: Option<GameInfo>,
    pub game_name: String,
    pub has_previous_cache: bool,
    pub has_local_manifests: bool,
    pub message: String,
}

pub fn parse_lua(content: &str) -> Option<GameInfo> {
    let mut info = GameInfo {
        appid: None,
        depots: Vec::new(),
        dlc_depots: Vec::new(),
        tokens: HashMap::new(),
        manifest_ids: HashMap::new(),
    };

    let re_depot = Regex::new(r#"addappid\((\d+),\s*\d+,\s*"([^"]*)"\)([^\r\n]*)"#).unwrap();
    let mut seen = std::collections::HashSet::new();
    for cap in re_depot.captures_iter(content) {
        let id: u32 = cap[1].parse().unwrap();
        if seen.contains(&id) {
            continue;
        }
        seen.insert(id);
        info.depots.push(DepotInfo {
            id,
            sha: cap[2].to_string(),
            name: line_comment(&cap[3]),
        });
    }

    let re_dlc = Regex::new(r"(?m)^\s*addappid\((\d+)\)").unwrap();
    for cap in re_dlc.captures_iter(content) {
        let id: u32 = cap[1].parse().unwrap();
        if seen.contains(&id) || id == info.appid.unwrap_or(0) {
            continue;
        }
        info.dlc_depots.push(id);
    }

    if info.appid.is_none() && !info.depots.is_empty() {
        info.appid = Some(info.depots[0].id);
    }

    let re_token = Regex::new(r#"addtoken\((\d+),\s*"(\d+)"\)"#).unwrap();
    for cap in re_token.captures_iter(content) {
        info.tokens
            .insert(cap[1].parse().unwrap(), cap[2].to_string());
    }

    let re_manifest = Regex::new(r##"setManifestid\((\d+),\s*"(\d+)"##).unwrap();
    for cap in re_manifest.captures_iter(content) {
        if let (Ok(id), Ok(mid)) = (cap[1].parse::<u32>(), cap[2].parse::<u64>()) {
            info.manifest_ids.insert(id, mid);
        }
    }

    Some(info)
}

fn has_previous_cache(app_handle: &AppHandle) -> bool {
    let depot_dir = depot_path(app_handle);
    if !depot_dir.exists() {
        return false;
    }
    let has_vdf = depot_dir.join("config.vdf").exists();
    let has_manifests = std::fs::read_dir(&depot_dir)
        .map(|dir| {
            dir.filter_map(|e| e.ok())
                .any(|e| e.file_name().to_string_lossy().ends_with(".manifest"))
        })
        .unwrap_or(false);
    has_vdf && has_manifests
}

fn get_game_name(file_path: &str) -> String {
    let name = std::path::Path::new(file_path)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy();
    let re = Regex::new(r#"[<>:"/\\|?*]"#).unwrap();
    re.replace_all(&name, "").to_string()
}

#[tauri::command]
pub async fn import_lua(app_handle: AppHandle, file_path: String) -> ImportResult {
    let path = std::path::Path::new(&file_path);
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let mut lua_content: Option<String> = None;
    let mut has_local_manifests = false;
    let depot_dir = depot_path(&app_handle);
    std::fs::create_dir_all(&depot_dir).ok();

    match ext {
        "lua" => match std::fs::read_to_string(&file_path) {
            Ok(c) => lua_content = Some(c),
            Err(e) => {
                return ImportResult {
                    success: false,
                    game_info: None,
                    game_name: String::new(),
                    has_previous_cache: false,
                    has_local_manifests: false,
                    message: format!("无法读取文件: {}", e),
                };
            }
        },
        "zip" => {
            let file = match std::fs::File::open(&file_path) {
                Ok(f) => f,
                Err(e) => {
                    return ImportResult {
                        success: false,
                        game_info: None,
                        game_name: String::new(),
                        has_previous_cache: false,
                        has_local_manifests: false,
                        message: format!("无法打开 zip: {}", e),
                    };
                }
            };
            let mut archive = match ZipArchive::new(file) {
                Ok(a) => a,
                Err(e) => {
                    return ImportResult {
                        success: false,
                        game_info: None,
                        game_name: String::new(),
                        has_previous_cache: false,
                        has_local_manifests: false,
                        message: format!("无法解析 zip: {}", e),
                    };
                }
            };
            for i in 0..archive.len() {
                let mut entry = match archive.by_index(i) {
                    Ok(e) => e,
                    Err(_) => continue,
                };
                let name = entry.name().to_string();
                if name.contains("__MACOSX") {
                    continue;
                }
                if name.ends_with(".lua") {
                    let mut buf = String::new();
                    if entry.read_to_string(&mut buf).is_ok() {
                        lua_content = Some(buf);
                    }
                } else if name.ends_with(".manifest") {
                    let out_path = depot_dir.join(&name);
                    if let Ok(mut out_file) = std::fs::File::create(&out_path) {
                        if std::io::copy(&mut entry, &mut out_file).is_ok() {
                            has_local_manifests = true;
                        }
                    }
                }
            }
        }
        _ => {
            return ImportResult {
                success: false,
                game_info: None,
                game_name: String::new(),
                has_previous_cache: false,
                has_local_manifests: false,
                message: "不支持的格式".to_string(),
            };
        }
    }

    let lua_content = match lua_content {
        Some(c) => c,
        None => {
            return ImportResult {
                success: false,
                game_info: None,
                game_name: String::new(),
                has_previous_cache: false,
                has_local_manifests: false,
                message: "未找到 lua 文件".to_string(),
            };
        }
    };

    let game_info = match parse_lua(&lua_content) {
        Some(info) => info,
        None => {
            return ImportResult {
                success: false,
                game_info: None,
                game_name: String::new(),
                has_previous_cache: false,
                has_local_manifests: false,
                message: "无法解析清单内容".to_string(),
            };
        }
    };

    let mut game_name = get_game_name(&file_path);
    let has_cache = has_previous_cache(&app_handle);
    let appid = game_info.appid;

    if let Some(id) = appid {
        if let Some(name) = fetch_game_name(id).await {
            if !name.is_empty() {
                game_name = name;
            }
        }
    }

    ImportResult {
        success: true,
        game_info: Some(game_info),
        game_name,
        has_previous_cache: has_cache,
        has_local_manifests,
        message: format!("清单已就绪，AppID: {:?}", appid),
    }
}
