# DataMigration

> 📁 **Data Migration** — the Rust library that reads other people's wireless-coordination reports (IAS, Soundbase, Shure Wireless Workbench, pattern-recognised CSV) and turns each into one channel record on the bus.

---

## 🧩 Architectural Role
* **Family**: Soft App library (Rust crate `apkaudio-data-migration`, lib name `apkaudio_data_migration`)
* **Category**: `MANAGEMENT_AND_UTILITIES` — data migration / import
* **Pipeline Stage**: intake — someone else's report in, one channel record per row out (in the running system, each importer's POD.workers worker answers a job on `APK.audio/System/Protocols/wireless_<source>/incoming/result`)
* **Used by**: the `wireless_ias` / `wireless_soundbase` / `wireless_wwb` / `wireless_csv` workers (`../../DATATYPE/DATATYPE.wireless.*/SRC/`), which compile the same importer files; `oaRustCore` (`PODS/POD.THICK/NODE.BareMetal/SRC/Core/Cargo.toml`) still depends on it by path and re-exports it as `data_migration`
* **Rendered by**: the Data Migration frames in `PODS/POD.THIN/WEB.Frontend-Assets/SRC/FrontEnd/Gui_Frames/3_Data_Migration/`

---

## ⚙️ Repository Overview
A standalone Git repository mounted at `PODS/POD.THIN/Softapps/MANAGEMENT_AND_UTILITIES/MANAGEMENT.DataMigration`. The crate root is `mod.rs` (no `src/`), and most of its modules live in sibling `DATATYPE` repositories, reached with `#[path]`:

| Module | Source file | Reads | Its worker answers (not retained) on |
|---|---|---|---|
| `ias_importer` | `../../DATATYPE/DATATYPE.wireless.IAS/ias_importer.rs` | IAS HTML report | `APK.audio/System/Protocols/wireless_ias/incoming/result` |
| `soundbase_importer` | `../../DATATYPE/DATATYPE.wireless.SOUNDbase/soundbase_importer.rs` | Soundbase CSV | `…/wireless_soundbase/incoming/result` |
| `wwb_importer` | `../../DATATYPE/DATATYPE.wireless.SHUREwirelessWorkbench/wwb_importer.rs` | Wireless Workbench `.shw` archive | `…/wireless_wwb/incoming/result` |
| `csv_importer` | `../../DATATYPE/DATATYPE.wireless.CSV/SRC/csv_importer.rs` | pattern-recognised CSV | `…/wireless_csv/incoming/result` |
| `data_editor` | `EDITOR.Data editor/data_editor.rs` | a `DataEditCommand` against a JSON-array dataset (append / modify / delete by `"id"`); a marker CSV | nothing (writes the dataset file; returns values) |

The importers name no topic and publish nothing. Each Data Migration frame posts the chosen file as a job to its worker and draws the worker's answer; the orchestrator's DataMigration inbox (`…/DataMigration/<source>/FilePath` → `…/ImportedData`) is retired (LOOSE-datamigration-frames-to-workers). Their `mqtt_topic` fields and helpers, and the browser's `/Clear` → `ImportedData` handling, went with it (LOOSE-importeddata-dead-topics).

The four `test_*` runners are `[[bin]]` targets declared in `Cargo.toml`, with their source beside each importer (`test_ias_importer`, `test_soundbase_importer`, `test_wwb_importer`, `test_csv_pattern_importer`).

**Nested repositories** (registered in `.gitmodules`, each its own repo):
- `Documentation Management/` — only a `display.json` (the *Datasheets* frames card, group `Pipeline › 01 · Event`).
- `Download folder cleanup/` — `clean download.py`, a Tkinter tidier that archives a folder in place with an `undo_cleanup.sh` and a `--dry-run`; `test_clean_download.py` (6 pytest tests, passing).

---

## 🚀 Usage & Integration
* **Build**: `cargo build` in this directory (its own workspace root; no `[workspace]` key on purpose). Builds clean.
* **Runners**: `cargo run --bin test_ias_importer` (and the other three) — each parses the fixture in its `DATATYPE.wireless.*/test_data/` and exits 0; nothing is published.
* **Tests**: `cargo test` — the unit tests, and `tests/frame_headers.rs`: every header a `Gui_Frames/3_Data_Migration/` table declares is a field its importer emits (root `check.sh datamigration`).
* **Web Portal Entry**: none of its own. The root `display.json` (`"entry": "main.py"`) is an old template card with no `main.py` behind it; the UI is the frames listed above.
* **Remote Repository**: `https://github.com/LikeDotAudio/APK.softapp.DataMigration.git`

---

## 🏛 Philosophy
The principles the code follows, and where it shows them:

- **Many formats in, one record out.** Four parsers, one channel shape, — `mod.rs`, the four `*_importer.rs`
- **It reads; it addresses no hardware.** That is why the crate sits under `Softapps/` and not in `BareMetal/Core/`. — `Cargo.toml` (header)
- **Parsing does not depend on the bus.** The importers parse and return; their workers publish. — each `*_importer.rs` header
- **No absolute fixture paths.** Runners resolve fixtures through `env!("CARGO_MANIFEST_DIR")`, and report a missing file with a non-zero exit instead of exiting 0. — the four `test_*.rs`
- **Say what was done, or fail.** The data editor counts only records it wrote, rewrites a dataset by temp file + rename, and returns `Err` for an unknown operation, a missing row, a duplicate append or a file that is not a JSON array. — `process_data_editor_command` in `EDITOR.Data editor/data_editor.rs`
- **Where it does not hold:** listed in [PRINCIPLES](Documentation/%F0%9F%8F%9BPhilosophy/PRINCIPLES.md).

## 📚 Wiki

The depth is in [Documentation/README.md](Documentation/README.md): [principles](Documentation/%F0%9F%8F%9BPhilosophy/PRINCIPLES.md), [architecture](Documentation/%F0%9F%A7%A9Manual%20and%20Artifacts/ARCHITECTURE.md), [delivery](Documentation/%F0%9F%8C%8DPublishing/DELIVERY.md).

<!-- wiki:smoke -->
**Smoke test** — ✅ PASS 1/1 since 2026-10-30 18:07 EDT — [results](Documentation/%F0%9F%A7%A9Manual%20and%20Artifacts/SMOKE.md)
<!-- /wiki:smoke -->
