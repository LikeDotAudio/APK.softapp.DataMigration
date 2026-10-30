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
| `data_editor` | `EDITOR.Data editor/data_editor.rs` | a `DataEditCommand`; a marker CSV | nothing (returns values) |

Topics are built from the contracts crate (`apkaudio_contracts::topics::data_migration`, `PODS/POD.THICK/NODE.BareMetal/SRC/contracts/rust`), never typed out. Each importer publishes to the loopback broker on the contract's default port, with the broker credentials from `apkaudio_contracts::broker_auth`.

The four `test_*` runners are `[[bin]]` targets declared in `Cargo.toml`, with their source beside each importer (`test_ias_importer`, `test_soundbase_importer`, `test_wwb_importer`, `test_csv_pattern_importer`).

**Nested repositories** (registered in `.gitmodules`, each its own repo):
- `Documentation Management/` — only a `display.json` (the *Datasheets* frames card, group `Pipeline › 01 · Event`).
- `Download folder cleanup/` — `clean download.py`, a Tkinter tidier that archives a folder in place with an `undo_cleanup.sh` and a `--dry-run`; `test_clean_download.py` (6 pytest tests, passing).

---

## 🚀 Usage & Integration
* **Build**: `cargo build` in this directory (its own workspace root; no `[workspace]` key on purpose). Builds clean.
* **Runners**: `cargo run --bin test_ias_importer` (and the other three). Today all four exit non-zero — see below.
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
- **Where it does not hold:**
  - The runners look for fixtures at `<this crate>/DataMigration/<N_source>/test_data/…`, which does not exist; the fixtures are in each `DATATYPE.wireless.*/test_data/`. All four runners fail on a missing file (exit 1, 1, 1, 101).
  - `data_editor::process_data_editor_command` writes nothing: it reports `status: "success"` and `records_modified: 1` for `append` / `modify` without touching the file — against *Refusal over fabrication*. Nothing in the tree calls it.
  - `mod.rs`'s header still says the runners are `[[bin]]` targets of `oaRustCore` and names `APK:FrontEnd/Gui_Frames/3_Data_Migration/`; both have moved (runners are this crate's bins; frames are under `WEB.Frontend-Assets/SRC/FrontEnd/`).
