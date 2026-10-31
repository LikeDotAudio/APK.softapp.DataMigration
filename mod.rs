// Part of the APK.audio project — http://APK.audio — made by Anthony Kuzub
// MIT Licence. Free, for everyone, for ever. Full text in LICENSE at the root.
//! The four report importers, and the one record they all produce.
//!
//! Owns: reading a wireless-coordination report in someone else's format and
//! emitting the channel record the migration tabs render. Four sources —
//! `1_IAS` (HTML), `2_Soundbase` (CSV), `3_Wireless_Workbench` (.shw archive),
//! `4_CSV` (pattern-recognised CSV) — and one shape out of all of them.
//!
//! The rule the frames set: a JSON frame under
//! `PODS/POD.THIN/WEB.Frontend-Assets/SRC/FrontEnd/Gui_Frames/3_Data_Migration/`
//! declares a topic and a list of headers, and the importer here must emit a
//! field for every header it declares. A header with no field on the record
//! renders as a blank column: no error, no log line, and it reads as a broken
//! parser. `tests/frame_headers.rs` compares the two against each source's
//! fixture; the root `check.sh datamigration` lane runs it.
//!
//! Must not: compile another repository's file. Each importer is a crate its
//! DATATYPE.wireless.* repository owns (`SRC/importer/`), taken here by git rev
//! (`Cargo.toml`) and re-exported under its historical module name, so callers
//! (`oaRustCore::data_migration::csv_importer::…`) are unchanged. Until the
//! 2026-10-30 audit these were `#[path]` includes and the four `test_*` runners
//! were `[[bin]]`s of this crate built from those repositories' files; the runners
//! are now each repository's own `importer/tests/fixtures.rs`.

pub use apkaudio_wireless_ias as ias_importer;
pub use apkaudio_wireless_soundbase as soundbase_importer;
pub use apkaudio_wireless_wwb as wwb_importer;
pub use apkaudio_wireless_csv as csv_importer;

#[path = "EDITOR.Data editor/data_editor.rs"]
pub mod data_editor;
