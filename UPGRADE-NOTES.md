# Upgrade notes

## What changed under the hood

- **New crate `atlas-catalog`** (libraries, persistent index, scanning, duplicate analysis) and new modules
  in `atlas-db`, `atlas-duplicate`, `atlas-scanner`, `atlas-library`, `atlas-core`.
- **Database migration 2** (`library_roots`, `library_assets`, `library_folders`, `archive_hashes`,
  `package_files`) runs automatically on first launch. It only adds tables.
- `uuid` gained the `v5` feature; `atlas-scanner` gained `serde`/`serde_json`; `atlas-library` gained a
  dev-dependency on `chrono`; `atlas-catalog` is new. Run `cargo build` (or `cargo test --workspace`) once and
  commit the updated `Cargo.lock`. CI does not use `--locked`, so it would also update it on its own.
- No new JavaScript dependencies (`pnpm install --frozen-lockfile` is unchanged).

## First launch after upgrading

1. The folder you had saved becomes your **first library** (its asset ids stay the same as in the previous
   build). Because nothing is indexed yet, one full scan runs; from then on start-up is instant.
2. Tags, ratings and favourites saved against the old random ids can't be matched (they were already orphaned on
   every restart), so they won't reappear.
3. Add more folders under **Libraries**.

## Please verify (could not be run where this was prepared)

`src-tauri` needs webkit2gtk 4.0 to build, which isn't available on the machine used here. Everything below the
Tauri layer is tested (91 Rust tests, `fmt`, `clippy -D warnings`; 32 frontend tests, `tsc`, ESLint, Prettier), and
the command code was type-checked against the real `atlas-catalog` API with a stand-in for Tauri, but the glue and
the CSP have not run in a real window. After building once (`pnpm tauri dev` and a release build):

- Open the window's developer tools and look for `Refused to ... because it violates the following Content
Security Policy` messages while you: browse the grid, open Quick Preview, open a PDF/SVG/video asset, use Compare,
  open Settings. If one appears, add the source it names to `csp` in `src-tauri/tauri.conf.json`.
  (`devCsp` is `null`, so `pnpm tauri dev` is not restricted; test the CSP with a build.)
- Add a second library, restart the app, and check both are there immediately.
- Unplug or rename a library folder and confirm its assets stay and the app reports it as offline.

## Optional hardening

In `src-tauri/tauri.conf.json` change `"assetScope": ["**"]` to `[]`. Every library folder is now allowed at
runtime (on start-up and when added), so previews keep working while the webview can no longer read arbitrary files.

## 7-Zip

Optional. Archive contents (and the content-based duplicate checks) come from Kaelovun's `.kaelovun.json` when
it exists; otherwise Evoury lists the archive with 7-Zip (`7z` on PATH, `C:\Program Files\7-Zip`, or `EVOURY_7Z`).
