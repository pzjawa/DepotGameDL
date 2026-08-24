<template>
	<UPopover>
		<UButton color="neutral" variant="outline" size="sm" square aria-label="主题设置">
			<MorphIcon :icon="themeIcons[themeMode] ?? Palette" :size="16" spring="snappy" />
		</UButton>

		<template #content>
			<div class="w-60 bg-(--surface) rounded-lg p-3">
				<div class="mb-1.5 text-xs text-neutral-500 dark:text-neutral-400">
					主题
				</div>
				<div class="grid grid-cols-3 rounded bg-neutral-100 p-1 dark:bg-neutral-800">
					<button
						v-for="m in themeModes"
						:key="m.value"
						class="rounded px-1.5 py-1 text-xs whitespace-nowrap transition-colors"
						:class="
							themeMode === m.value
								? 'bg-white dark:bg-neutral-700 shadow-sm text-neutral-900 dark:text-white'
								: 'text-neutral-500 dark:text-neutral-400 hover:text-neutral-700 dark:hover:text-neutral-200'
						"
						@click="setThemeMode(m.value)"
					>
						{{ m.label }}
					</button>
				</div>

				<div class="mt-4 mb-1.5 text-xs text-neutral-500 dark:text-neutral-400">
					样式
				</div>
				<div class="grid grid-cols-3 rounded bg-neutral-100 p-1 dark:bg-neutral-800">
					<button
						v-for="s in styles"
						:key="s.value"
						class="rounded px-1.5 py-1 text-xs whitespace-nowrap transition-colors"
						:class="
							windowStyle === s.value
								? 'bg-white dark:bg-neutral-700 shadow-sm text-neutral-900 dark:text-white'
								: 'text-neutral-500 dark:text-neutral-400 hover:text-neutral-700 dark:hover:text-neutral-200'
						"
						@click="setWindowStyle(s.value)"
					>
						{{ s.label }}
					</button>
				</div>
			</div>
		</template>
	</UPopover>
</template>

<script setup lang="ts">
	import { icons as tabler } from "@iconify-json/tabler";
	import { svgToIcon } from "morphicons/adapters";
	import { MorphIcon } from "morphicons/vue";

	const Palette = svgToIcon(tabler.icons.palette!.body);
	const themeIcons: Record<string, ReturnType<typeof svgToIcon>> = {
		system: Palette,
		light: svgToIcon(tabler.icons.sun!.body),
		dark: svgToIcon(tabler.icons.moon!.body)
	};

	const { themeMode, windowStyle, setThemeMode, setWindowStyle } = useDepotGameDL();

	const themeModes = [
		{ label: "跟随系统", value: "system" },
		{ label: "浅色", value: "light" },
		{ label: "深色", value: "dark" }
	];

	const styles = [
		{ label: "Default", value: "default" },
		{ label: "Mica Alt", value: "mica_alt" },
		{ label: "Acrylic", value: "acrylic" }
	];
</script>
