<template>
	<UModal v-model:open="patchDialogOpen" title="下载完成" description="游戏下载完成，是否添加补丁？">
		<template #footer>
			<div class="grid w-full auto-cols-max grid-flow-col justify-end gap-2">
				<UButton color="neutral" variant="ghost" @click="patchDialogOpen = false">
					暂不添加
				</UButton>
				<UButton color="primary" @click="confirmAddPatch">
					添加补丁
				</UButton>
			</div>
		</template>
	</UModal>
</template>

<script setup lang="ts">
	const { patchDialogOpen, confirmAddPatch } = useDepotGameDL();
</script>
