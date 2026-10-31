# 🔍 Architecture audit — MANAGEMENT.DataMigration (2026-10-30)

Audited against the APK.audio multi-repo architecture standard: coupling, boundaries, duplication, contracts.
← [Audits](INDEX.md) · Improvements: [🛣Roadmap/IMPROVEMENTS.md](../%F0%9F%9B%A3Roadmap/IMPROVEMENTS.md)

## Bounded context
Claims to own "someone else's wireless-coordination report in, one channel record out" (IAS, Soundbase, Shure WWB, pattern CSV) plus a JSON-array data editor. Rust crate `apkaudio-data-migration`; the repo itself holds only `mod.rs` (41 lines), `EDITOR.Data editor/data_editor.rs` (170), `tests/frame_headers.rs` (137) and two nested submodules. Everything tracked was read.

## Context map
| Talks to | How (topic / endpoint / file / import) | Relationship |
|---|---|---|
| DATATYPE.wireless.IAS / .SOUNDbase / .SHUREwirelessWorkbench / .CSV | depends on their importer crates (`SRC/importer/`) by git rev; re-exported in `mod.rs` | Customer of versioned crates |
| WEB.Frontend-Assets `Gui_Frames/3_Data_Migration/` | `tests/frame_headers.rs` reads the frame JSON from the checkout and checks every header against the importer's record | Consumer-driven contract (kept by design) |
| NODE.BareMetal `oaRustCore` | depends on this crate by git rev (patched locally) and re-exports it as `data_migration` | Customer by version, one-way |
| `Documentation Management/`, `Download folder cleanup/` | nested submodules (`.gitmodules`) — audited in their own repos | — |

## Scorecard
| Category | Score (0–3) | Finding |
|---|---|---|
| Coupling & Cohesion | 3 | No other repository's file compiled here; four importer crates by rev, one owner each; no dependency on NODE.BareMetal |
| Boundary Integrity | 2 | importers parse and return records, publish nothing; the bus edge is the workers' |
| Duplication Control | 3 | each importer has one build owner; this crate only re-exports |
| Testing & Contracts | 3 | `cargo test --locked` runs the data editor's tests and the frame-headers contract; each importer's tests run at home |
| Change management | 3 | `0.2.0`, importers pinned by pushed revs that resolve from GitHub |
Score key: 3 meets the standard · 2 minor gaps · 1 significant gaps · 0 absent / violated.

## Findings
1. **Good: a real consumer-driven contract.** `tests/frame_headers.rs` parses each source's fixture and asserts every `headers` entry declared by the `Gui_Frames/3_Data_Migration/` tables is a field on the record. It reads the frames and fixtures from the checkout — kept by design (PRINCIPLES): it is checked against the real consumer.
2. **Good: retired topics removed.** The orchestrator inbox and its `mqtt_topic` helpers were dropped (LOOSE-importeddata-dead-topics); the importers name no topic.
3. **Good: the editor fails loudly.** `EDITOR.Data editor/data_editor.rs` returns `Err` for an unknown op, missing row, duplicate append or non-array file and writes by temp file + rename.
4. **Good: builds standalone.** `git archive HEAD` built outside the monorepo resolves the four pinned importer revs (and the kernel) from GitHub.
