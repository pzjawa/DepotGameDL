export default {
	common: {
		cancel: "取消",
		confirm: "确定"
	},
	titleBar: {
		minimize: "最小化",
		maximize: "最大化",
		close: "关闭"
	},
	index: {
		importManifest: "导入清单",
		cleanCache: "清理缓存",
		selectDownloadDir: "选择下载目录",
		about: "关于"
	},
	downloadBar: {
		fetchingManifest: "清单获取中"
	},
	downloadTasks: {
		downloadTask: "下载任务"
	},
	theme: {
		settings: "主题设置",
		theme: "主题",
		style: "样式",
		followSystem: "跟随系统",
		light: "浅色",
		dark: "深色"
	},
	patchArea: {
		onlinePatch: "联网补丁",
		localPatchTab: "本地补丁",
		onlinePatchTab: "在线补丁",
		steamLinkPlaceholder: "Steam游戏链接或ID",
		gameDirPlaceholder: "游戏目录",
		patchFolderPlaceholder: "补丁文件夹",
		addPatch: "添加补丁",
		removePatch: "移除补丁",
		helpTitle: "使用说明",
		helpDenuvo: "联网补丁仅支持非 Denuvo Anti-Tamper 游戏，添加后可正常使用联机等联网功能",
		helpSteamClient: "游玩时需启动 Steam 客户端，否则会无法启动游戏",
		toastInvalidContent: "内容无效",
		toastSelectGameDirFirst: "请先选择游戏目录",
		toastSelectPatchFolderFirst: "请先选择补丁文件夹",
		toastPatchAdded: "补丁已添加",
		toastPatchRemoved: "已移除补丁"
	},
	resumeDialog: {
		title: "下载缓存",
		description: "是否使用上次下载的缓存继续？",
		cleanCache: "清理缓存",
		continueDownload: "继续下载"
	},
	manifestDialog: {
		title: "本地版本",
		description: "已有本地版本，是否尝试获取最新版本？",
		useLocal: "使用本地版本",
		fetchLatest: "获取最新版本"
	},
	patchDialog: {
		title: "下载完成",
		description: "游戏下载完成，是否添加补丁？",
		notNow: "暂不添加",
		addPatch: "添加补丁"
	},
	updateDialog: {
		title: "发现新版本",
		downloading: "正在下载更新… {progress}%",
		skipVersion: "跳过这个版本",
		remindLater: "本次不再提醒",
		updateNow: "立即更新"
	},
	updateFailDialog: {
		title: "检查更新",
		description: "更新失败，是否前往手动更新？",
		cancel: "取消",
		confirm: "确定"
	},
	about: {
		projectRepo: "项目地址",
		checkUpdate: "检查更新",
		credits: "致谢",
		toastAlreadyLatest: "当前已是最新版本"
	},
	toasts: {
		manifestProgress: "获取清单 [{current}/{total}]",
		currentNode: "当前节点：{node}",
		downloadFinished: "下载完毕",
		resumeLastDownload: "继续上次下载",
		cacheCleaned: "缓存已清理",
		cacheCleanFailed: "清理缓存失败: {error}",
		parsingManifest: "正在解析清单",
		parseManifestFailed: "清单解析失败",
		fetchingOnlineManifest: "在线获取清单",
		fetchManifestFailed: "获取清单失败: {error}",
		localManifestInvalid: "本地清单无效: {error}",
		actionFailed: "失败: {error}",
		cacheIncomplete: "缓存不完整，请重新导入清单",
		importManifestFirst: "请先导入清单文件",
		downloadAlreadyDone: "下载已完成",
		startDownloadFailed: "启动下载失败: {error}",
		cannotAddPatch: "无法添加补丁",
		patchAdded: "补丁已添加",
		manifestFilterName: "清单文件"
	}
};
