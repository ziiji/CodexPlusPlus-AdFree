# HANDOVER - CodexPlusPlus Ad-Free

## Active Work

Sync the ad-free fork from upstream `v1.2.45` to `v1.2.46`.

- Upstream: `BigPizzaV3/CodexPlusPlus`
- Fork: `ziiji/CodexPlusPlus-AdFree`
- Upstream tag: `v1.2.46` (`a30b550`)
- Release commit subject: `Publish v1.2.46 ad-free fork`

## Completed

- Merged the official `v1.2.46` source tree and preserved the DreamSkin community theme market, package validation, and safe CSS checks.
- Reapplied the existing ad-free patch after the merge; the only source conflict was resolved in `crates/codex-plus-core/src/assets.rs` so managed theme CSS remains available while promotional theme fields stay stripped.
- Updated release-facing documentation to `1.2.46`.
- Preserved the fork update channel at `ziiji/CodexPlusPlus-AdFree`.
- Kept the default ad source list empty and removed the new bundled upstream sponsors.
- Preserved the upstream `v1.2.45` feature changes, including the native new-thread flow, VLM bridge fixes, installer progress handling, and renderer updates.
- Preserved the upstream `v1.2.46` DreamSkin community gallery, one-click installation flow, and local package safety verification.
- Kept provider presets that users explicitly select; they are not rendered automatically as ads.

## Verification

Passed locally:

```text
npm test: 36 passed, 0 failed
npm run check: passed
npm run vite:build: passed
node --check assets/inject/renderer-inject.js
JSON parse check: 22 files valid
runtime ad-marker scan: clean
release binary ad-marker scan: clean
git diff --check: clean
theme JSON SHA-256 checks: all expected values matched
cargo test -p codex-plus-core: passed with one Windows privilege-only test skipped
cargo build --release: passed
```

Installed development dependencies:

```text
Node.js 24.18.1 / npm 11.16.0
frontend dependencies from npm ci
rustc/cargo 1.97.1 (stable-x86_64-pc-windows-msvc)
NSIS 3.12
```

`app_paths_resolves_portable_current_link_to_directory_version` requires Windows symbolic-link privileges and was skipped because this session was not elevated. All other Rust tests passed in a single-threaded run. A parallel run also exposed one transient mock HTTP 502; the affected test passed alone and in the final single-threaded suite.

## Next Steps

1. Build the NSIS installer and release metadata.
2. Publish the `v1.2.46` installers and `latest.json`.
3. Tag the final release commit as `v1.2.46` and push `main` plus the tag to the fork.

Unused sponsor images remain in the source history but have no code or documentation references and are not embedded in the executable. Do not bulk-delete them unless repository-level asset removal is explicitly desired.
