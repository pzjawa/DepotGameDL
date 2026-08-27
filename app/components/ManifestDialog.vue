<template>
	<UModal v-model:open="manifestDialogOpen" :title="t('manifestDialog.title')" :description="t('manifestDialog.description')">
		<template #footer>
			<div class="grid w-full auto-cols-max grid-flow-col justify-end gap-2">
				<UButton color="neutral" variant="ghost" @click="chooseLocalManifest">
					{{ t('manifestDialog.useLocal') }}
				</UButton>
				<UButton color="primary" @click="chooseOnlineManifest">
					{{ t('manifestDialog.fetchLatest') }}
				</UButton>
			</div>
		</template>
	</UModal>
</template>

<script setup lang="ts">
	import { t } from "~/locales";
	const { manifestDialogOpen, chooseOnlineManifest, chooseLocalManifest } = useDepotGameDL();
</script>
