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

let dismissedThisSession = false;
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

	function skipVersion() {
		if (updateInfo.value) {
			try {
				localStorage.setItem(SKIP_VERSION_KEY, updateInfo.value.version);
			} catch {}
		}
		pendingUpdate = null;
		updateDialogOpen.value = false;
	}

	function remindLaterSession() {
		dismissedThisSession = true;
		pendingUpdate = null;
		updateDialogOpen.value = false;
	}

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
