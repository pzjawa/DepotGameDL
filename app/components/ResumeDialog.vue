<template>
	<UModal
		v-model:open="resumeDialogOpen"
		:title="t('resumeDialog.title')"
		:description="t('resumeDialog.description')"
	>
		<template #footer>
			<div class="grid w-full auto-cols-max grid-flow-col justify-end gap-2">
				<UButton color="neutral" variant="ghost" @click="discardResume">
					{{ t('resumeDialog.cleanCache') }}
				</UButton>
				<UButton color="primary" @click="confirmResume">
					{{ t('resumeDialog.continueDownload') }}
				</UButton>
			</div>
		</template>
	</UModal>
</template>

<script setup lang="ts">
	import { t } from "~/locales";
	const { resumeDialogOpen, confirmResume, discardResume } = useDepotGameDL();
</script>
