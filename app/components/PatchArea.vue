<template>
	<div class="grid h-full min-h-0 grid-rows-[1fr_auto] gap-3">
		<GameCover :app-id="coverAppId" />

		<div class="grid gap-3 rounded-lg border border-(--surface-border) p-3">
			<div class="grid grid-cols-[1fr_auto] items-center gap-2">
				<span class="text-sm font-semibold text-neutral-700 dark:text-neutral-200">{{ t('patchArea.onlinePatch') }}</span>
				<div class="grid auto-cols-max grid-flow-col items-center gap-2">
					<div class="grid auto-cols-max grid-flow-col gap-1 rounded-md bg-neutral-100 p-1 dark:bg-neutral-800">
						<UButton
							size="sm"
							:color="patchMode === 'local' ? 'primary' : 'neutral'"
							:variant="patchMode === 'local' ? 'solid' : 'ghost'"
							@click="patchMode = 'local'"
						>
							{{ t('patchArea.localPatchTab') }}
						</UButton>
						<UButton
							size="sm"
							:color="patchMode === 'online' ? 'primary' : 'neutral'"
							:variant="patchMode === 'online' ? 'solid' : 'ghost'"
							@click="patchMode = 'online'"
						>
							{{ t('patchArea.onlinePatchTab') }}
						</UButton>
					</div>
					<UButton
						square
						size="sm"
						color="neutral"
						variant="outline"
						icon="i-fluent:link-20-regular"
						:aria-label="patchMode === 'local' ? 'FreeTP Website' : 'Online-Fix Website'"
						@click="openPatchSite"
					/>
					<UButton
						square
						size="sm"
						color="neutral"
						variant="outline"
						icon="i-fluent:question-circle-20-regular"
						aria-label="Instructions for Use"
						@click="helpOpen = true"
					/>
				</div>
			</div>

			<div v-if="patchMode === 'local'" class="grid gap-2 rounded-lg border border-(--surface-border) p-3">
				<div class="grid grid-cols-[1fr_auto] items-center gap-2">
					<UInput
						v-model="steamLink"
						:placeholder="t('patchArea.steamLinkPlaceholder')"
						class="w-full transition-opacity duration-300"
						:class="linkDimmed ? 'opacity-50' : 'opacity-100'"
						@keyup.enter="onLinkSubmit"
						@focusin="onLinkFocusIn"
						@focusout="onLinkFocusOut"
						@update:model-value="onLinkInput"
					/>
					<UButton
						square
						size="sm"
						color="neutral"
						variant="outline"
						icon="i-fluent:arrow-enter-left-20-regular"
						aria-label="Submit"
						@click="onLinkButtonClick"
					/>
				</div>
				<div class="grid grid-cols-[1fr_auto] items-center gap-2">
					<UInput v-model="localGameDir" :placeholder="t('patchArea.gameDirPlaceholder')" class="w-full" />
					<UButton
						size="sm"
						color="neutral"
						variant="outline"
						icon="i-fluent:folder-20-regular"
						@click="selectLocalGameDir"
					/>
				</div>
				<div class="grid grid-cols-2 gap-2">
					<UButton color="primary" @click="addPatch">
						{{ t('patchArea.addPatch') }}
					</UButton>
					<UButton color="neutral" variant="outline" @click="removePatch">
						{{ t('patchArea.removePatch') }}
					</UButton>
				</div>
			</div>

			<div v-else class="grid gap-2 rounded-lg border border-(--surface-border) p-3">
				<div class="grid grid-cols-[1fr_auto] items-center gap-2">
					<UInput v-model="onlinePatchPath" :placeholder="t('patchArea.patchFolderPlaceholder')" class="w-full" readonly />
					<UButton
						size="sm"
						color="neutral"
						variant="outline"
						icon="i-fluent:folder-20-regular"
						@click="selectOnlinePatchDir"
					/>
				</div>
				<div class="grid grid-cols-[1fr_auto] items-center gap-2">
					<UInput v-model="onlineGameDir" :placeholder="t('patchArea.gameDirPlaceholder')" class="w-full" readonly />
					<UButton
						size="sm"
						color="neutral"
						variant="outline"
						icon="i-fluent:folder-20-regular"
						@click="selectOnlineGameDir"
					/>
				</div>
			<div class="grid grid-cols-2 gap-2">
				<UButton color="primary" @click="addOnlinePatch">
					{{ t('patchArea.addPatch') }}
				</UButton>
				<UButton color="neutral" variant="outline" :disabled="!onlinePatchApplied" @click="removeOnlinePatch">
					{{ t('patchArea.removePatch') }}
				</UButton>
			</div>
			</div>

		<UModal v-model:open="helpOpen" :title="t('patchArea.helpTitle')">
			<template #body>
				<div class="grid gap-2">
					<div
						class="rounded-lg border border-(--surface-border) p-3 text-sm leading-relaxed text-neutral-700 dark:text-neutral-200"
					>
						{{ t('patchArea.helpDenuvo') }}
					</div>
					<div
						class="rounded-lg border border-(--surface-border) p-3 text-sm leading-relaxed text-neutral-700 dark:text-neutral-200"
					>
						{{ t('patchArea.helpSteamClient') }}
					</div>
				</div>
			</template>
		</UModal>
		</div>
	</div>
</template>

<script setup lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { open } from "@tauri-apps/plugin-dialog";
	import { open as openUrl } from "@tauri-apps/plugin-shell";
	import { computed, ref } from "vue";
	import { t } from "~/locales";

	const { showToast, currentInfo, gameName } = useDepotGameDL();

	const patchMode = ref<"local" | "online">("local");

	const localGameDir = ref("");
	const steamLink = ref("");
	const localAppId = ref("");
	const onlinePatchPath = ref("");
	const onlineGameDir = ref("");
	const onlinePatchApplied = ref(false);
	const helpOpen = ref(false);

	const coverAppId = computed(() => (gameName.value ? currentInfo.value?.appid || "" : ""));

	const linkActive = ref(false);
	const linkDimmed = computed(() => !linkActive.value && steamLink.value.trim() !== "");

	function onLinkFocusIn() {
		linkActive.value = true;
	}

	function onLinkFocusOut() {
		linkActive.value = false;
	}

	async function selectLocalGameDir() {
		const selected = await open({ directory: true, multiple: false });
		if (typeof selected === "string") {
			localGameDir.value = selected;
		}
	}

	const APP_ID_RE = /\/app\/(\d+)/i;
	function extractAppId(el?: HTMLInputElement | null) {
		const input = steamLink.value.trim();
		if (/^\d+$/.test(input)) {
			localAppId.value = input;
		} else {
			const m = input.match(APP_ID_RE);
			if (m && m[1]) {
				localAppId.value = m[1];
			} else {
				localAppId.value = "";
				showToast(t('patchArea.toastInvalidContent'));
			}
		}

		linkActive.value = false;
		el?.blur();
	}

	function onLinkSubmit($event: KeyboardEvent) {
		extractAppId($event.currentTarget as HTMLInputElement | null);
	}

	function onLinkButtonClick() {
		extractAppId();
	}

	function onLinkInput() {
		linkActive.value = true;
	}

	async function addPatch() {
		if (!localGameDir.value) {
			showToast(t('patchArea.toastSelectGameDirFirst'));
			return;
		}
		try {
			await invoke("add_local_patch", {
				gameDir: localGameDir.value,
				appId: localAppId.value
			});
			showToast(t('patchArea.toastPatchAdded'));
		} catch (e) {
			showToast(String(e));
		}
	}

	async function removePatch() {
		if (!localGameDir.value) {
			showToast(t('patchArea.toastSelectGameDirFirst'));
			return;
		}
		try {
			await invoke("remove_local_patch", { gameDir: localGameDir.value });
		} catch (e) {
			showToast(String(e));
			return;
		}

		showToast(t('patchArea.toastPatchRemoved'));
	}

	async function selectOnlinePatchDir() {
		const selected = await open({ directory: true, multiple: false });
		if (typeof selected === "string") {
			onlinePatchPath.value = selected;
		}
	}

	async function selectOnlineGameDir() {
		const selected = await open({ directory: true, multiple: false });
		if (typeof selected === "string") {
			onlineGameDir.value = selected;
		}
	}

	async function addOnlinePatch() {
		if (!onlinePatchPath.value) {
			showToast(t('patchArea.toastSelectPatchFolderFirst'));
			return;
		}
		if (!onlineGameDir.value) {
			showToast(t('patchArea.toastSelectGameDirFirst'));
			return;
		}
		try {
			await invoke("add_online_patch", {
				patchDir: onlinePatchPath.value,
				gameDir: onlineGameDir.value
			});
			onlinePatchApplied.value = true;
			showToast(t('patchArea.toastPatchAdded'));
		} catch (e) {
			showToast(String(e));
		}
	}

	async function removeOnlinePatch() {
		if (!onlineGameDir.value) {
			showToast(t('patchArea.toastSelectGameDirFirst'));
			return;
		}
		try {
			await invoke("remove_online_patch", { gameDir: onlineGameDir.value });
		} catch (e) {
			showToast(String(e));
			return;
		}
		onlinePatchApplied.value = false;
		showToast(t('patchArea.toastPatchRemoved'));
	}

	function openPatchSite() {
		const url = patchMode.value === "local" ? "https://freetp.org/" : "https://online-fix.me/";
		openUrl(url).catch(() => {});
	}
</script>
