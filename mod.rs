// Part of the APK.audio project — http://APK.audio — made by Anthony Kuzub
// MIT Licence. Free, for everyone, for ever. Full text in LICENSE at the root.
//! The four report importers, and the one record they all produce.
//!
//! Owns: reading a wireless-coordination report in someone else's format and
//! emitting the channel record the migration tabs render. Four sources —
//! `1_IAS` (HTML), `2_Soundbase` (CSV), `3_Wireless_Workbench` (.shw archive),
//! `4_CSV` (pattern-recognised CSV) — and one shape out of all of them.
//!
//! The rule the gate enforces: a JSON frame under
//! `APK:FrontEnd/Gui_Frames/3_Data_Migration/` declares a topic and a list of
//! headers, and the importer here must emit a field for every header it
//! declares. `check.sh datamigration` compares the two, and its lane comment
//! says why it has to: "A header with no field on the record renders as a blank
//! column: no error, no log line, and it reads as a broken parser."
//!
//! Must not: hardcode an absolute path to a fixture. The four `test_*.rs`
//! runners are `[[bin]]` targets of `oaRustCore` and resolve `test_data/`
//! through `env!("CARGO_MANIFEST_DIR")`. They previously named
//! `/home/anthony/.../APK:BareMetal/...`, a directory the rename removed, and
//! each printed the failure to stderr and exited 0 — so they could not have
//! failed a gate even while every one of them was reading nothing.

//! Data Migration Rust module aggregator.

#[path = "../../DATATYPE/DATATYPE.wireless.IAS/ias_importer.rs"]
pub mod ias_importer;

#[path = "../../DATATYPE/DATATYPE.wireless.SOUNDbase/soundbase_importer.rs"]
pub mod soundbase_importer;

#[path = "../../DATATYPE/DATATYPE.wireless.SHUREwirelessWorkbench/wwb_importer.rs"]
pub mod wwb_importer;

#[path = "../../DATATYPE/DATATYPE.wireless.CSV/csv_importer.rs"]
pub mod csv_importer;

#[path = "EDITOR.Data editor/data_editor.rs"]
pub mod data_editor;
