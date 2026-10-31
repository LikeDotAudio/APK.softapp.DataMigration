# Principles — MANAGEMENT.DataMigration

What the importer crate keeps, each with the code that keeps it. Ecosystem rules are the root's —
[Refusal over fabrication](../../../../../../../Documentation/%F0%9F%8F%9BPhilosophy/Doctrine/readme.Refusal%20over%20fabrication.md),
[Data handoff](../../../../../../../Documentation/%F0%9F%8F%9BPhilosophy/DATA%20HANDOFF%20PHILOSOPHY.md) — cited, not copied. `DATATYPE/…` means
`PODS/POD.THIN/Softapps/DATATYPE/`.

| Principle | Kept by |
|---|---|
| **Many formats in, one record out.** Four parsers (IAS HTML, Soundbase CSV, WWB `.shw`, pattern CSV), one channel record. | `mod.rs` (`#[path]` modules); `DATATYPE/DATATYPE.wireless.*/*_importer.rs` |
| **It reads; it addresses no hardware** — which is why the crate lives under `Softapps/`, not `BareMetal/Core/`. | `Cargo.toml` header |
| **Parsing does not depend on the bus.** The importers are pure — they parse and return; their POD.workers workers publish — so a test run never writes the live bus. | each `*_importer.rs` header; `handle_job` in `DATATYPE/DATATYPE.wireless.*/SRC/src/lib.rs` |
| **A declared header is an emitted field.** Each `Gui_Frames/3_Data_Migration/` table's headers are checked against a record parsed from that source's fixture. | `tests/frame_headers.rs`; root `check.sh datamigration` |
| **Nothing is invented.** An empty cell stays empty; a channel takes the zone heading above its table. | tests `nothing_is_invented`, `an_empty_cell_stays_empty` (`ias_importer.rs:202`, :209) |
| **One owner per importer.** Each importer is a crate its DATATYPE repository owns; this crate depends on the four by git rev and compiles no other repository's file. | `Cargo.toml` `[dependencies]`; `mod.rs` re-exports |
| **A missing fixture fails.** Each importer's fixture test resolves its fixture from its own crate (`env!("CARGO_MANIFEST_DIR")`) in its own repository. | `DATATYPE/…/SRC/importer/tests/fixtures.rs` |
| **Kept by design: the frame contract test reads the consumer's files.** `tests/frame_headers.rs` reads the Data Migration frames and the importers' fixtures from the monorepo checkout — a consumer-driven contract test is only meaningful against the real consumer, so it runs in a full checkout (by `cargo test` here) rather than against a copied header list that could drift. | `tests/frame_headers.rs` |
| **Say what was done, or fail.** The data editor counts only records it wrote, rewrites by temp file + rename, and returns `Err` for an unknown op, a missing row, a duplicate append or a non-array file. | `process_data_editor_command()` (`EDITOR.Data editor/data_editor.rs:36`); tests `append_modify_delete_change_the_file`, `nothing_written_is_never_success` |
| **Its own workspace root** — no `[workspace]` key, so it never claims crates filed beside it. | `Cargo.toml` header |

## Where it does not hold

- **`convert_marker_csv_to_mqtt_json`** splits on `,` with no quoting, and nothing in the tree calls it or the data editor.
