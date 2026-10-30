# Delivery — MANAGEMENT.DataMigration

## Build

`cargo build` here (Rust 2021; deps `apkaudio-contracts` by path, `serde`, `csv`, `scraper`, `zip`).

## Test

- `cargo test --lib` — the data editor's and the importers' unit tests.
- `cargo test --test frame_headers` — every header the four `Gui_Frames/3_Data_Migration/` tables declare is a
  field of a record parsed from that source's fixture; root `.apk.scripts/check.sh datamigration` runs it.
- `cargo run --bin test_*` — the four runners each parse their `DATATYPE.wireless.*/test_data/` fixture and exit 0;
  the importers publish nothing, so a run never touches the broker.
- The importers' own unit tests sit in the `DATATYPE` files and run with the crate.
- Monorepo smoke run: [SMOKE.md](../%F0%9F%A7%A9Manual%20and%20Artifacts/SMOKE.md).

## Ship

- **Repository**: `https://github.com/LikeDotAudio/APK.softapp.DataMigration` — a submodule at
  `PODS/POD.THIN/Softapps/MANAGEMENT_AND_UTILITIES/MANAGEMENT.DataMigration`, with two nested submodules
  (`Documentation Management`, `Download folder cleanup`). No GitHub workflow.
- **Where it lands**: compiled into the BareMetal node. `PODS/POD.THICK/NODE.BareMetal/Docker/Dockerfile` copies
  `PODS/POD.THIN/Softapps` to its real relative position so `Core/Cargo.toml`'s path dependency resolves, and the
  importers run inside the orchestrator binary.
- **Web images**: `web-static` copies the Softapps tree with the `.rs` and `Cargo.*` files in it, and its nginx
  answers `*.rs`, `*.toml`, `*.lock` with 404 (`WEB.Frontend-Assets/Docker/nginx.conf`); `*.py` under Softapps —
  `clean download.py` — never reaches any image (root `.dockerignore`, PLAN-3422.01).

## After an edit

Rebuild the node image (`up-stack.sh` for NODE.BareMetal). Push and pointer bump: `.apk.scripts/deploy_all.py`.

## Not automated

- No CI runs `cargo test` or the runners; the header check runs in the local `check.sh datamigration` lane.
