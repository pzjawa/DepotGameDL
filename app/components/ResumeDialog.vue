<template>
	<UModal
		v-model:open="resumeDialogOpen"
		title="下载缓存"
		description="是否使用上次下载的缓存继续？"
	>
		<template #footer>
			<div class="grid w-full auto-cols-max grid-flow-col justify-end gap-2">
				<UButton color="neutral" variant="ghost" @click="discardResume">
					清理缓存
				</UButton>
				<UButton color="primary" @click="confirmResume">
					继续下载
				</UButton>
			</div>
		</template>
	</UModal>
</template>

<script setup lang="ts">
	const { resumeDialogOpen, confirmResume, discardResume } = useDepotGameDL();
</script>
