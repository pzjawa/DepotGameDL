import type { Update } from "@tauri-apps/plugin-updater";
import { invoke } from "@tauri-apps/api/core";
import { relaunch } from "@tauri-apps/plugin-process";
import { open as shellOpen } from "@tauri-apps/plugin-shell";
import { check } from "@tauri-apps/plugin-updater";
import { ref } from "vue";

type CheckResult = "update" | "latest" | "error" | null;

interface UpdateMeta {
	version: string
	notes: string
}

interface AppMeta {
	repository: string
}

const checking = ref(false);
const downloading = ref(false);
const downloadProgress = ref(0);
const updateDialogOpen = ref(false);
const failDialogOpen = ref(false);
const updateInfo = ref<UpdateMeta | null>(null);
const currentVersion = ref("");

let pendingUpdate: Update | null = null;
let repository = "";

// 「本次启动不再提醒」：会话级标记，重启后失效
let dismissedThisSession = false;
// 「跳过这个版本」：持久化到 localStorage，同版本不再提醒
const SKIP_VERSION_KEY = "updater.skipped_version";

function getSkippedVersion(): string {
	try {
		return localStorage.getItem(SKIP_VERSION_KEY) || "";
	} catch {
		return "";
	}
}

async function ensureAppMeta() {
	if (!currentVersion.value) {
		const meta = await invoke<AppMeta & { version: string }>("get_app_meta");
		currentVersion.value = meta.version.replace(/^v/i, "");
		repository = meta.repository;
	}
}

export function useUpdater() {
	/**
	 * 检查更新（tauri-plugin-updater）。
	 * silent=true（启动自动检查）：静默返回；受「跳过这个版本」「本次启动不再提醒」两个偏好限制；
	 * silent=false（手动检查）：始终展示结果弹窗，不受偏好限制。
	 */
	async function checkUpdate(silent = false): Promise<CheckResult> {
		if (checking.value) return null;
		checking.value = true;
		try {
			const update = await check();
			if (!update) return "latest";

			if (silent && (dismissedThisSession || update.version === getSkippedVersion())) {
				return "latest";
			}

			pendingUpdate = update;
			updateInfo.value = { version: update.version, notes: update.body || "" };
			await ensureAppMeta();
			updateDialogOpen.value = true;
			return "update";
		} catch {
			await ensureAppMeta().catch(() => {});
			if (!silent) failDialogOpen.value = true;
			return "error";
		} finally {
			checking.value = false;
		}
	}

	/** 「立即更新」：下载 + 验签 + 静默安装，完成后重启应用 */
	async function installUpdate() {
		if (!pendingUpdate || downloading.value) return;
		downloading.value = true;
		downloadProgress.value = 0;
		try {
			let total = 0;
			let received = 0;
			await pendingUpdate.downloadAndInstall((event) => {
				switch (event.event) {
					case "Started":
						total = event.data.contentLength ?? 0;
						break;
					case "Progress":
						received += event.data.chunkLength;
						if (total > 0) {
							downloadProgress.value = Math.min(100, Math.round((received / total) * 100));
						}
						break;
					case "Finished":
						downloadProgress.value = 100;
						break;
				}
			});
			await relaunch();
		} catch {
			failDialogOpen.value = true;
			updateDialogOpen.value = false;
			downloading.value = false;
		}
	}

	/** 「跳过这个版本」：记住该版本号，之后自动检查不再提示 */
	function skipVersion() {
		if (updateInfo.value) {
			try {
				localStorage.setItem(SKIP_VERSION_KEY, updateInfo.value.version);
			} catch {}
		}
		pendingUpdate = null;
		updateDialogOpen.value = false;
	}

	/** 「本次启动不再提醒」：仅本次运行期间不再自动提示，重启后恢复 */
	function remindLaterSession() {
		dismissedThisSession = true;
		pendingUpdate = null;
		updateDialogOpen.value = false;
	}

	/** 手动兜底：前往仓库 Releases 页面 */
	async function goManualUpdate() {
		failDialogOpen.value = false;
		try {
			await ensureAppMeta();
		} catch {}
		if (repository) {
			shellOpen(`${repository.replace(/\/+$/, "")}/releases`).catch(() => {});
		}
	}

	return {
		checking,
		downloading,
		downloadProgress,
		updateDialogOpen,
		failDialogOpen,
		updateInfo,
		currentVersion,
		checkUpdate,
		installUpdate,
		skipVersion,
		remindLaterSession,
		goManualUpdate
	};
}
