import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import { computed, reactive, ref } from "vue";

let toast: ReturnType<typeof useToast> | null = null;

// ========== 全局状态 ==========
const progress = ref(0);
const progressText = ref("");
const progressDepotId = ref("");
const gameName = ref("");
const gameThumb = ref("");
const currentInfo = ref<any>(null);
const isDownloading = ref(false);
const downloadPath = ref("");
const localManifests = ref(false);
const downloadDir = ref("");
const defaultDownloadDir = ref("");
const completedDepots = ref(new Set<number>());
const manifestProgress = ref<{
	current: number
	total: number
	completed: boolean
} | null>(null);
const preparedDepotIds = ref<number[]>([]);
const hasProgress = ref(false);
const downloadStarted = ref(false);
const windowStyle = ref("default");
const themeMode = ref("system");
const resumeDialogOpen = ref(false);
const resumeCacheResult = ref<any>(null);
const manifestDialogOpen = ref(false);
const pendingManifest = ref<any>(null);
const aboutOpen = ref(false);
const preparingManifest = ref(false);
const patchDialogOpen = ref(false);

let progressSaveTimer: ReturnType<typeof setTimeout> | null = null;
const pausedByUser = ref(false);
let resumeMode = false;

const stepStatus = reactive<Record<string, "idle" | "ok" | "fail">>({
	step2: "idle",
	step3: "idle",
	step4: "idle"
});

function showToast(title: string) {
	if (toast) {
		toast.add({
			title,
			close: false,
			progress: false,
			duration: 3000
		});
	} else {
		console.warn("Toast 尚未初始化");
	}
}

let manifestToastId: string | number | null = null;

function clearManifestToast() {
	if (toast && manifestToastId !== null) {
		toast.remove(manifestToastId);
		manifestToastId = null;
	}
}

const PROGRESS_PCT_RE = /(\d+)%/;
const PROGRESS_SIZE_RE = /([\d.]+(?:[A-Z_]\w*)?\/[\d.]+\w*)/i;
const PROGRESS_RATE_RE = /\[[^,\]]*,\s*([^,\]]+)/;
const PROGRESS_REMAIN_RE = /<([^,\]]+)/;
const DEPOT_PREFIX_RE = /^Depot\s+/;

const THUMB_LOGO = (appid: number) => `/steam/apps/${appid}/logo.png`;

function loadGameThumb() {
	gameThumb.value = "";
	const appid = currentInfo.value?.appid;
	if (!appid) return;
	gameThumb.value = THUMB_LOGO(appid);
}

function parseProgress(text: string): { depotId: string, info: string } {
	const prefix = (text.split("|")[0] ?? "").trim();
	const depotId = prefix.replace(DEPOT_PREFIX_RE, "").split(":")[0]!.trim();
	const pct = text.match(PROGRESS_PCT_RE)?.[1];
	const size = text.match(PROGRESS_SIZE_RE)?.[1];
	const rate = text.match(PROGRESS_RATE_RE)?.[1];
	const remain = text.match(PROGRESS_REMAIN_RE)?.[1];
	const parts: string[] = [];
	if (size) parts.push(size);
	if (pct) parts.push(`${pct}%`);
	if (rate && !rate.includes("?")) parts.push(rate);
	if (remain && !remain.includes("?")) parts.push(remain);
	return { depotId, info: parts.join(" | ") };
}

function depotDisplayName(id: number | string): string {
	return depotName(id) ?? String(id);
}

// depot 的 addappid 行尾注释（Rust 解析进 DepotInfo.name），无注释返回 null
function depotName(id: number | string): string | null {
	const num = Number(id);
	if (Number.isNaN(num)) return null;
	const depot = (currentInfo.value?.depots as Array<{ id: number, name?: string | null }> | undefined)?.find(
		(d) => d.id === num
	);
	return depot?.name || null;
}

const allDepotIds = computed<number[]>(() => {
	if (!currentInfo.value) return [];
	const ids: number[] = [];
	const seen = new Set<number>();
	const add = (id: number) => {
		if (!seen.has(id)) {
			seen.add(id);
			ids.push(id);
		}
	};
	(currentInfo.value.depots || []).forEach((d: any) => add(d.id));
	(currentInfo.value.dlc_depots || []).forEach((id: number) => add(id));
	return ids;
});

function sortPreparedIds() {
	const order = allDepotIds.value;
	if (order.length > 0) {
		preparedDepotIds.value = preparedDepotIds.value.sort((a, b) => order.indexOf(a) - order.indexOf(b));
	}
}

let unlisteners: Array<() => void> = [];

let manifestCancelled = false;

function bind(event: string, handler: (event: any) => void) {
	listen(event, handler)
		.then((un) => unlisteners.push(un))
		.catch(() => {});
}

export function initDepotGameDL() {
	toast = useToast();

	unlisteners.forEach((un) => un());
	unlisteners = [];

	(async () => {
		try {
			const config: any = await invoke("load_config");
			downloadDir.value = config.download_dir || "";
			windowStyle.value = config.window_style || "default";
			themeMode.value = config.theme_mode || "system";
			updateHtmlClass();
			await ensureDefaultDownloadDir();
			if (downloadDir.value === defaultDownloadDir.value) {
				downloadDir.value = "";
			}
		} catch {}
	})();

	(async () => {
		try {
			const savedProgress: any = await invoke("load_progress");
			if (savedProgress.progress > 0) {
				progress.value = savedProgress.progress;
				progressText.value = savedProgress.progress_text || "";
				hasProgress.value = true;
			}
		} catch {}
	})();

	bind("download-log", (event: any) => {
		const text = event.payload as string;
		const parsed = parseProgress(text);
		const lineDepotId = Number(parsed.depotId);
		const alreadyDone = !Number.isNaN(lineDepotId) && completedDepots.value.has(lineDepotId);

		const match = text.match(PROGRESS_PCT_RE);
		if (match && match[1]) {
			if (alreadyDone) return;
			const newProgress = Number.parseInt(match[1]);
			progress.value = newProgress;
			progressDepotId.value = depotDisplayName(parsed.depotId);
			progressText.value = parsed.info;
			hasProgress.value = true;
			if (progressSaveTimer) clearTimeout(progressSaveTimer);
			progressSaveTimer = setTimeout(() => {
				invoke("save_progress", {
					progress: newProgress,
					progress_text: parsed.info
				}).catch(() => {});
			}, 500);
		} else if (text.includes("completed")) {
			if (alreadyDone) return;
			progress.value = 100;
			progressText.value = text;
			hasProgress.value = true;
			invoke("save_progress", { progress: 100, progress_text: text }).catch(() => {});
		}
	});

	bind("manifest-progress", (event: any) => {
		const payload = event.payload as {
			current: number
			total: number
			completed: boolean
		};
		manifestProgress.value = payload;

		if (toast) {
			const title = `获取清单 [${payload.current}/${payload.total}]`;
			if (manifestToastId === null) {
				manifestToastId = toast.add({
					title,
					close: false,
					progress: false,
					duration: 0
				}).id;
			} else {
				toast.update(manifestToastId, { title, duration: 0 });
			}
			if (payload.completed && manifestToastId !== null) {
				const id = manifestToastId;
				manifestToastId = null;
				setTimeout(() => toast?.remove(id), 1500);
			}
		}
	});

	bind("manifest-ready", (event: any) => {
		const depotId = Number(event.payload);
		if (!preparedDepotIds.value.includes(depotId)) {
			preparedDepotIds.value.push(depotId);
			sortPreparedIds();
		}
	});

	bind("depot-completed", (event: any) => {
		const depotId = Number(event.payload);
		completedDepots.value.add(depotId);
		saveGameInfoToCache();
	});

	bind("download-finished", (event: any) => {
		const code = event.payload.exit_code as number;
		if (pausedByUser.value) {
			pausedByUser.value = false;
			isDownloading.value = false;
		} else if (code === 0) {
			stepStatus.step3 = "ok";
			isDownloading.value = false;
			preparedDepotIds.value.forEach((id) => completedDepots.value.add(id));
			progress.value = 100;
			progressDepotId.value = "all";
			progressText.value = "completed";
			hasProgress.value = true;
			invoke("save_progress", {
				progress: 100,
				progress_text: "completed"
			}).catch(() => {});
			showToast("下载完毕");
			patchDialogOpen.value = true;
		} else {
			isDownloading.value = false;
		}
	});

	checkCacheOnStartup();
}

async function checkCacheOnStartup() {
	try {
		const result: any = await invoke("check_cache");
		if (result.has_cache) {
			resumeCacheResult.value = result;
			resumeDialogOpen.value = true;
		}
	} catch {
		resumeMode = false;
	}
}

async function confirmResume() {
	resumeDialogOpen.value = false;
	const result = resumeCacheResult.value;
	if (!result) return;
	resumeMode = true;
	gameName.value = result.game_name || "";
	if (result.game_info) {
		currentInfo.value = result.game_info;
		loadGameThumb();
		const appid = currentInfo.value?.appid;
		if (appid) {
			try {
				const onlineName = (await invoke("fetch_game_name_cmd", { appid })) as string | null;
				if (onlineName) gameName.value = onlineName;
			} catch {}
		}
	}
	if (gameName.value) {
		try {
			downloadPath.value = (await invoke("get_download_path", {
				gameName: gameName.value
			})) as string;
		} catch {}
	}
	if (result.prepared_depot_ids && result.prepared_depot_ids.length > 0) {
		preparedDepotIds.value = result.prepared_depot_ids;
		sortPreparedIds();
	}
	if (result.completed_depot_ids && result.completed_depot_ids.length > 0) {
		completedDepots.value = new Set<number>(result.completed_depot_ids);
	}
	if (gameName.value && currentInfo.value && preparedDepotIds.value.length > 0) {
		stepStatus.step2 = "ok";
		downloadStarted.value = true;
		showToast("继续上次下载");
		const total = preparedDepotIds.value.length;
		manifestProgress.value = {
			current: total,
			total,
			completed: true
		};
	} else {
		stepStatus.step2 = "idle";
	}
}

async function discardResume() {
	resumeDialogOpen.value = false;
	try {
		await invoke("clean_cache_cmd");
		showToast("缓存已清理");
	} catch (e) {
		showToast(`清理缓存失败: ${e}`);
	}
	resumeMode = false;
	downloadStarted.value = false;
	gameName.value = "";
	gameThumb.value = "";
	currentInfo.value = null;
	preparedDepotIds.value = [];
	manifestProgress.value = null;
	clearManifestToast();
}

function buildConfig() {
	return {
		download_dir: downloadDir.value,
		auto_retry: true,
		window_style: windowStyle.value,
		theme_mode: themeMode.value
	};
}

async function ensureDefaultDownloadDir() {
	try {
		const dir = (await invoke("get_default_download_dir")) as string;
		defaultDownloadDir.value = dir;
	} catch {}
}

async function saveDownloadDir() {
	try {
		await invoke("save_config", {
			config: {
				...buildConfig(),
				download_dir: downloadDir.value || defaultDownloadDir.value
			}
		});
	} catch {}
}

function isDarkMode() {
	if (themeMode.value === "dark") return true;
	if (themeMode.value === "light") return false;
	return window.matchMedia("(prefers-color-scheme: dark)").matches;
}

function updateHtmlClass() {
	const el = document.documentElement;
	el.classList.toggle("dark", isDarkMode());
	el.classList.toggle("default", windowStyle.value === "default");
}

async function applyWindowStyle() {
	updateHtmlClass();
	try {
		await invoke("set_window_style", {
			style: windowStyle.value,
			dark: isDarkMode()
		});
	} catch {}
}

async function setWindowStyle(style: string) {
	windowStyle.value = style;
	await applyWindowStyle();
	await saveDownloadDir();
}

async function setThemeMode(mode: string) {
	themeMode.value = mode;
	await applyWindowStyle();
	await saveDownloadDir();
}

async function selectDownloadDir() {
	const selected = await openDialog({
		directory: true,
		title: "选择下载目录",
		defaultPath: downloadDir.value || defaultDownloadDir.value || undefined
	});
	if (selected) {
		downloadDir.value = selected as string;
		await saveDownloadDir();
	}
}

async function importKey() {
	if (isDownloading.value) return;
	const selected = await openDialog({
		multiple: false,
		filters: [{ name: "清单文件", extensions: ["lua", "zip", "7z", "rar"] }]
	});
	if (!selected) return;
	const filePath = selected as string;

	manifestProgress.value = null;
	clearManifestToast();

	showToast("正在解析清单");

	try {
		const result: any = await invoke("import_lua", { filePath });
		if (!result.success) {
			showToast(result.message || "清单解析失败");
			stepStatus.step2 = "fail";
			return;
		}

		const parsedGameInfo = result.game_info;
		const parsedGameName = result.game_name || "";
		localManifests.value = result.has_local_manifests;
		currentInfo.value = parsedGameInfo;
		loadGameThumb();
		preparedDepotIds.value = [];

		if (localManifests.value) {
			pendingManifest.value = {
				gameInfo: parsedGameInfo,
				gameName: parsedGameName
			};
			manifestDialogOpen.value = true;
		} else {
			showToast("在线获取清单");
			manifestCancelled = false;
			preparingManifest.value = true;
			try {
				const preparedIds: number[] = await invoke("prepare_download", {
					gameInfo: parsedGameInfo,
					useLocal: false
				});
				preparedDepotIds.value = preparedIds;
				sortPreparedIds();
				gameName.value = parsedGameName;
				completedDepots.value = new Set<number>();
				stepStatus.step2 = "ok";
				saveGameInfoToCache();
			} catch (e) {
				if (manifestCancelled) {
					manifestCancelled = false;
					clearManifestToast();
					return;
				}
				showToast(`获取清单失败: ${e}`);
				stepStatus.step2 = "fail";
				manifestProgress.value = null;
				clearManifestToast();
			} finally {
				preparingManifest.value = false;
			}
		}
	} catch {
		showToast("清单解析失败");
		stepStatus.step2 = "fail";
	}
}

async function applyManifest(useLocal: boolean) {
	const p = pendingManifest.value;
	if (!p) return;
	manifestCancelled = false;
	preparingManifest.value = true;
	try {
		const preparedIds: number[] = await invoke("prepare_download", {
			gameInfo: p.gameInfo,
			useLocal
		});
		preparedDepotIds.value = preparedIds;
		sortPreparedIds();
		gameName.value = p.gameName;
		completedDepots.value = new Set<number>();
		stepStatus.step2 = "ok";
		saveGameInfoToCache();
	} catch (e) {
		if (manifestCancelled) {
			manifestCancelled = false;
			clearManifestToast();
			return;
		}
		showToast(useLocal ? `本地清单无效: ${e}` : `失败: ${e}`);
		stepStatus.step2 = "fail";
		manifestProgress.value = null;
		clearManifestToast();
	} finally {
		preparingManifest.value = false;
	}
}

async function chooseOnlineManifest() {
	manifestDialogOpen.value = false;
	await applyManifest(false);
}

async function chooseLocalManifest() {
	manifestDialogOpen.value = false;
	await applyManifest(true);
}

async function saveGameInfoToCache() {
	if (!currentInfo.value || !gameName.value) return;
	try {
		await invoke("save_game_info", {
			gameName: gameName.value,
			gameInfo: currentInfo.value,
			preparedDepotIds: preparedDepotIds.value,
			completedDepotIds: [...completedDepots.value]
		});
	} catch {
	}
}

async function startDownload() {
	if (isDownloading.value) return;

	if (resumeMode && !currentInfo.value) {
		showToast("缓存不完整，请重新导入清单");
		return;
	}

	if (!gameName.value || !currentInfo.value) {
		showToast("请先导入清单文件");
		return;
	}

	if (preparedDepotIds.value.length > 0 && preparedDepotIds.value.every((id) => completedDepots.value.has(id))) {
		showToast("下载已完成");
		return;
	}

	pausedByUser.value = false;
	isDownloading.value = true;
	if (completedDepots.value.size === 0 && !hasProgress.value) {
		progress.value = 0;
		progressText.value = "";
	}

	try {
		downloadPath.value = (await invoke("get_download_path", {
			gameName: gameName.value
		})) as string;
		await invoke("start_download", { downloadPath: downloadPath.value });
		downloadStarted.value = true;
	} catch (e) {
		isDownloading.value = false;
		showToast(`启动下载失败: ${e}`);
	}
}

async function pauseDownload() {
	if (!isDownloading.value) return;
	pausedByUser.value = true;
	isDownloading.value = false;
	try {
		await invoke("pause_download");
	} catch (e) {
		pausedByUser.value = false;
		showToast(`${e}`);
	}
}

async function confirmAddPatch() {
	patchDialogOpen.value = false;
	const gameDir = downloadPath.value;
	const appId = currentInfo.value?.appid;
	if (!gameDir || !appId) {
		showToast("无法添加补丁");
		return;
	}
	try {
		await invoke("add_local_patch", { gameDir, appId: String(appId) });
		showToast("补丁已添加");
	} catch (e) {
		showToast(String(e));
	}
}

async function cleanCache() {
	if (isDownloading.value) {
		pausedByUser.value = true;
		await pauseDownload();
	}
	manifestCancelled = true;
	preparingManifest.value = false;
	const allDone = preparedDepotIds.value.length > 0
		&& preparedDepotIds.value.every((id) => completedDepots.value.has(id));
	const pathToClean = allDone ? "" : downloadPath.value;
	resumeMode = false;
	downloadStarted.value = false;
	gameName.value = "";
	gameThumb.value = "";
	currentInfo.value = null;
	downloadPath.value = "";
	completedDepots.value = new Set<number>();
	preparedDepotIds.value = [];
	manifestProgress.value = null;
	clearManifestToast();
	progress.value = 0;
	progressText.value = "";
	hasProgress.value = false;
	invoke("save_progress", { progress: 0, progress_text: "" }).catch(() => {});
	stepStatus.step2 = "idle";
	stepStatus.step4 = "ok";

	try {
		await invoke("clean_cache_cmd", { downloadPath: pathToClean });
		showToast("缓存已清理");
	} catch (e) {
		showToast(`清理缓存失败: ${e}`);
	}
}

export function useDepotGameDL() {
	return {
		progress,
		progressText,
		progressDepotId,
		gameName,
		gameThumb,
		currentInfo,
		completedDepots,
		manifestProgress,
		preparedDepotIds,
		hasProgress,
		stepStatus,
		isDownloading,
		downloadDir,
		defaultDownloadDir,
		windowStyle,
		themeMode,
		resumeDialogOpen,
		confirmResume,
		discardResume,
		manifestDialogOpen,
		chooseOnlineManifest,
		chooseLocalManifest,
		aboutOpen,
		preparingManifest,
		patchDialogOpen,
		confirmAddPatch,
		selectDownloadDir,
		saveDownloadDir,
		setWindowStyle,
		setThemeMode,
		importKey,
		startDownload,
		pauseDownload,
		cleanCache,
		downloadStarted,
		showToast,
		depotDisplayName,
		depotName
	};
}
