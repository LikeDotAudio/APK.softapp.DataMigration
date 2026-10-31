# Architecture — MANAGEMENT.DataMigration

## Use

```
cargo build                                  # this directory is its own workspace root
cargo test --locked                          # data editor unit tests + tests/frame_headers.rs
```

(Run from this directory: the git-excluded `.cargo/config.toml` here patches the four importer
crates to the checkout. Each importer's own fixture test runs in its repository:
`bash DATATYPE/DATATYPE.wireless.<X>/SRC/check.sh`.)

In the running system the importers run inside their POD.workers workers
(`DATATYPE/DATATYPE.wireless.{IAS,SOUNDbase,SHUREwirelessWorkbench,CSV}/SRC/`, tokens `wireless_ias`,
`wireless_soundbase`, `wireless_wwb`, `wireless_csv`). Each repository owns its parser as a crate
(`SRC/importer/`): its worker links it by path, this crate by git rev — one owner per importer.
`PODS/POD.THICK/NODE.BareMetal/SRC/Core/Cargo.toml` takes this crate by git rev (patched to the checkout) and
`oaRustCore` re-exports it as `data_migration` (the CSV parser reads it there). Dependencies run one way:
Core → this crate → the four importer crates → the wireless kernel; this crate names nothing of NODE.BareMetal. The orchestrator's `services::data_migration` inbox on
`APK.audio/System/DataMigration/+/FilePath` is retired (LOOSE-datamigration-frames-to-workers).

## Data flow

```
frame upload (WEB.Frontend-Assets/SRC/FrontEnd/Gui_Frames/3_Data_Migration/{1_IAS,2_Soundbase,3_Wireless_Workbench,4_CSV})
   └─ the browser reads the file and posts a job, not retained:
      APK.audio/System/Protocols/wireless_<source>/outgoing/job   {job_id, filename, text | content_b64}
        └─ the wireless_<source> worker (its own container)
             ├─ <source>_importer::classify_*()  — pure: parses, returns, publishes nothing
             └─ the worker answers {schema, job_id, ok, result, error}, not retained, on
                APK.audio/System/Protocols/wireless_<source>/incoming/result
                   └─ the frame's table draws result.channels / result.records under its declared headers
                      (tests/frame_headers.rs: every header is a field)
```

## Modules

| Module | Re-export of (crate, by git rev) | Reads | Its worker answers on |
|---|---|---|---|
| `ias_importer` | `apkaudio-wireless-ias` (DATATYPE.wireless.IAS/SRC/importer) | IAS HTML report | `…/wireless_ias/incoming/result` |
| `soundbase_importer` | `apkaudio-wireless-soundbase` (DATATYPE.wireless.SOUNDbase/SRC/importer) | Soundbase CSV | `…/wireless_soundbase/incoming/result` |
| `wwb_importer` | `apkaudio-wireless-wwb` (DATATYPE.wireless.SHUREwirelessWorkbench/SRC/importer) | WWB `.shw` (zip) | `…/wireless_wwb/incoming/result` |
| `csv_importer` | `apkaudio-wireless-csv` (DATATYPE.wireless.CSV/SRC/importer) | pattern-recognised CSV | `…/wireless_csv/incoming/result` |
| `data_editor` | this repo: `EDITOR.Data editor/data_editor.rs` (`#[path]`, its own file) | `DataEditCommand` on a JSON-array dataset; a marker CSV | writes the dataset file |

### The data editor

`process_data_editor_command(DataEditCommand { target_file, operation, row_id, payload })`:

| `operation` | Effect | Error when |
|---|---|---|
| `append` | push `payload` (an object; `"id"` set to `row_id` if absent) | a record with that id exists; payload not an object |
| `modify` | merge `payload`'s fields into the record whose `"id"` = `row_id` | no such record; payload not an object |
| `delete` | remove that record | no such record |

The file must be a JSON array; ids match as string or number. The file is rewritten via `<name>.edit.tmp` + rename.
`records_modified` is `1` on success.

## Nested repositories

- `Documentation Management/` — a `display.json` only (the *Datasheets* frames card, `Pipeline › 01 · Event`).
- `Download folder cleanup/` — `clean download.py`, a Tkinter tidier with `--dry-run` and an `undo_cleanup.sh`; `test_clean_download.py`.
