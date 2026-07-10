# CodexPlusPlus Ad-Free

<p align="center">
  <img src="docs/images/codex-plus-plus.png" alt="Codex++ 图标" width="160">
</p>

<p align="center">
  中文 | <a href="README_EN.md">English</a>
</p>

<p align="center">
  <img alt="Release" src="https://img.shields.io/github/v/release/ziiji/CodexPlusPlus-AdFree">
  <img alt="License" src="https://img.shields.io/github/license/ziiji/CodexPlusPlus-AdFree">
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-orange">
  <img alt="Tauri" src="https://img.shields.io/badge/tauri-2.x-24C8DB">
</p>

这是 [BigPizzaV3/CodexPlusPlus](https://github.com/BigPizzaV3/CodexPlusPlus) 的非官方去广告分支。项目保留 Codex++ 的主要功能，并在每次同步上游版本后重新移除广告和推广界面。

本仓库不是原项目的官方发布渠道。原项目名称、代码和版权归原作者及贡献者所有；本分支按 AGPL-3.0-only 许可证发布。

## 下载

从本仓库的 [GitHub Releases](https://github.com/ziiji/CodexPlusPlus-AdFree/releases) 下载最新版：

- Windows：`CodexPlusPlus-*-windows-x64-setup.exe`
- macOS Intel：`CodexPlusPlus-*-macos-x64.dmg`
- macOS Apple Silicon：`CodexPlusPlus-*-macos-arm64.dmg`

Windows 安装包目前没有商业代码签名，首次运行时可能出现 SmartScreen 提示。请只从本仓库 Release 页面下载，并可用 Release 中公布的 SHA256 校验文件。

## 与上游的差异

- 禁用远端广告列表，不再连接 `BigPizzaV3/Ad-List`。
- 移除内置赞助商及其图片数据。
- 移除管理器中的推荐内容页和概览推广卡片。
- 移除注入菜单中的推荐、赞助、赞赏二维码和社群推广入口。
- 自动更新只检查 `ziiji/CodexPlusPlus-AdFree` 的 GitHub Releases。
- 保留用户主动选择的 API 供应商预设；供应商预设不作为展示广告自动出现。

## 更新策略

本分支版本号跟随上游版本，例如上游 `v1.2.36` 对应本分支 `v1.2.36`。每个上游版本只发布一个去广告构建。软件通过 Release 中的 `latest.json` 检查和下载安装包。

上游更新后，本分支会重新执行去广告测试、前端检查、Rust 测试、release 构建和最终二进制广告关键词扫描。

## 从源码构建

需要 Rust、Node.js 22+；Windows 安装包还需要 NSIS。

```powershell
cd apps/codex-plus-manager
npm ci
npm run check
npm run vite:build
cd ../..
cargo test -p codex-plus-core
cargo build --release
```

Windows NSIS 安装包：

```powershell
New-Item -ItemType Directory -Force dist/windows/app | Out-Null
Copy-Item target/release/codex-plus-plus.exe dist/windows/app/
Copy-Item target/release/codex-plus-plus-manager.exe dist/windows/app/
Push-Location scripts/installer/windows
& "${env:ProgramFiles(x86)}\NSIS\makensis.exe" "/INPUTCHARSET" "UTF8" "/DVERSION=1.2.36" CodexPlusPlus.nsi
Pop-Location
```

## 许可证与归属

本项目使用 [GNU Affero General Public License v3.0 only](LICENSE)。

- 上游项目：[BigPizzaV3/CodexPlusPlus](https://github.com/BigPizzaV3/CodexPlusPlus)
- 去广告分支维护者：[ziiji](https://github.com/ziiji)
- 本分支问题反馈：[Issues](https://github.com/ziiji/CodexPlusPlus-AdFree/issues)

请不要向上游项目报告仅在本去广告分支中出现的问题。
