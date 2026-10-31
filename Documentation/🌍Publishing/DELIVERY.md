# Delivery — MANAGEMENT.DataMigration

## Build

`cargo build` here (Rust 2021; deps the four importer crates by git rev, `serde`, `serde_json`). In the monorepo the git-excluded `.cargo/config.toml` patches the importers to the checkout.

## Test

- `cargo test --locked` (from this directory) — the data editor's unit tests and `tests/frame_headers.rs`: every
  header the four `Gui_Frames/3_Data_Migration/` tables declare is a field of a record parsed from that source's
  fixture (2 + 5 passed 2026-10-30). This is the one local check; the root CI verifies nothing on purpose.
- The importers' own tests run in their repositories (`bash DATATYPE/DATATYPE.wireless.<X>/SRC/check.sh`).
- Monorepo smoke run: [SMOKE.md](../%F0%9F%A7%A9Manual%20and%20Artifacts/SMOKE.md).

## Ship

- **Repository**: `https://github.com/LikeDotAudio/APK.softapp.DataMigration` — a submodule at
  `PODS/POD.THIN/Softapps/MANAGEMENT_AND_UTILITIES/MANAGEMENT.DataMigration`, with two nested submodules
  (`Documentation Management`, `Download folder cleanup`). No GitHub workflow.
- **Where it lands**: compiled into the BareMetal node. `PODS/POD.THICK/NODE.BareMetal/Docker/Dockerfile` copies
  this repository and the four DATATYPE.wireless.* repositories so Core's `[patch]` entries resolve them locally. The
  importers that answer the frames run in the `wireless_*` worker images (`DATATYPE.wireless.*/Docker/`), not in
  the orchestrator, whose DataMigration inbox is retired.
- **Web images**: `web-static` copies the Softapps tree with the `.rs` and `Cargo.*` files in it, and its nginx
  answers `*.rs`, `*.toml`, `*.lock` with 404 (`WEB.Frontend-Assets/Docker/nginx.conf`); `*.py` under Softapps —
  `clean download.py` — never reaches any image (root `.dockerignore`, PLAN-3422.01).

## After an edit

Rebuild the node image (`up-stack.sh` for NODE.BareMetal). Push and pointer bump: `.apk.scripts/deploy_all.py`.

## Not automated

- No CI runs `cargo test` (by design at the root); it is run by hand before a push.

## Delivered from the 2026-10-30 audit

- **One owner per importer** — the four `#[path]` includes of DATATYPE files are gone: `mod.rs` re-exports `apkaudio-wireless-{ias,soundbase,wwb,csv}`, crates owned by their DATATYPE.wireless.* repositories (`SRC/importer/`) and taken here by git rev; their workers link the same crates by path. `oaRustCore::data_migration::…` paths are unchanged. Verified by `cargo test --locked` (data editor 2, frame_headers 5).
- **Runners moved home** — the four `[[bin]] test_*` runners built from other repositories' files are gone; each is `importer/tests/fixtures.rs` in its own repository, fixture path from its own manifest.
- **No dependency on NODE.BareMetal** — the manifest names no BareMetal crate (the dependency runs Core → this crate only). NODE.BareMetal's side of the cycle (Core by git rev + `[patch]`) was done by W04; handoff filed for its new `[patch]` entries for the importer crates.
- **oaRustCore takes this crate by version** — NODE.BareMetal `SRC/Core/Cargo.toml` names `APK.softapp.DataMigration` by `rev` with a local `[patch]` (W04, 2026-10-30).
- **Real display.json** — name "Data Migration", a description of the crate, `entry: mod.rs`; README's `main.py` line fixed.
- **Frame contract test** — kept and run by `cargo test` here (one local check). Kept by design: it reads the frames and fixtures from the checkout, because a consumer-driven contract is checked against the real consumer, not a copied header list.
