# CodexPlusPlus Ad-Free

<p align="center">
  <img src="docs/images/codex-plus-plus.png" alt="Codex++ icon" width="160">
</p>

<p align="center">
  <a href="README.md">中文</a> | English
</p>

<p align="center">
  <img alt="Release" src="https://img.shields.io/github/v/release/ziiji/CodexPlusPlus-AdFree">
  <img alt="License" src="https://img.shields.io/github/license/ziiji/CodexPlusPlus-AdFree">
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-orange">
  <img alt="Tauri" src="https://img.shields.io/badge/tauri-2.x-24C8DB">
</p>

This is an unofficial ad-free fork of [BigPizzaV3/CodexPlusPlus](https://github.com/BigPizzaV3/CodexPlusPlus). It keeps the main Codex++ functionality while removing advertising and promotional UI again after each upstream update.

This repository is not an official distribution channel for the upstream project. The original project name, code, and copyrights belong to their respective authors and contributors. This fork is distributed under the AGPL-3.0-only License.

## Downloads

Download the latest build from this repository's [GitHub Releases](https://github.com/ziiji/CodexPlusPlus-AdFree/releases):

- Windows: `CodexPlusPlus-*-windows-x64-setup.exe`
- macOS Intel: `CodexPlusPlus-*-macos-x64.dmg`
- macOS Apple Silicon: `CodexPlusPlus-*-macos-arm64.dmg`

The Windows installer is not commercially code-signed, so Windows SmartScreen may warn on first launch. Download only from this repository's Release page and verify the published SHA256 when available.

## Differences From Upstream

- Disables the remote ad list and no longer connects to `BigPizzaV3/Ad-List`.
- Removes bundled sponsors and sponsor image data.
- Removes the recommendation page and overview promotion from the manager.
- Removes recommendation, sponsor, donation QR, and community promotion entries from the injected menu.
- Automatic updates only check GitHub Releases from `ziiji/CodexPlusPlus-AdFree`.
- Keeps API provider presets that users explicitly choose; presets are not automatically displayed as advertisements.

## Update Policy

Version numbers follow upstream releases. For example, upstream `v1.2.41` maps to this fork's `v1.2.41`. One ad-free build is published for each upstream version. The app checks the Release asset `latest.json` to discover and install updates.

Each sync runs ad-removal regression tests, frontend checks, Rust tests, a release build, and a final binary scan for advertising markers.

## Building From Source

Rust and Node.js 22+ are required. NSIS is also required for the Windows installer.

```powershell
cd apps/codex-plus-manager
npm ci
npm run check
npm run vite:build
cd ../..
cargo test -p codex-plus-core
cargo build --release
```

## License and Attribution

This project is distributed under the [GNU Affero General Public License v3.0 only](LICENSE).

- Upstream: [BigPizzaV3/CodexPlusPlus](https://github.com/BigPizzaV3/CodexPlusPlus)
- Ad-free fork maintainer: [ziiji](https://github.com/ziiji)
- Fork issue tracker: [Issues](https://github.com/ziiji/CodexPlusPlus-AdFree/issues)

Please do not report fork-specific issues to the upstream project.
