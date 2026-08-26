<template>
	<UModal v-model:open="updateDialogOpen" title="发现新版本" :ui="{ overlay: 'z-100', content: 'z-100' }">
		<template #body>
			<div class="grid gap-3">
				<div class="grid auto-cols-max grid-flow-col items-center gap-2">
					<span class="text-sm text-neutral-500 line-through dark:text-neutral-400">V{{ currentVersion }}</span>
					<UIcon name="i-fluent:arrow-right-20-regular" class="size-4 text-neutral-400 dark:text-neutral-500" />
					<span class="text-sm font-semibold text-primary">V{{ updateInfo?.version }}</span>
				</div>

				<p
					v-if="updateInfo?.notes"
					class="max-h-48 overflow-y-auto text-left text-sm whitespace-pre-line text-neutral-700 dark:text-neutral-300"
				>
					{{ updateInfo.notes }}
				</p>

				<div v-if="downloading" class="grid gap-1.5">
					<UProgress v-model="downloadProgress" />
					<span class="text-xs text-neutral-500 dark:text-neutral-400">正在下载更新… {{ downloadProgress }}%</span>
				</div>
			</div>
		</template>

		<template #footer>
			<div class="grid w-full auto-cols-max grid-flow-col items-center justify-between gap-2">
				<UButton color="neutral" variant="ghost" :disabled="downloading" @click="skipVersion">
					跳过这个版本
				</UButton>
				<div class="grid auto-cols-max grid-flow-col items-center gap-2">
					<UButton color="neutral" variant="ghost" :disabled="downloading" @click="remindLaterSession">
						本次不再提醒
					</UButton>
					<UButton color="primary" :disabled="downloading" @click="installUpdate">
						立即更新
					</UButton>
				</div>
			</div>
		</template>
	</UModal>
</template>

<script setup lang="ts">
	const {
		downloading,
		downloadProgress,
		updateDialogOpen,
		updateInfo,
		currentVersion,
		installUpdate,
		skipVersion,
		remindLaterSession
	} = useUpdater();
</script>
