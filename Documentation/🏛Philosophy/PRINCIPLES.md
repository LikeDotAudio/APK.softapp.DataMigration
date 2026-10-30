# Principles — MANAGEMENT.DataMigration

What the importer crate keeps, each with the code that keeps it. Ecosystem rules are the root's —
[Refusal over fabrication](../../../../../../../Documentation/%F0%9F%8F%9BPhilosophy/Doctrine/readme.Refusal%20over%20fabrication.md),
[Data handoff](../../../../../../../Documentation/%F0%9F%8F%9BPhilosophy/DATA%20HANDOFF%20PHILOSOPHY.md) — cited, not copied. `DATATYPE/…` means
`PODS/POD.THIN/Softapps/DATATYPE/`.

| Principle | Kept by |
|---|---|
| **Many formats in, one record out.** Four parsers (IAS HTML, Soundbase CSV, WWB `.shw`, pattern CSV), one channel record, one topic family. | `mod.rs` (`#[path]` modules); `DATATYPE/DATATYPE.wireless.*/*_importer.rs` |
| **Topics come from the contract.** Each importer calls `topics::data_migration(source, "ImportedData")`, never a typed string. | `ias_mqtt_topic()` (`DATATYPE.wireless.IAS/ias_importer.rs:36`) and its three siblings |
| **It reads; it addresses no hardware** — which is why the crate lives under `Softapps/`, not `BareMetal/Core/`. | `Cargo.toml` header |
| **Parsing does not depend on the bus.** The importers are pure — they parse and return; the orchestrator's daemon publishes — so a test run never writes the live bus. | each `*_importer.rs` header; `services/data_migration.rs` in the orchestrator |
| **A declared header is an emitted field.** Each `Gui_Frames/3_Data_Migration/` table's headers are checked against a record parsed from that source's fixture. | `tests/frame_headers.rs`; root `check.sh datamigration` |
| **Nothing is invented.** An empty cell stays empty; a channel takes the zone heading above its table. | tests `nothing_is_invented`, `an_empty_cell_stays_empty` (`ias_importer.rs:202`, :209) |
| **No absolute fixture paths; a missing fixture fails.** Runners resolve fixtures via `env!("CARGO_MANIFEST_DIR")`. | the four `DATATYPE/…/test_*.rs` (e.g. `test_soundbase_importer.rs:12`) |
| **Say what was done, or fail.** The data editor counts only records it wrote, rewrites by temp file + rename, and returns `Err` for an unknown op, a missing row, a duplicate append or a non-array file. | `process_data_editor_command()` (`EDITOR.Data editor/data_editor.rs:36`); tests `append_modify_delete_change_the_file`, `nothing_written_is_never_success` |
| **Its own workspace root** — no `[workspace]` key, so it never claims crates filed beside it. | `Cargo.toml` header |

## Where it does not hold

- **The root `display.json`** (`"entry": "main.py"`) is an old template card with nothing behind it.
- **`convert_marker_csv_to_mqtt_json`** splits on `,` with no quoting, and nothing in the tree calls it or the data editor.
