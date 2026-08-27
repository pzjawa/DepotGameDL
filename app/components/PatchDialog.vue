<template>
	<UModal v-model:open="patchDialogOpen" :title="t('patchDialog.title')" :description="t('patchDialog.description')">
		<template #footer>
			<div class="grid w-full auto-cols-max grid-flow-col justify-end gap-2">
				<UButton color="neutral" variant="ghost" @click="patchDialogOpen = false">
					{{ t('patchDialog.notNow') }}
				</UButton>
				<UButton color="primary" @click="confirmAddPatch">
					{{ t('patchDialog.addPatch') }}
				</UButton>
			</div>
		</template>
	</UModal>
</template>

<script setup lang="ts">
	import { t } from "~/locales";
	const { patchDialogOpen, confirmAddPatch } = useDepotGameDL();
</script>
