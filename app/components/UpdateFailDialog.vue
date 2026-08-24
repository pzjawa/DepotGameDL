<template>
	<UModal
		v-model:open="failDialogOpen"
		title="检查更新"
		description="更新失败，是否前往手动更新？"
		:ui="{ overlay: 'z-100', content: 'z-100' }"
	>
		<template #footer>
			<div class="grid w-full auto-cols-max grid-flow-col justify-end gap-2">
				<UButton color="neutral" variant="ghost" @click="failDialogOpen = false">
					取消
				</UButton>
				<UButton color="primary" @click="goManualUpdate">
					确定
				</UButton>
			</div>
		</template>
	</UModal>
</template>

<script setup lang="ts">
	const { failDialogOpen, goManualUpdate } = useUpdater();
</script>
