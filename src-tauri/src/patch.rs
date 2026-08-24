use crate::downloader::cache_root;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};

fn freetp_path(app_handle: &AppHandle) -> PathBuf {
    app_handle
        .path()
        .resource_dir()
        .unwrap()
        .join("FreeTP_Patch")
}

fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let target = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_all(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), &target)?;
        }
    }
    Ok(())
}

#[tauri::command]
pub fn add_local_patch(
    app_handle: AppHandle,
    game_dir: String,
    app_id: String,
) -> Result<String, String> {
    let src = freetp_path(&app_handle);
    if !src.exists() {
        return Err(format!("找不到 FreeTP_Patch 资源目录: {}", src.display()));
    }
    let dst = Path::new(&game_dir);
    if !dst.exists() || !dst.is_dir() {
        return Err(format!("游戏目录不存在: {}", game_dir));
    }

    copy_dir_all(&src, dst).map_err(|e| format!("复制补丁失败: {}", e))?;

    if !app_id.is_empty() {
        let ini = dst.join("SteamFix.ini");
        if ini.exists() {
            let content =
                fs::read_to_string(&ini).map_err(|e| format!("读取 SteamFix.ini 失败: {}", e))?;
            let re = regex::Regex::new(r"(?i)RealAppId=\d+")
                .map_err(|e| format!("正则编译失败: {}", e))?;
            let new_content = re.replace(&content, format!("RealAppId={}", app_id));
            fs::write(&ini, new_content.as_ref())
                .map_err(|e| format!("写入 SteamFix.ini 失败: {}", e))?;
        }
    }

    Ok("ok".into())
}

const PATCH_ITEMS: &[&str] = &[
    "FreeTP",
    "EpicFix.ini",
    "EpicFix64.dll",
    "SteamFix.ini",
    "SteamFix64.dll",
    "winmm.dll",
    "winmm.txt",
];

#[tauri::command]
pub fn remove_local_patch(game_dir: String) -> Result<String, String> {
    let base = Path::new(&game_dir);
    if !base.exists() || !base.is_dir() {
        return Err(format!("游戏目录不存在: {}", game_dir));
    }
    let mut removed = 0u32;
    for item in PATCH_ITEMS {
        let target = base.join(item);
        if target.is_dir() {
            if let Ok(()) = fs::remove_dir_all(&target) {
                removed += 1;
            }
        } else if target.exists() {
            if let Ok(()) = fs::remove_file(&target) {
                removed += 1;
            }
        }
    }
    Ok(removed.to_string())
}

const DLL_BACKUP_NAME: &str = "steam_api64.dll";

fn safe_key(game_dir: &str) -> String {
    game_dir
        .replace(':', "_")
        .replace('\\', "_")
        .replace('/', "_")
}

fn patch_cache_dir(app_handle: &AppHandle, game_dir: &str) -> PathBuf {
    cache_root(app_handle)
        .join("patches")
        .join(safe_key(game_dir))
}

fn collect_files(base: &Path, dir: &Path, out: &mut Vec<String>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_files(base, &path, out)?;
        } else {
            let rel = path
                .strip_prefix(base)
                .unwrap_or(&path)
                .to_string_lossy()
                .to_string();
            out.push(rel);
        }
    }
    Ok(())
}

fn remove_patched_file(base: &Path, rel: &str) -> io::Result<()> {
    let target = base.join(rel);
    if !target.starts_with(base) {
        return Ok(());
    }
    if target.is_file() {
        fs::remove_file(&target)?;
    }
    Ok(())
}

#[tauri::command]
pub fn add_online_patch(
    app_handle: AppHandle,
    patch_dir: String,
    game_dir: String,
) -> Result<String, String> {
    let src = Path::new(&patch_dir);
    let dst = Path::new(&game_dir);
    if !src.exists() || !src.is_dir() {
        return Err(format!("补丁目录不存在: {}", patch_dir));
    }
    if !dst.exists() || !dst.is_dir() {
        return Err(format!("游戏目录不存在: {}", game_dir));
    }

    let cache = patch_cache_dir(&app_handle, &game_dir);
    fs::create_dir_all(&cache).map_err(|e| format!("创建缓存目录失败: {}", e))?;

    let orig_dll = dst.join(DLL_BACKUP_NAME);
    if orig_dll.exists() {
        let backup = cache.join(DLL_BACKUP_NAME);
        if !backup.exists() {
            fs::copy(&orig_dll, &backup)
                .map_err(|e| format!("备份 steam_api64.dll 失败: {}", e))?;
        }
    }

    let mut copied: Vec<String> = Vec::new();
    collect_files(src, src, &mut copied).map_err(|e| format!("扫描补丁目录失败: {}", e))?;
    for rel in &copied {
        let from = src.join(rel);
        let to = dst.join(rel);
        if !to.starts_with(dst) {
            continue;
        }
        if let Some(parent) = to.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("创建目标目录失败: {}", e))?;
        }
        fs::copy(&from, &to).map_err(|e| format!("复制 {} 失败: {}", rel, e))?;
    }

    let manifest = serde_json::to_string(&copied).map_err(|e| format!("序列化失败: {}", e))?;
    fs::write(cache.join("copied_files.json"), manifest)
        .map_err(|e| format!("写入清单失败: {}", e))?;

    Ok(format!("{}", copied.len()))
}

#[tauri::command]
pub fn remove_online_patch(app_handle: AppHandle, game_dir: String) -> Result<String, String> {
    let dst = Path::new(&game_dir);
    if !dst.exists() || !dst.is_dir() {
        return Err(format!("游戏目录不存在: {}", game_dir));
    }

    let cache = patch_cache_dir(&app_handle, &game_dir);

    let manifest_path = cache.join("copied_files.json");
    if manifest_path.exists() {
        let raw = fs::read_to_string(&manifest_path).map_err(|e| format!("读取清单失败: {}", e))?;
        let copied: Vec<String> =
            serde_json::from_str(&raw).map_err(|e| format!("解析清单失败: {}", e))?;
        for rel in &copied {
            let _ = remove_patched_file(dst, rel);
        }
    }

    let backup = cache.join(DLL_BACKUP_NAME);
    if backup.is_file() {
        let orig_dll = dst.join(DLL_BACKUP_NAME);
        if orig_dll.is_file() {
            let _ = fs::remove_file(&orig_dll);
        }
        fs::copy(&backup, &orig_dll).map_err(|e| format!("恢复 steam_api64.dll 失败: {}", e))?;
    }

    if let Ok(entries) = fs::read_dir(&cache) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_file() {
                let _ = fs::remove_file(&p);
            }
        }
    }

    Ok("ok".into())
}
