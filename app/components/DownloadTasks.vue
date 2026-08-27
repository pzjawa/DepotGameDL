<template>
	<div
		class="grid size-full grid-rows-[auto_1fr] overflow-hidden rounded-lg border border-(--surface-border) bg-(--surface) shadow-sm"
	>
		<div class="grid grid-flow-col items-center justify-between border-b border-(--surface-border) px-4 py-3">
			<span class="font-semibold text-neutral-700 dark:text-neutral-200">{{ gameName || t('downloadTasks.downloadTask') }}</span>
			<span class="text-xs text-neutral-500 dark:text-neutral-400"> {{ completedCount }}/{{ totalDepots }} </span>
		</div>
		<div class="overflow-y-auto p-4">
			<ul class="space-y-2">
				<li
					v-for="depotId in preparedDepotIds"
					:key="depotId"
					class="grid grid-cols-[auto_minmax(0,1fr)] items-center gap-2"
				>
					<MorphIcon
						:icon="isCompleted(depotId) ? Check : Square"
						:size="16"
						spring="snappy"
						class="shrink-0"
						:class="isCompleted(depotId) ? 'text-green-600 dark:text-green-500' : 'text-neutral-400 dark:text-neutral-500'"
					/>
					<span
						class="grid w-fit max-w-full grid-cols-[auto_minmax(0,1fr)] items-center gap-1.5 overflow-hidden rounded-full border py-0.5 pr-2.5 pl-1 text-xs"
						:class="
							isCompleted(depotId)
								? 'border-green-200 bg-green-50 text-green-700 dark:border-green-800/60 dark:bg-green-900/20 dark:text-green-400'
								: 'border-(--surface-border) bg-neutral-100 text-neutral-700 dark:bg-neutral-800 dark:text-neutral-300'
						"
					>
						<template v-if="depotName(depotId)">
							<span class="shrink-0 rounded-full bg-white/70 px-1.5 font-mono dark:bg-black/20">{{ depotId }}</span>
							<span class="min-w-0 truncate">{{ depotName(depotId) }}</span>
						</template>
						<template v-else>
							<span class="px-1.5">{{ depotId }}</span>
						</template>
					</span>
				</li>
			</ul>
		</div>
	</div>
</template>

<script setup lang="ts">
	import { icons as tabler } from "@iconify-json/tabler";
	import { svgToIcon } from "morphicons/adapters";
	import { MorphIcon } from "morphicons/vue";
	import { t } from "~/locales";

	const Square = svgToIcon(tabler.icons.square!.body);
	const Check = svgToIcon(tabler.icons.check!.body);

	const { preparedDepotIds, completedDepots, gameName, depotName } = useDepotGameDL();

	const totalDepots = computed(() => preparedDepotIds.value.length);
	const completedCount = computed(() => preparedDepotIds.value.filter((id) => completedDepots.value.has(id)).length);

	function isCompleted(id: number) {
		return completedDepots.value.has(id);
	}
</script>
