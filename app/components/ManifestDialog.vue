<template>
	<UModal v-model:open="manifestDialogOpen" title="本地版本" description="已有本地版本，是否尝试获取最新版本？">
		<template #footer>
			<div class="grid w-full auto-cols-max grid-flow-col justify-end gap-2">
				<UButton color="neutral" variant="ghost" @click="chooseLocalManifest">
					使用本地版本
				</UButton>
				<UButton color="primary" @click="chooseOnlineManifest">
					获取最新版本
				</UButton>
			</div>
		</template>
	</UModal>
</template>

<script setup lang="ts">
	const { manifestDialogOpen, chooseOnlineManifest, chooseLocalManifest } = useDepotGameDL();
</script>
