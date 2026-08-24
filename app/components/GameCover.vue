<template>
	<div
		class="relative mx-auto aspect-2/1 h-full max-w-full overflow-hidden rounded-lg bg-neutral-200 dark:bg-neutral-700"
	>
		<img
			v-if="appId && !coverExhausted"
			:key="appId"
			:src="coverSrc"
			class="size-full object-cover"
			alt=""
			@error="onError"
		>
	</div>
</template>

<script setup lang="ts">
	import { watch } from "vue";

	const props = defineProps<{
		appId: number | string
	}>();

	const {
		src: coverSrc,
		exhausted: coverExhausted,
		onError,
		reset
	} = useGameImage(() => `/steam/apps/${props.appId}/header.jpg`);

	watch(
		() => props.appId,
		() => {
			reset();
		}
	);
</script>
