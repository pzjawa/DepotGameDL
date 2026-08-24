<template>
	<div class="grid h-full grid-rows-[auto_auto_1fr] overflow-hidden p-4 font-sans">
		<div
			class="mb-4 grid grid-flow-col items-center justify-between rounded-lg border border-(--surface-border) bg-(--surface) px-3 py-2"
		>
			<div class="grid auto-cols-max grid-flow-col items-center gap-2">
				<UButton color="neutral" @click="importKey">
					导入清单
				</UButton>
				<UButton color="neutral" @click="cleanCache">
					清理缓存
				</UButton>
			</div>
			<div class="grid auto-cols-max grid-flow-col items-center gap-2">
				<UInput
					:model-value="pathInputValue"
					:placeholder="pathPlaceholder"
					:disabled="downloadStarted"
					class="w-64 transition-opacity duration-300"
					:class="downloadStarted ? 'opacity-50' : 'opacity-100'"
					@update:model-value="onPathInput"
					@focusin="pathFocused = true"
					@focusout="onPathFocusOut"
				/>
				<UButton
					size="sm"
					color="neutral"
					variant="outline"
					icon="i-fluent:folder-20-regular"
					:disabled="downloadStarted"
					@click="selectDownloadDir"
				/>
				<Theme />
				<UButton
					size="sm"
					color="neutral"
					variant="outline"
					icon="i-fluent:info-20-regular"
					aria-label="关于"
					@click="aboutOpen = true"
				/>
			</div>
		</div>

		<DownloadBar class="mb-4" />

		<div class="grid min-h-0 grid-cols-[minmax(0,1fr)_auto] gap-4">
			<DownloadTasks class="min-h-0" />
			<PatchArea />
		</div>

		<ResumeDialog />

		<ManifestDialog />

		<PatchDialog />

		<About />

		<UpdateDialog />

		<UpdateFailDialog />
	</div>
</template>

<script setup lang="ts">
	import { computed, onMounted, ref } from "vue";
	import { initDepotGameDL } from "~/composables/useDepotGameDL";
	import { useUpdater } from "~/composables/useUpdater";

	initDepotGameDL();

	// 启动时静默检查更新：有新版本才弹「发现新版本」对话框，检查失败不打扰用户
	const { checkUpdate: checkForUpdate } = useUpdater();
	onMounted(() => {
		checkForUpdate(true);
	});

	const {
		downloadDir,
		defaultDownloadDir,
		gameName,
		downloadStarted,
		selectDownloadDir,
		saveDownloadDir,
		importKey,
		cleanCache,
		aboutOpen
	} = useDepotGameDL();

	const pathFocused = ref(false);
	const pathInputValue = computed(() => {
		if (downloadStarted.value) return "";
		if (pathFocused.value) return downloadDir.value;
		if (downloadDir.value && gameName.value) return `${downloadDir.value.replace(/[\\/]+$/, "")}\\${gameName.value}`;
		return downloadDir.value;
	});

	const pathPlaceholder = computed(() => {
		const base = downloadDir.value || defaultDownloadDir.value;
		if (gameName.value && base) return `${base.replace(/[\\/]+$/, "")}\\${gameName.value}`;
		return base || "选择下载目录";
	});

	function onPathInput(val: string) {
		downloadDir.value = val;
	}

	function onPathFocusOut() {
		pathFocused.value = false;
		saveDownloadDir();
	}
</script>
