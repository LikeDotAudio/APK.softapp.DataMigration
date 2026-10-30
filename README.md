# DataMigration

> 📁 **Data Migration** — the Rust library that reads other people's wireless-coordination reports (IAS, Soundbase, Shure Wireless Workbench, pattern-recognised CSV) and turns each into one channel record on the bus.

---

## 🧩 Architectural Role
* **Family**: Soft App library (Rust crate `apkaudio-data-migration`, lib name `apkaudio_data_migration`)
* **Category**: `MANAGEMENT_AND_UTILITIES` — data migration / import
* **Pipeline Stage**: intake — someone else's report in, `APK.audio/System/DataMigration/<source>/ImportedData` out
* **Used by**: `oaRustCore` (`PODS/POD.THICK/NODE.BareMetal/SRC/Core/Cargo.toml`) depends on it by path and re-exports it as `data_migration`
* **Rendered by**: the Data Migration frames in `PODS/POD.THIN/WEB.Frontend-Assets/SRC/FrontEnd/Gui_Frames/3_Data_Migration/`

---

## ⚙️ Repository Overview
A standalone Git repository mounted at `PODS/POD.THIN/Softapps/MANAGEMENT_AND_UTILITIES/MANAGEMENT.DataMigration`. The crate root is `mod.rs` (no `src/`), and most of its modules live in sibling `DATATYPE` repositories, reached with `#[path]`:

| Module | Source file | Reads | Publishes (retained, QoS 1) |
|---|---|---|---|
| `ias_importer` | `../../DATATYPE/DATATYPE.wireless.IAS/ias_importer.rs` | IAS HTML report | `APK.audio/System/DataMigration/IAS/ImportedData` |
| `soundbase_importer` | `../../DATATYPE/DATATYPE.wireless.SOUNDbase/soundbase_importer.rs` | Soundbase CSV | `…/DataMigration/Soundbase/ImportedData` |
| `wwb_importer` | `../../DATATYPE/DATATYPE.wireless.SHUREwirelessWorkbench/wwb_importer.rs` | Wireless Workbench `.shw` archive | `…/DataMigration/WWB/ImportedData` |
| `csv_importer` | `../../DATATYPE/DATATYPE.wireless.CSV/csv_importer.rs` | pattern-recognised CSV | `…/DataMigration/CSV/ImportedData` |
| `data_editor` | `EDITOR.Data editor/data_editor.rs` | a `DataEditCommand` against a JSON-array dataset (append / modify / delete by `"id"`); a marker CSV | nothing (writes the dataset file; returns values) |

Topics are built from the contracts crate (`apkaudio_contracts::topics::data_migration`, `PODS/POD.THICK/NODE.BareMetal/SRC/contracts/rust`), never typed out. Each importer publishes to the loopback broker on the contract's default port, with the broker credentials from `apkaudio_contracts::broker_auth`.

The four `test_*` runners are `[[bin]]` targets declared in `Cargo.toml`, with their source beside each importer (`test_ias_importer`, `test_soundbase_importer`, `test_wwb_importer`, `test_csv_pattern_importer`).

**Nested repositories** (registered in `.gitmodules`, each its own repo):
- `Documentation Management/` — only a `display.json` (the *Datasheets* frames card, group `Pipeline › 01 · Event`).
- `Download folder cleanup/` — `clean download.py`, a Tkinter tidier that archives a folder in place with an `undo_cleanup.sh` and a `--dry-run`; `test_clean_download.py` (6 pytest tests, passing).

---

## 🚀 Usage & Integration
* **Build**: `cargo build` in this directory (its own workspace root; no `[workspace]` key on purpose). Builds clean.
* **Runners**: `cargo run --bin test_ias_importer` (and the other three) — each parses the fixture in its `DATATYPE.wireless.*/test_data/`, publishes, and exits 0 (2026-10-30).
* **Tests**: `cargo test --lib` — the data editor's two unit tests.
* **Web Portal Entry**: none of its own. The root `display.json` (`"entry": "main.py"`) is an old template card with no `main.py` behind it; the UI is the frames listed above.
* **Remote Repository**: `https://github.com/LikeDotAudio/APK.softapp.DataMigration.git`

---

## 🏛 Philosophy
The principles the code follows, and where it shows them:

- **Many formats in, one record out.** Four parsers, one channel shape, one topic family. — `mod.rs`, the four `*_importer.rs`
- **Topics come from the contract.** Every importer calls `topics::data_migration(source, "ImportedData")` instead of spelling the string. — `*_importer.rs` (`*_mqtt_topic`)
- **It reads; it addresses no hardware.** That is why the crate sits under `Softapps/` and not in `BareMetal/Core/`. — `Cargo.toml` (header)
- **Keep going on an error.** Parsing does not depend on the bus: the publish waits a bounded time for a broker (5 s for IAS and WWB, 3 s for Soundbase and CSV) and the parsed result is returned whether or not anything acked. — `publish_to_mqtt` in each `*_importer.rs`
- **No absolute fixture paths.** Runners resolve fixtures through `env!("CARGO_MANIFEST_DIR")`, and report a missing file with a non-zero exit instead of exiting 0. — the four `test_*.rs`
- **Say what was done, or fail.** The data editor counts only records it wrote, rewrites a dataset by temp file + rename, and returns `Err` for an unknown operation, a missing row, a duplicate append or a file that is not a JSON array. — `process_data_editor_command` in `EDITOR.Data editor/data_editor.rs`
- **Where it does not hold:**
  - Nothing compares a frame's declared headers with the fields its importer emits; the `check.sh datamigration` lane that did is gone with `check.sh`.
  - The runners publish retained records to the live broker as a side effect of a test run.

## 📚 Wiki

The depth is in [Documentation/README.md](Documentation/README.md): [principles](Documentation/%F0%9F%8F%9BPhilosophy/PRINCIPLES.md), [architecture](Documentation/%F0%9F%A7%A9Manual%20and%20Artifacts/ARCHITECTURE.md), [delivery](Documentation/%F0%9F%8C%8DPublishing/DELIVERY.md).

<!-- wiki:smoke -->
**Smoke test** — ✅ PASS 1/1 since 2026-10-30 12:51 EDT — [results](Documentation/%F0%9F%A7%A9Manual%20and%20Artifacts/SMOKE.md)
<!-- /wiki:smoke -->
