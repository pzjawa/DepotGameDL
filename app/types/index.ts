export interface DepotInfo {
	id: number
	sha: string
}

export interface LuaInfo {
	appid: number | null
	depots: DepotInfo[]
	dlc_depots: number[]
	tokens: Record<number, string>
	missing_keys: number[]
	dlc_only: number[]
}
