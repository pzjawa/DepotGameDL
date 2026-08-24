<p align="center">
  <img width="150" src="./public/logo.avif" alt="DepotGameDL Logo">
</p>

<h1 align="center">DepotGameDL</h1>

<p align="center">
  基于 <a href="https://github.com/NicolaSpadari/nuxtor">NUXTOR</a>（<a href="https://nuxt.com">Nuxt 4</a> + <a href="https://v2.tauri.app">Tauri 2</a>）
  构建的 Steam 游戏下载工具
</p>

<p align="center">
  <img src="https://img.shields.io/github/package-json/v/pzjawa/depotgamedl" alt=""/>
  <img src="https://img.shields.io/github/license/pzjawa/depotgamedl" alt=""/>
</p>

> ⚠️本项目仅作技术学习交流用途，请尊重游戏开发者的劳动成果

## 使用说明

1. 点击「导入清单」选择 `.lua` 文件或清单压缩包
2. 选择使用本地或在线版本
3. 下载完成后添加联网补丁

> 需自行获取清单，本项目暂不支持该功能  

## 功能特性

- **高速下载**：使用 Steam 官方服务器，下载不限速
- **下载功能**：下载进度显示、断点续传，下载缓存
- **联网补丁**：内置补丁添加功能，可选择使用 [FreeTp.Org](https://freetp.org/) 或 [Online-Fix](https://online-fix.me/)
- **安全可靠**：非假入库，无封号风险
- **主题切换**：WinUI3 设计语言，适配昼夜模式

> 可用于下载 Steam 游戏的特定版本或游玩锁区游戏

## 技术栈

- **前端**
  - Nuxt 4
  - Vue 3
  - Nuxt UI v4
  - Tailwind CSS v4
  - VueUse
  - TypeScript
  - ESLint
- **后端**
  - Tauri 2
  - Rust
- **依赖**
  - [detiam/DepotDownloader](https://github.com/detiam/DepotDownloader)

## 环境要求

- Node.js ≥ 20
- Rust
- Windows 10/11

## 首次使用

```sh
# 安装依赖
pnpm install

# 开发模式
pnpm tauri:dev
```

## 构建打包

```sh
# 正式构建
pnpm tauri:build
```

输出目录：src-tauri/target

```sh
# 调试构建
pnpm tauri:build:debug
```

## 致谢

- [NUXTOR](https://github.com/NicolaSpadari/nuxtor)
- [detiam/DepotDownloader](https://github.com/detiam/DepotDownloader)

## 开源协议

[MIT](./LICENSE) © 2026 pzjawa

