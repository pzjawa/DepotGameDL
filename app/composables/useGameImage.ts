import { computed, ref } from "vue";

const IMAGE_DOMAINS = [
	"https://steamcdn-a.akamaihd.net",
	"https://cdn.akamai.steamstatic.com",
	"https://steamcommunity-a.akamaihd.net"
];

export function useGameImage(path: () => string, domains: string[] = IMAGE_DOMAINS) {
	const domainIndex = ref(0);
	const exhausted = ref(false);
	const src = computed(() => `${domains[domainIndex.value]}${path()}`);

	function onError() {
		if (!path()) return;
		if (domainIndex.value < domains.length - 1) {
			domainIndex.value += 1;
		} else {
			exhausted.value = true;
		}
	}

	function reset() {
		domainIndex.value = 0;
		exhausted.value = false;
	}

	return { src, exhausted, onError, reset };
}
