pub struct Strings {
	pub process_start_failed: &'static str,
	pub no_manifest_in_depot: &'static str,
	pub no_config_vdf: &'static str,
	pub depotdownloader_not_found: &'static str,
	pub create_depot_dir_failed: &'static str,
	pub fetch_all_manifests_failed: &'static str,
	pub manifest_cancelled: &'static str,
	pub gen_config_vdf_failed: &'static str,
	pub prepare_no_manifest: &'static str,
	pub freetp_resource_dir_not_found: &'static str,
	pub game_dir_not_exists: &'static str,
	pub copy_patch_failed: &'static str,
	pub read_steamfix_ini_failed: &'static str,
	pub regex_compile_failed: &'static str,
	pub write_steamfix_ini_failed: &'static str,
	pub patch_dir_not_exists: &'static str,
	pub create_cache_dir_failed: &'static str,
	pub backup_steam_api_dll_failed: &'static str,
	pub scan_patch_dir_failed: &'static str,
	pub create_target_dir_failed: &'static str,
	pub copy_file_failed: &'static str,
	pub serialize_manifest_failed: &'static str,
	pub write_manifest_failed: &'static str,
	pub read_manifest_failed: &'static str,
	pub parse_manifest_failed: &'static str,
	pub restore_steam_api_dll_failed: &'static str,
	pub read_file_failed: &'static str,
	pub open_zip_failed: &'static str,
	pub parse_zip_failed: &'static str,
	pub unsupported_format: &'static str,
	pub lua_not_found: &'static str,
	pub parse_manifest_content_failed: &'static str,
	pub manifest_ready: &'static str,
}

pub const ZH: Strings = Strings {
	process_start_failed: "启动进程失败: {}",
	no_manifest_in_depot: "depot 目录中没有 manifest 文件，请先导入清单并选择版本",
	no_config_vdf: "depot 目录中没有 config.vdf 文件",
	depotdownloader_not_found: "找不到 DepotDownloader.exe: {}",
	create_depot_dir_failed: "创建 depot 目录失败: {}",
	fetch_all_manifests_failed: "无法获取任何清单，请检查网络或稍后再试",
	manifest_cancelled: "清单获取已取消",
	gen_config_vdf_failed: "生成 config.vdf 失败: {}",
	prepare_no_manifest: "准备失败：depot 目录中没有 manifest 文件",
	freetp_resource_dir_not_found: "找不到 FreeTP_Patch 资源目录: {}",
	game_dir_not_exists: "游戏目录不存在: {}",
	copy_patch_failed: "复制补丁失败: {}",
	read_steamfix_ini_failed: "读取 SteamFix.ini 失败: {}",
	regex_compile_failed: "正则编译失败: {}",
	write_steamfix_ini_failed: "写入 SteamFix.ini 失败: {}",
	patch_dir_not_exists: "补丁目录不存在: {}",
	create_cache_dir_failed: "创建缓存目录失败: {}",
	backup_steam_api_dll_failed: "备份 steam_api64.dll 失败: {}",
	scan_patch_dir_failed: "扫描补丁目录失败: {}",
	create_target_dir_failed: "创建目标目录失败: {}",
	copy_file_failed: "复制 {} 失败: {}",
	serialize_manifest_failed: "序列化失败: {}",
	write_manifest_failed: "写入清单失败: {}",
	read_manifest_failed: "读取清单失败: {}",
	parse_manifest_failed: "解析清单失败: {}",
	restore_steam_api_dll_failed: "恢复 steam_api64.dll 失败: {}",
	read_file_failed: "无法读取文件: {}",
	open_zip_failed: "无法打开 zip: {}",
	parse_zip_failed: "无法解析 zip: {}",
	unsupported_format: "不支持的格式",
	lua_not_found: "未找到 lua 文件",
	parse_manifest_content_failed: "无法解析清单内容",
	manifest_ready: "清单已就绪，AppID: {:?}",
};

pub const EN: Strings = Strings {
	process_start_failed: "Failed to start process: {}",
	no_manifest_in_depot: "No manifest file in depot directory. Please import a manifest and select a version first",
	no_config_vdf: "No config.vdf file in depot directory",
	depotdownloader_not_found: "Cannot find DepotDownloader.exe: {}",
	create_depot_dir_failed: "Failed to create depot directory: {}",
	fetch_all_manifests_failed: "Failed to fetch any manifests. Please check your network or try again later",
	manifest_cancelled: "Manifest fetch cancelled",
	gen_config_vdf_failed: "Failed to generate config.vdf: {}",
	prepare_no_manifest: "Preparation failed: no manifest file in depot directory",
	freetp_resource_dir_not_found: "FreeTP_Patch resource directory not found: {}",
	game_dir_not_exists: "Game directory does not exist: {}",
	copy_patch_failed: "Failed to copy patch: {}",
	read_steamfix_ini_failed: "Failed to read SteamFix.ini: {}",
	regex_compile_failed: "Regex compilation failed: {}",
	write_steamfix_ini_failed: "Failed to write SteamFix.ini: {}",
	patch_dir_not_exists: "Patch directory does not exist: {}",
	create_cache_dir_failed: "Failed to create cache directory: {}",
	backup_steam_api_dll_failed: "Failed to backup steam_api64.dll: {}",
	scan_patch_dir_failed: "Failed to scan patch directory: {}",
	create_target_dir_failed: "Failed to create target directory: {}",
	copy_file_failed: "Failed to copy {}: {}",
	serialize_manifest_failed: "Serialization failed: {}",
	write_manifest_failed: "Failed to write manifest: {}",
	read_manifest_failed: "Failed to read manifest: {}",
	parse_manifest_failed: "Failed to parse manifest: {}",
	restore_steam_api_dll_failed: "Failed to restore steam_api64.dll: {}",
	read_file_failed: "Failed to read file: {}",
	open_zip_failed: "Failed to open zip: {}",
	parse_zip_failed: "Failed to parse zip: {}",
	unsupported_format: "Unsupported format",
	lua_not_found: "Lua file not found",
	parse_manifest_content_failed: "Failed to parse manifest content",
	manifest_ready: "Manifest ready, AppID: {:?}",
};

#[cfg(target_os = "windows")]
pub fn system_is_zh_cn() -> bool {
	unsafe {
		use windows_sys::Win32::Globalization::GetUserDefaultLCID;
		GetUserDefaultLCID() == 0x0804
	}
}

#[cfg(not(target_os = "windows"))]
pub fn system_is_zh_cn() -> bool {
	std::env::var("LANG")
		.or_else(|_| std::env::var("LC_ALL"))
		.map(|v| v.starts_with("zh_CN"))
		.unwrap_or(false)
}

pub fn strings() -> &'static Strings {
	if system_is_zh_cn() { &ZH } else { &EN }
}
