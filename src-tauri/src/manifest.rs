use crate::downloader::depot_path;
use crate::parser::GameInfo;
use reqwest::Client;
use std::collections::HashMap;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tauri::{AppHandle, Emitter};

pub(crate) static CANCEL_MANIFEST: AtomicBool = AtomicBool::new(false);

pub static STEAMCMD_API: &str = "https://api.steamcmd.net/v1/info/";
pub static CDN_BASE: &str = "http://steamcdn-a.akamaihd.net";

struct RequestCodeSource {
    base: &'static str,
    json_content: bool,
    user_agent: Option<&'static str>,
}

static REQUEST_CODE_SOURCES: [RequestCodeSource; 4] = [
    RequestCodeSource {
        base: "https://manifest.steam.ooo/",
        json_content: false,
        user_agent: None,
    },
    RequestCodeSource {
        base: "https://manifest.opensteamtool.com/",
        json_content: false,
        user_agent: Some("OpenSteamTool/1.0"),
    },
    RequestCodeSource {
        base: "http://gmrc.wudrm.com/manifest/",
        json_content: false,
        user_agent: None,
    },
    RequestCodeSource {
        base: "https://manifest.steam.run/api/manifest/",
        json_content: true,
        user_agent: None,
    },
];

pub async fn fetch_latest_manifests(
    game_info: &GameInfo,
    emitter: &AppHandle,
) -> HashMap<u32, u64> {
    let appid = game_info.appid.unwrap_or(0);
    let depot_ids: Vec<u32> = game_info.depots.iter().map(|d| d.id).collect();
    let mut result = HashMap::new();

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap_or_default();
    let url = format!("{}{}", STEAMCMD_API, appid);
    let _ = emitter;

    if let Ok(resp) = client.get(&url).send().await {
        if resp.status().is_success() {
            if let Ok(json) = resp.json::<serde_json::Value>().await {
                if let Some(data_root) = json.get("data").and_then(|d| d.get(appid.to_string())) {
                    let mut depot_sources: Vec<&serde_json::Map<String, serde_json::Value>> =
                        Vec::new();

                    if let Some(depots_obj) = data_root.get("depots").and_then(|d| d.as_object()) {
                        depot_sources.push(depots_obj);
                    }

                    if let Some(branches) = data_root.get("branches").and_then(|b| b.as_object()) {
                        for (_, branch_val) in branches {
                            if let Some(depots_obj) =
                                branch_val.get("depots").and_then(|d| d.as_object())
                            {
                                depot_sources.push(depots_obj);
                            }
                        }
                    }

                    if let Some(priv_branches) =
                        data_root.get("privatebranches").and_then(|b| b.as_object())
                    {
                        for (_, branch_val) in priv_branches {
                            if let Some(depots_obj) =
                                branch_val.get("depots").and_then(|d| d.as_object())
                            {
                                depot_sources.push(depots_obj);
                            }
                        }
                    }

                    for &did in &depot_ids {
                        if result.contains_key(&did) {
                            continue;
                        }
                        for source in &depot_sources {
                            if let Some(depot_val) = source.get(&did.to_string()) {
                                if let Some(gid) = extract_manifest_gid(depot_val) {
                                    result.insert(did, gid);
                                }
                                break;
                            }
                        }
                    }

                    if let Some(hdlc) = data_root.get("hasdepotsindlc").and_then(|d| d.as_object())
                    {
                        for &did in &depot_ids {
                            if !result.contains_key(&did) {
                                if let Some(depot_val) = hdlc.get(&did.to_string()) {
                                    if let Some(gid) = extract_manifest_gid(depot_val) {
                                        result.insert(did, gid);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    for &did in &depot_ids {
        if !result.contains_key(&did) {
            if let Some(mid) = game_info.manifest_ids.get(&did) {
                result.insert(did, *mid);
            }
        }
    }

    let missing: Vec<u32> = depot_ids
        .iter()
        .cloned()
        .filter(|id| !result.contains_key(id))
        .collect();
    for &did in &missing {
        for &neighbor in &[did + 1, did.wrapping_sub(1)] {
            if neighbor == appid {
                continue;
            }
            let url = format!("{}{}", STEAMCMD_API, neighbor);
            if let Ok(resp) = client.get(&url).send().await {
                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    if let Some(gid) = find_manifest_in_data(&json, did) {
                        result.insert(did, gid);
                        break;
                    }
                }
            }
        }
    }

    result
}

fn extract_manifest_gid(depot: &serde_json::Value) -> Option<u64> {
    fn to_u64(v: &serde_json::Value) -> Option<u64> {
        if let Some(num) = v.as_u64() {
            return Some(num);
        }
        if let Some(s) = v.as_str() {
            return s.parse::<u64>().ok();
        }
        None
    }

    if let Some(gid_val) = depot
        .get("manifests")
        .and_then(|m| m.get("public"))
        .and_then(|m| m.get("gid"))
    {
        if let Some(gid) = to_u64(gid_val) {
            return Some(gid);
        }
    }

    if let Some(manifests) = depot.get("manifests").and_then(|m| m.as_object()) {
        for (_, branch) in manifests {
            if let Some(gid_val) = branch.get("gid") {
                if let Some(gid) = to_u64(gid_val) {
                    return Some(gid);
                }
            }
        }
    }
    None
}

fn find_manifest_in_data(data: &serde_json::Value, target: u32) -> Option<u64> {
    fn search(obj: &serde_json::Value, target: u32, depth: usize) -> Option<u64> {
        if depth > 5 {
            return None;
        }
        if let Some(map) = obj.as_object() {
            if let Some(v) = map.get(&target.to_string()) {
                if let Some(gid) = extract_manifest_gid(v) {
                    return Some(gid);
                }
            }
            for (key, val) in map {
                if [
                    "branches",
                    "privatebranches",
                    "common",
                    "extended",
                    "config",
                ]
                .contains(&key.as_str())
                {
                    continue;
                }
                if let Some(gid) = search(val, target, depth + 1) {
                    return Some(gid);
                }
            }
        } else if let Some(arr) = obj.as_array() {
            for item in arr {
                if let Some(gid) = search(item, target, depth + 1) {
                    return Some(gid);
                }
            }
        }
        None
    }
    search(data, target, 0)
}

pub async fn fetch_request_code(mid: u64) -> Option<String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .unwrap_or_default();

    for source in &REQUEST_CODE_SOURCES {
        let mut req = client.get(&format!("{}{}", source.base, mid));
        if let Some(ua) = source.user_agent {
            req = req.header("User-Agent", ua);
        }
        let Ok(resp) = req.send().await else {
            continue;
        };
        let Ok(text) = resp.text().await else {
            continue;
        };
        let code = if source.json_content {
            match serde_json::from_str::<serde_json::Value>(&text) {
                Ok(json) => json
                    .get("content")
                    .and_then(|c| c.as_str())
                    .map(|s| s.to_string()),
                Err(_) => None,
            }
        } else {
            Some(text.trim().to_string())
        };
        if let Some(code) = code {
            if code.parse::<u64>().is_ok() {
                return Some(code);
            }
        }
    }
    None
}

pub async fn download_manifest(
    depot_id: u32,
    manifest_id: u64,
    request_code: &str,
    out_dir: &Path,
    emitter: &AppHandle,
) -> bool {
    let url = format!(
        "{}/depot/{}/manifest/{}/5/{}",
        CDN_BASE, depot_id, manifest_id, request_code
    );
    let _ = emitter;

    let client = Client::builder()
        .timeout(Duration::from_secs(60))
        .build()
        .unwrap_or_default();

    if let Ok(resp) = client.get(&url).send().await {
        if resp.status().is_success() {
            if let Ok(data) = resp.bytes().await {
                let cursor = std::io::Cursor::new(data);
                if let Ok(mut archive) = zip::ZipArchive::new(cursor) {
                    if let Ok(mut file) = archive.by_index(0) {
                        let file_name = format!("{}_{}.manifest", depot_id, manifest_id);
                        let out_path = out_dir.join(&file_name);
                        if let Ok(mut out_file) = std::fs::File::create(out_path) {
                            if std::io::copy(&mut file, &mut out_file).is_ok() {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

pub fn generate_config_vdf(info: &GameInfo, out_dir: &Path) -> std::io::Result<()> {
    let mut content = String::from("\"depots\"\n{\n");
    for d in &info.depots {
        content.push_str(&format!(
            "    \"{}\"\n    {{\n        \"DecryptionKey\" \"{}\"\n    }}\n",
            d.id, d.sha
        ));
    }
    content.push('}');
    std::fs::write(out_dir.join("config.vdf"), content)
}

#[tauri::command]
pub fn get_download_path(game_name: String) -> String {
    let config = crate::config::load_config();
    let base = if config.download_dir.is_empty() {
        crate::config::get_default_download_dir()
    } else {
        config.download_dir
    };
    std::path::Path::new(&base)
        .join(&game_name)
        .to_string_lossy()
        .to_string()
}

fn has_local_manifest(depot_dir: &Path, depot_id: u32) -> bool {
    if let Ok(entries) = std::fs::read_dir(depot_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            if name
                .to_string_lossy()
                .starts_with(&format!("{}_", depot_id))
            {
                return true;
            }
        }
    }
    false
}

fn collect_prepared_depot_ids_in_order(game_info: &GameInfo, depot_dir: &Path) -> Vec<u32> {
    let mut ordered_ids = Vec::new();
    for d in &game_info.depots {
        if has_local_manifest(depot_dir, d.id) {
            if !ordered_ids.contains(&d.id) {
                ordered_ids.push(d.id);
            }
        }
    }
    for &id in &game_info.dlc_depots {
        if has_local_manifest(depot_dir, id) {
            if !ordered_ids.contains(&id) {
                ordered_ids.push(id);
            }
        }
    }
    ordered_ids
}

#[tauri::command]
pub async fn prepare_download(
    app_handle: AppHandle,
    game_info: GameInfo,
    use_local: bool,
) -> Result<Vec<u32>, String> {
    let depot_dir = depot_path(&app_handle);
    std::fs::create_dir_all(&depot_dir).map_err(|e| format!("创建 depot 目录失败: {}", e))?;

    if !use_local {
        CANCEL_MANIFEST.store(false, Ordering::Relaxed);

        let _ = app_handle.emit(
            "manifest-progress",
            serde_json::json!({
                "current": 0,
                "total": 0,
                "completed": false,
            }),
        );

        let manifests = fetch_latest_manifests(&game_info, &app_handle).await;
        let total = manifests.len();
        if total == 0 {
            return Err("无法获取任何清单，请检查网络或稍后再试".into());
        }

        let _ = app_handle.emit(
            "manifest-progress",
            serde_json::json!({
                "current": 0,
                "total": total,
                "completed": false,
            }),
        );

        let mut processed = 0;
        for (did, mid) in &manifests {
            if CANCEL_MANIFEST.load(Ordering::Relaxed) {
                return Err("清单获取已取消".into());
            }
            let rc = fetch_request_code(*mid).await;
            let success = match rc {
                Some(code) => download_manifest(*did, *mid, &code, &depot_dir, &app_handle).await,
                None => false,
            };
            if CANCEL_MANIFEST.load(Ordering::Relaxed) {
                let _ = std::fs::remove_file(
                    depot_dir.join(format!("{}_{}.manifest", did, mid)),
                );
                return Err("清单获取已取消".into());
            }
            if success || has_local_manifest(&depot_dir, *did) {
                let _ = app_handle.emit("manifest-ready", *did);
            }
            processed += 1;
            let _ = app_handle.emit(
                "manifest-progress",
                serde_json::json!({
                    "current": processed,
                    "total": total,
                    "completed": processed >= total,
                }),
            );
        }
    } else {
        let prepared_ids = collect_prepared_depot_ids_in_order(&game_info, &depot_dir);
        let total = prepared_ids.len().max(1);
        for did in &prepared_ids {
            let _ = app_handle.emit("manifest-ready", *did);
        }
        let _ = app_handle.emit(
            "manifest-progress",
            serde_json::json!({
                "current": total,
                "total": total,
                "completed": true,
            }),
        );
    }

    if !depot_dir.join("config.vdf").exists() {
        generate_config_vdf(&game_info, &depot_dir)
            .map_err(|e| format!("生成 config.vdf 失败: {}", e))?;
    }

    let prepared_ids = collect_prepared_depot_ids_in_order(&game_info, &depot_dir);
    if prepared_ids.is_empty() {
        return Err("准备失败：depot 目录中没有 manifest 文件".into());
    }

    Ok(prepared_ids)
}
