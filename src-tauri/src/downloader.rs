use crate::parser::GameInfo;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use tauri::{AppHandle, Emitter, Manager};
use tokio::io::{AsyncReadExt, BufReader};
use tokio::process::Command;

static PROCESS_PID: Mutex<Option<u32>> = Mutex::new(None);

pub(crate) fn cache_root(app_handle: &AppHandle) -> PathBuf {
    static CACHE: OnceLock<PathBuf> = OnceLock::new();
    CACHE
        .get_or_init(|| {
            let resource = app_handle.path().resource_dir().unwrap_or_default();
            if is_c_drive(&resource) {
                app_handle.path().app_config_dir().unwrap()
            } else {
                resource
            }
        })
        .clone()
}

fn is_c_drive(path: &Path) -> bool {
    path.components()
        .next()
        .map(|c| {
            c.as_os_str()
                .to_string_lossy()
                .to_lowercase()
                .starts_with("c:")
        })
        .unwrap_or(false)
}

fn kill_download_process() {
    let pid = {
        let guard = PROCESS_PID.lock().unwrap();
        guard.clone()
    };
    if let Some(pid) = pid {
        #[cfg(target_os = "windows")]
        {
            use windows_sys::Win32::Foundation::CloseHandle;
            use windows_sys::Win32::System::Threading::{
                OpenProcess, PROCESS_TERMINATE, TerminateProcess,
            };
            let (open_null, terminate_ok) = unsafe {
                let handle = OpenProcess(PROCESS_TERMINATE, 0, pid);
                if handle.is_null() {
                    (true, false)
                } else {
                    let ok = TerminateProcess(handle, 1) != 0;
                    let _ = CloseHandle(handle);
                    (false, ok)
                }
            };
            if open_null || !terminate_ok {
                let _ = std::process::Command::new("taskkill")
                    .args(&["/F", "/T", "/PID", &pid.to_string()])
                    .output();
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = std::process::Command::new("kill")
                .args(&["-9", &pid.to_string()])
                .output();
        }
    }
}

pub(crate) fn depot_path(app_handle: &AppHandle) -> PathBuf {
    cache_root(app_handle).join("depot")
}

fn depotdownloader_path(app_handle: &AppHandle) -> PathBuf {
    app_handle
        .path()
        .resource_dir()
        .unwrap()
        .join("DepotDownloader")
        .join("DepotDownloader.exe")
}

async fn stream_output<R: AsyncReadExt + Unpin>(reader: R, emitter: AppHandle) {
    let mut reader = BufReader::new(reader);
    let mut line = String::new();
    let mut byte = [0u8; 1];
    loop {
        match reader.read(&mut byte).await {
            Ok(0) => break,
            Ok(_) => {
                let ch = byte[0] as char;
                if ch == '\r' || ch == '\n' {
                    let trimmed = line.trim().to_string();
                    handle_line(&trimmed, &emitter);
                    line.clear();
                } else {
                    line.push(ch);
                }
            }
            Err(_) => break,
        }
    }
    let trimmed = line.trim().to_string();
    handle_line(&trimmed, &emitter);
}

fn handle_line(line: &str, emitter: &AppHandle) {
    if line.is_empty() {
        return;
    }
    if let Some(depot_id) = extract_depot_id_from_completed(line) {
        let _ = emitter.emit("depot-completed", depot_id);
    } else {
        let _ = emitter.emit("download-log", line);
    }
}

fn extract_depot_id_from_completed(line: &str) -> Option<u32> {
    if !line.contains("completed") || !line.contains("Depot ") {
        return None;
    }
    let after_depot = line.split("Depot ").nth(1)?;
    let digits: String = after_depot
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    digits.parse::<u32>().ok()
}

#[tauri::command]
pub async fn start_download(app_handle: AppHandle, download_path: String) -> Result<(), String> {
    kill_download_process();

    let depot_dir = depot_path(&app_handle);

    let has_manifest = std::fs::read_dir(&depot_dir)
        .map(|dir| {
            dir.filter_map(|e| e.ok())
                .any(|e| e.file_name().to_string_lossy().ends_with(".manifest"))
        })
        .unwrap_or(false);
    if !has_manifest {
        return Err("depot 目录中没有 manifest 文件，请先导入清单并选择版本".into());
    }
    if !depot_dir.join("config.vdf").exists() {
        return Err("depot 目录中没有 config.vdf 文件".into());
    }

    let exe = depotdownloader_path(&app_handle);
    if !exe.exists() {
        return Err(format!("找不到 DepotDownloader.exe: {}", exe.display()));
    }

    let working_dir = depot_dir.clone();
    let args = [
        "-l",
        "-u",
        "China",
        "--use-http",
        "-o",
        &download_path,
        "app",
        "-p",
        depot_dir.to_str().unwrap(),
    ];

    let mut cmd = Command::new(&exe);
    cmd.args(&args)
        .current_dir(working_dir)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .kill_on_drop(true);

    #[cfg(target_os = "windows")]
    cmd.creation_flags(0x08000000);

    let mut child = cmd.spawn().map_err(|e| format!("启动进程失败: {}", e))?;

    let pid = child.id().unwrap_or(0);

    {
        let mut guard = PROCESS_PID.lock().unwrap();
        *guard = Some(pid);
    }

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let handle_stdout = app_handle.clone();
    tokio::spawn(async move { stream_output(stdout, handle_stdout).await });

    let handle_stderr = app_handle.clone();
    tokio::spawn(async move { stream_output(stderr, handle_stderr).await });

    let handle_finish = app_handle.clone();
    let my_pid = pid;
    tokio::spawn(async move {
        let status = child.wait().await;
        let exit_code = status.map(|s| s.code().unwrap_or(1)).unwrap_or(1);

        {
            let mut guard = PROCESS_PID.lock().unwrap();
            if *guard == Some(my_pid) {
                *guard = None;
            }
        }

        let _ = handle_finish.emit(
            "download-finished",
            serde_json::json!({ "exit_code": exit_code }),
        );
    });

    Ok(())
}

#[tauri::command]
pub async fn pause_download() -> Result<(), String> {
    kill_download_process();
    Ok(())
}

#[tauri::command]
pub async fn clean_cache_cmd(
    app_handle: AppHandle,
    download_path: Option<String>,
) -> Result<(), String> {
    crate::manifest::CANCEL_MANIFEST.store(true, std::sync::atomic::Ordering::Relaxed);

    kill_download_process();

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    if let Some(p) = download_path {
        let dir = Path::new(p.trim());
        if dir.is_dir() {
            let _ = std::fs::remove_dir_all(dir);
        }
    }

    let depot = depot_path(&app_handle);
    if depot.exists() {
        let depot_clone = depot.clone();
        let _ = tokio::task::spawn_blocking(move || std::fs::remove_dir_all(&depot_clone)).await;
        std::fs::create_dir_all(&depot).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn save_game_info(
    app_handle: AppHandle,
    game_name: String,
    game_info: GameInfo,
    prepared_depot_ids: Vec<u32>,
    completed_depot_ids: Vec<u32>,
) -> Result<(), String> {
    let depot = depot_path(&app_handle);
    std::fs::create_dir_all(&depot).map_err(|e| e.to_string())?;
    let info_path = depot.join("game_info.json");
    let data = serde_json::json!({
        "game_name": game_name,
        "game_info": game_info,
        "prepared_depot_ids": prepared_depot_ids,
        "completed_depot_ids": completed_depot_ids,
    });
    std::fs::write(&info_path, data.to_string()).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn check_cache(app_handle: AppHandle) -> Result<serde_json::Value, String> {
    let depot = depot_path(&app_handle);
    let mut has_cache = false;
    let mut game_name = String::new();
    let mut game_info: Option<GameInfo> = None;
    let mut prepared_depot_ids: Vec<u32> = Vec::new();
    let mut completed_depot_ids: Vec<u32> = Vec::new();

    if depot.exists() {
        let has_vdf = depot.join("config.vdf").exists();
        let has_manifest = std::fs::read_dir(&depot)
            .map(|dir| {
                dir.filter_map(|e| e.ok())
                    .any(|e| e.file_name().to_string_lossy().ends_with(".manifest"))
            })
            .unwrap_or(false);
        if has_vdf && has_manifest {
            has_cache = true;
            let info_path = depot.join("game_info.json");
            if let Ok(content) = std::fs::read_to_string(&info_path) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    game_name = json
                        .get("game_name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    if let Some(gi) = json.get("game_info") {
                        game_info = serde_json::from_value::<GameInfo>(gi.clone()).ok();
                    }
                    if let Some(ids) = json.get("prepared_depot_ids").and_then(|v| v.as_array()) {
                        prepared_depot_ids = ids
                            .iter()
                            .filter_map(|v| v.as_u64().map(|n| n as u32))
                            .collect();
                    }
                    if let Some(ids) = json.get("completed_depot_ids").and_then(|v| v.as_array()) {
                        completed_depot_ids = ids
                            .iter()
                            .filter_map(|v| v.as_u64().map(|n| n as u32))
                            .collect();
                    }
                }
            }
        }
    }

    Ok(serde_json::json!({
        "has_cache": has_cache,
        "game_name": game_name,
        "game_info": game_info,
        "prepared_depot_ids": prepared_depot_ids,
        "completed_depot_ids": completed_depot_ids,
    }))
}

#[tauri::command]
pub async fn save_progress(
    app_handle: AppHandle,
    progress: u32,
    progress_text: String,
) -> Result<(), String> {
    let depot = depot_path(&app_handle);
    std::fs::create_dir_all(&depot).map_err(|e| e.to_string())?;
    let progress_path = depot.join("progress.json");
    let data = serde_json::json!({
        "progress": progress,
        "progress_text": progress_text,
    });
    std::fs::write(&progress_path, data.to_string()).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn load_progress(app_handle: AppHandle) -> Result<serde_json::Value, String> {
    let depot = depot_path(&app_handle);
    let progress_path = depot.join("progress.json");
    if progress_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&progress_path) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                return Ok(json);
            }
        }
    }
    Ok(serde_json::json!({
        "progress": 0,
        "progress_text": "",
    }))
}
