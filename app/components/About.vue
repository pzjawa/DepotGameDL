<template>
	<UModal v-model:open="aboutOpen">
		<template #body>
			<div class="grid justify-items-center py-2">
				<img src="/logo.avif" class="mb-3 size-16 rounded-lg" alt="">

				<span class="text-lg font-semibold text-neutral-900 dark:text-white">{{ displayName }}</span>

				<span class="mt-0.5 text-sm text-neutral-500 dark:text-neutral-400">V{{ version }}</span>

				<button
					class="mt-5 grid w-full grid-cols-[auto_1fr] items-center gap-3 rounded-lg bg-neutral-100 p-3 text-left transition-colors hover:bg-neutral-200 dark:bg-neutral-800 dark:hover:bg-neutral-700"
					@click="openProfile"
				>
					<img
						:src="avatarFailed ? '/logo.avif' : avatarUrl"
						class="size-10 rounded-full"
						alt=""
						@error="avatarFailed = true"
					>
					<div class="grid">
						<span class="text-sm font-medium text-neutral-800 dark:text-neutral-200">yukino</span>
						<span class="text-xs text-neutral-500 dark:text-neutral-400">@{{ author }}</span>
					</div>
				</button>

				<div class="mt-2 grid w-full grid-cols-[1fr_auto] items-stretch gap-2">
					<div
						class="grid cursor-pointer grid-cols-[auto_1fr_auto] items-center gap-3 select-none rounded-lg bg-neutral-100 p-3 transition-colors hover:bg-neutral-200 dark:bg-neutral-800 dark:hover:bg-neutral-700"
						@click="openRepo"
					>
						<UIcon name="i-tabler:brand-github" class="size-5 text-neutral-700 dark:text-neutral-300" />
						<span class="text-left text-sm text-neutral-800 dark:text-neutral-200">{{ t('about.projectRepo') }}</span>
						<UIcon
							name="i-fluent:chevron-right-20-regular"
							class="size-5 justify-self-end text-neutral-400 dark:text-neutral-500"
						/>
					</div>

					<button
						class="grid auto-cols-max grid-flow-col items-center gap-2 rounded-lg bg-neutral-100 px-4 transition-colors hover:bg-neutral-200 dark:bg-neutral-800 dark:hover:bg-neutral-700"
						@click="onCheckUpdate"
					>
						<UIcon
							name="i-fluent:arrow-sync-20-regular"
							class="size-5 text-neutral-700 dark:text-neutral-300"
							:class="{ 'animate-spin': checking }"
						/>
						<span class="text-sm text-neutral-800 dark:text-neutral-200">{{ t('about.checkUpdate') }}</span>
					</button>
				</div>

				<span class="mb-1 mt-5 w-full text-left text-sm font-semibold text-neutral-900 dark:text-white">{{ t('about.credits') }}</span>

				<button
					v-for="(p, i) in refProjects"
					:key="p.url"
					class="grid w-full grid-cols-[auto_1fr_auto] items-center gap-3 rounded-lg bg-neutral-100 p-3 transition-colors hover:bg-neutral-200 dark:bg-neutral-800 dark:hover:bg-neutral-700"
					:class="i === 0 ? '' : 'mt-2'"
					@click="openLink(p.url)"
				>
					<div class="grid size-8 shrink-0 place-items-center">
						<img
							v-if="logoUrl(p.url) && !failedLogos.has(p.url)"
							:src="logoUrl(p.url)"
							class="size-full rounded-lg"
							alt=""
							@error="failedLogos.add(p.url)"
						>
						<UIcon
							v-else
							name="i-tabler:brand-github"
							class="size-5 text-neutral-700 dark:text-neutral-300"
						/>
					</div>
					<span class="text-left text-sm text-neutral-800 dark:text-neutral-200">{{ p.name }}</span>
					<UIcon
						name="i-fluent:chevron-right-20-regular"
						class="size-5 justify-self-end text-neutral-400 dark:text-neutral-500"
					/>
				</button>
			</div>
		</template>
	</UModal>
</template>

<script setup lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { open } from "@tauri-apps/plugin-shell";
	import { onMounted, ref } from "vue";
	import { t } from "~/locales";

	interface AppMeta {
		name: string
		display_name: string
		version: string
		repository: string
		author: string
	}

	const { aboutOpen } = useDepotGameDL();
	const { checking, checkUpdate } = useUpdater();

	const toast = useToast();

	async function onCheckUpdate() {
		const result = await checkUpdate();
		if (result === "latest") {
			toast.add({ title: t('about.toastAlreadyLatest'), close: false, progress: false, duration: 3000 });
		}
	}

	const refProjects = [
		{ name: "Nuxtor", url: "https://github.com/NicolaSpadari/nuxtor" },
		{ name: "DepotDownloader", url: "https://github.com/detiam/DepotDownloader" }
	];

	const failedLogos = ref(new Set<string>());

	function logoUrl(url: string) {
		const owner = url.match(/github\.com\/([^/#?]+)/)?.[1];
		return owner ? `https://github.com/${owner}.png` : "";
	}

	const displayName = ref("");
	const version = ref("");
	const author = ref("");
	const repository = ref("");
	const avatarUrl = ref("");
	const avatarFailed = ref(false);

	onMounted(async () => {
		try {
			const meta = await invoke<AppMeta>("get_app_meta");
			displayName.value = meta.display_name || meta.name;
			version.value = meta.version;
			author.value = meta.author;
			repository.value = meta.repository;
			avatarUrl.value = `https://github.com/${meta.author}.png`;
		} catch {}
	});

	function openProfile() {
		if (author.value) {
			open(`https://github.com/${author.value}`).catch(() => {});
		}
	}

	function openRepo() {
		if (repository.value) {
			open(repository.value).catch(() => {});
		}
	}

	function openLink(url: string) {
		open(url).catch(() => {});
	}
</script>
