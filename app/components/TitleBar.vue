<template>
	<div
		class="grid h-10 grid-flow-col items-center justify-between px-4 text-neutral-800 dark:text-neutral-100"
		data-tauri-drag-region
	>
		<div
			class="grid auto-cols-max grid-flow-col items-center gap-2 text-sm font-medium"
		>
			<img src="/logo.avif" class="size-5" alt="">
			<span>{{ name }}</span>
		</div>

		<div class="grid auto-cols-max grid-flow-col items-center gap-2">
			<button
				class="group grid size-5 place-items-center rounded-full bg-[#febc2e] text-black/60 transition-all duration-200"
				:title="t('titleBar.minimize')"
				@click="minimizeWindow"
			>
				<UIcon
					name="i-fluent:subtract-20-regular"
					class="size-5 opacity-0 transition-opacity duration-200 group-hover:opacity-100"
				/>
			</button>
			<button
				class="group grid size-5 place-items-center rounded-full bg-[#28c840] text-black/60 transition-all duration-200"
				:title="t('titleBar.maximize')"
				@click="toggleMaximize"
			>
				<UIcon
					name="i-fluent:square-20-regular"
					class="size-5 opacity-0 transition-opacity duration-200 group-hover:opacity-100"
				/>
			</button>
			<button
				class="group grid size-5 place-items-center rounded-full bg-[#ff5f57] text-black/60 transition-all duration-200"
				:title="t('titleBar.close')"
				@click="closeWindow"
			>
				<UIcon
					name="i-fluent:dismiss-20-regular"
					class="size-5 opacity-0 transition-opacity duration-200 group-hover:opacity-100"
				/>
			</button>
		</div>
	</div>
</template>

<script setup lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { getCurrentWindow } from "@tauri-apps/api/window";
	import { onMounted, ref } from "vue";
	import { t } from "~/locales";

	interface AppMeta {
		name: string
	}

	const appWindow = getCurrentWindow();
	const name = ref("");

	onMounted(async () => {
		try {
			const meta = await invoke<AppMeta>("get_app_meta");
			name.value = meta.name;
		} catch {}
	});

	async function minimizeWindow() {
		await appWindow.minimize();
	}

	async function toggleMaximize() {
		await appWindow.toggleMaximize();
	}

	async function closeWindow() {
		await appWindow.close();
	}
</script>

<style scoped>
[data-tauri-drag-region] {
  -webkit-app-region: drag;
}
:deep(button) {
  -webkit-app-region: no-drag;
}
</style>
