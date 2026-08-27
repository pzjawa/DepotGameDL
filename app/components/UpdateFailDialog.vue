<template>
	<UModal
		v-model:open="failDialogOpen"
		:title="t('updateFailDialog.title')"
		:description="t('updateFailDialog.description')"
		:ui="{ overlay: 'z-100', content: 'z-100' }"
	>
		<template #footer>
			<div class="grid w-full auto-cols-max grid-flow-col justify-end gap-2">
				<UButton color="neutral" variant="ghost" @click="failDialogOpen = false">
					{{ t('updateFailDialog.cancel') }}
				</UButton>
				<UButton color="primary" @click="goManualUpdate">
					{{ t('updateFailDialog.confirm') }}
				</UButton>
			</div>
		</template>
	</UModal>
</template>

<script setup lang="ts">
	import { t } from "~/locales";
	const { failDialogOpen, goManualUpdate } = useUpdater();
</script>
