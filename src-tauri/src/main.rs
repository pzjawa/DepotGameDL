#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use depotgamedl_lib::config::{apply_style, load_config};
use depotgamedl_lib::{config, downloader, manifest, parser, patch};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(target_os = "windows")]
            {
                let config = load_config();
                let dark = config.theme_mode == "dark";
                apply_style(app.handle(), &config.window_style, dark);
            }
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            config::load_config,
            config::save_config,
            config::get_app_meta,
            config::set_window_style,
            config::get_default_download_dir,
            parser::import_lua,
            parser::fetch_game_name_cmd,
            manifest::get_download_path,
            manifest::prepare_download,
            downloader::check_cache,
            downloader::save_game_info,
            downloader::start_download,
            downloader::pause_download,
            downloader::clean_cache_cmd,
            downloader::save_progress,
            downloader::load_progress,
            patch::add_local_patch,
            patch::remove_local_patch,
            patch::add_online_patch,
            patch::remove_online_patch,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
