import enUS from "./en-US";
import zhCN from "./zh-CN";

const locales: Record<string, typeof zhCN> = {
	"zh-CN": zhCN,
	"en-US": enUS
};

function detectLocale(): string {
	const lang = navigator.language;
	if (lang in locales) return lang;
	if (lang.startsWith("zh")) return "zh-CN";
	return "en-US";
}

export const currentLocale = detectLocale();

const dict: Record<string, Record<string, string>> = locales[currentLocale] ?? zhCN;

function lookup(key: string): string | undefined {
	const idx = key.indexOf(".");
	if (idx === -1) return undefined;
	const section = key.slice(0, idx);
	const name = key.slice(idx + 1);
	return dict[section]?.[name];
}

type Primitive = string | number;

export function t(key: string, params?: Record<string, Primitive>): string {
	const template = lookup(key);
	if (!template) return key;
	if (!params) return template;
	return template.replace(/\{(\w+)\}/g, (_, name: string) =>
		params[name] !== undefined ? String(params[name]) : `{${name}}`);
}
