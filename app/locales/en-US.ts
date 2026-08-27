export default {
	common: {
		cancel: "Cancel",
		confirm: "OK"
	},
	titleBar: {
		minimize: "Minimize",
		maximize: "Maximize",
		close: "Close"
	},
	index: {
		importManifest: "Import Manifest",
		cleanCache: "Clear Cache",
		selectDownloadDir: "Select Download Directory",
		about: "About"
	},
	downloadBar: {
		fetchingManifest: "Fetching Manifest"
	},
	downloadTasks: {
		downloadTask: "Download Tasks"
	},
	theme: {
		settings: "Theme Settings",
		theme: "Theme",
		style: "Style",
		followSystem: "Follow System",
		light: "Light",
		dark: "Dark"
	},
	patchArea: {
		onlinePatch: "Online Patch",
		localPatchTab: "Local Patch",
		onlinePatchTab: "Online Patch",
		steamLinkPlaceholder: "Steam Game Link or ID",
		gameDirPlaceholder: "Game Directory",
		patchFolderPlaceholder: "Patch Folder",
		addPatch: "Add Patch",
		removePatch: "Remove Patch",
		helpTitle: "Instructions",
		helpDenuvo: "Online patch only supports non-Denuvo Anti-Tamper games. After adding, you can use online multiplayer and other network features.",
		helpSteamClient: "You need to launch the Steam client while playing, otherwise the game cannot be started.",
		toastInvalidContent: "Invalid content",
		toastSelectGameDirFirst: "Please select game directory first",
		toastSelectPatchFolderFirst: "Please select patch folder first",
		toastPatchAdded: "Patch added",
		toastPatchRemoved: "Patch removed"
	},
	resumeDialog: {
		title: "Download Cache",
		description: "Resume with previous download cache?",
		cleanCache: "Clear Cache",
		continueDownload: "Continue Download"
	},
	manifestDialog: {
		title: "Local Version",
		description: "Local version exists. Try to get the latest version?",
		useLocal: "Use Local Version",
		fetchLatest: "Get Latest Version"
	},
	patchDialog: {
		title: "Download Complete",
		description: "Game download complete. Add patch?",
		notNow: "Not Now",
		addPatch: "Add Patch"
	},
	updateDialog: {
		title: "New Version Found",
		downloading: "Downloading update... {progress}%",
		skipVersion: "Skip This Version",
		remindLater: "Don't Remind This Session",
		updateNow: "Update Now"
	},
	updateFailDialog: {
		title: "Check for Updates",
		description: "Update failed. Go to manual update?",
		cancel: "Cancel",
		confirm: "OK"
	},
	about: {
		projectRepo: "Project Repository",
		checkUpdate: "Check for Updates",
		credits: "Credits",
		toastAlreadyLatest: "Already the latest version"
	},
	toasts: {
		manifestProgress: "Fetching manifest [{current}/{total}]",
		currentNode: "Current node: {node}",
		downloadFinished: "Download finished",
		resumeLastDownload: "Resuming previous download",
		cacheCleaned: "Cache cleared",
		cacheCleanFailed: "Cache clear failed: {error}",
		parsingManifest: "Parsing manifest",
		parseManifestFailed: "Manifest parse failed",
		fetchingOnlineManifest: "Fetching manifest online",
		fetchManifestFailed: "Failed to fetch manifest: {error}",
		localManifestInvalid: "Local manifest invalid: {error}",
		actionFailed: "Failed: {error}",
		cacheIncomplete: "Cache incomplete, please re-import manifest",
		importManifestFirst: "Please import manifest file first",
		downloadAlreadyDone: "Download already complete",
		startDownloadFailed: "Failed to start download: {error}",
		cannotAddPatch: "Unable to add patch",
		patchAdded: "Patch added",
		manifestFilterName: "Manifest Files"
	}
};
