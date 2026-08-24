<template>
	<div class="grid auto-cols-max grid-flow-col items-center gap-4">
		<UButton
			:color="isDownloading ? 'neutral' : 'primary'"
			square
			:disabled="preparingManifest"
			:title="preparingManifest ? '清单获取中' : ''"
			@click="toggleDownload"
		>
			<MorphIcon :icon="isDownloading ? Pause : Play" :size="20" spring="snappy" />
		</UButton>

		<div class="min-w-0 grid auto-cols-max grid-flow-col items-center gap-3">
			<div
				class="relative size-10 shrink-0 overflow-hidden rounded-lg border border-neutral-200 dark:border-neutral-700"
			>
				<img
					v-if="thumbSrc && !fallbackExhausted && (showFallback || gameThumb)"
					:key="showFallback ? 'fallback' : gameThumb"
					:src="thumbSrc"
					class="size-full p-1 object-contain"
					alt=""
					@error="onThumbError"
				>
			</div>

			<template v-if="hasProgress || isDownloading">
				<div class="grid min-w-0 gap-1">
					<span
						class="inline-block min-h-4 text-xs font-semibold whitespace-nowrap text-neutral-700 dark:text-neutral-200"
					>
						{{ progressDepotId }}
					</span>
					<div class="relative h-1.5 w-72 overflow-hidden rounded bg-neutral-200 dark:bg-neutral-700">
						<div
							class="h-full rounded bg-primary transition-all duration-300 ease-linear"
							:style="{ width: `${progress}%` }"
						/>
					</div>
				</div>

				<span class="w-72 overflow-hidden font-mono text-xs whitespace-nowrap text-neutral-500 dark:text-neutral-400">
					{{ progressText }}
				</span>
			</template>
		</div>
	</div>
</template>

<script setup lang="ts">
	import { icons as tabler } from "@iconify-json/tabler";
	import { svgToIcon } from "morphicons/adapters";
	import { MorphIcon } from "morphicons/vue";
	import { computed, ref, watch } from "vue";

	const Play = svgToIcon(tabler.icons["player-play"]!.body);
	const Pause = svgToIcon(tabler.icons["player-pause"]!.body);

	const {
		isDownloading,
		preparingManifest,
		startDownload,
		pauseDownload,
		hasProgress,
		progress,
		progressText,
		progressDepotId,
		gameThumb
	} = useDepotGameDL();

	function toggleDownload() {
		if (isDownloading.value) {
			pauseDownload();
		} else {
			startDownload();
		}
	}

	const thumbImg = useGameImage(() => gameThumb.value);
	const fallbackImg = useGameImage(() => "/favicon.ico", ["https://steamcommunity-a.akamaihd.net"]);

	const { exhausted: fallbackExhausted } = fallbackImg;
	const showFallback = ref(false);
	const thumbSrc = computed(() => (showFallback.value ? fallbackImg.src.value : thumbImg.src.value));

	watch(thumbImg.exhausted, (v) => {
		if (v) showFallback.value = true;
	});

	watch(gameThumb, () => {
		thumbImg.reset();
		fallbackImg.reset();
		showFallback.value = false;
	});

	function onThumbError() {
		if (!showFallback.value) thumbImg.onError();
		else fallbackImg.onError();
	}
</script>
