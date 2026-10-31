// Part of the APK.audio project — http://APK.audio — made by Anthony Kuzub
// MIT Licence. Free, for everyone, for ever. Full text in LICENSE at the root.
//! Every header a Data Migration frame declares is a field its importer emits.
//!
//! The frames under `WEB.Frontend-Assets/SRC/FrontEnd/Gui_Frames/3_Data_Migration/`
//! name a table topic and a list of `headers`; the table draws one column per
//! header from the records on that topic. A header with no field on the record
//! renders as a blank column — no error, no log line, and it reads as a broken
//! parser. The `check.sh datamigration` lane that compared the two went with
//! check.sh (LOOSE-management-datamigration-nothing-checks-that); this is that
//! comparison, run by `cargo test`, against a record parsed from each source's
//! own fixture.

use std::path::PathBuf;

use apkaudio_data_migration::{csv_importer, ias_importer, soundbase_importer, wwb_importer};
use serde_json::Value;

fn here() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn frames_dir() -> PathBuf {
    here().join("../../../WEB.Frontend-Assets/SRC/FrontEnd/Gui_Frames/3_Data_Migration")
}

fn fixture(repo: &str, name: &str) -> PathBuf {
    here().join("../../DATATYPE").join(repo).join("test_data").join(name)
}

/// One table a frame declares: the topic it listens on, the `also_topics` it
/// draws the same rows from, and its headers.
struct Table {
    topic: String,
    also_topics: Vec<String>,
    headers: Vec<String>,
}

impl Table {
    /// Every topic whose records fill this table's columns.
    fn topics(&self) -> impl Iterator<Item = &str> {
        std::iter::once(self.topic.as_str()).chain(self.also_topics.iter().map(String::as_str))
    }
}

fn strings(v: Option<&Value>) -> Vec<String> {
    v.and_then(Value::as_array)
        .map(|a| a.iter().filter_map(Value::as_str).map(str::to_string).collect())
        .unwrap_or_default()
}

/// Every table in a frame file.
fn tables(v: &Value, out: &mut Vec<Table>) {
    match v {
        Value::Object(map) => {
            if let Some(Value::Array(_)) = map.get("headers") {
                out.push(Table {
                    topic: map.get("topic").and_then(Value::as_str).unwrap_or_default().to_string(),
                    also_topics: strings(map.get("also_topics")),
                    headers: strings(map.get("headers")),
                });
            }
            map.values().for_each(|child| tables(child, out));
        }
        Value::Array(items) => items.iter().for_each(|child| tables(child, out)),
        _ => {}
    }
}

fn frame_tables(frame: &str) -> Vec<Table> {
    let path = frames_dir().join(frame);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let json: Value = serde_json::from_str(&text).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let mut found = Vec::new();
    tables(&json, &mut found);
    found
}

/// The headers of the table that draws `topic`, as its `topic` or one of its
/// `also_topics`.
fn frame_table(frame: &str, topic: &str) -> Vec<String> {
    let found = frame_tables(frame);
    let topics: Vec<String> = found.iter().flat_map(|t| t.topics().map(str::to_string)).collect();
    found
        .into_iter()
        .find(|t| t.topics().any(|x| x == topic))
        .map(|t| t.headers)
        .unwrap_or_else(|| panic!("{frame} has no table on {topic}; found {topics:?}"))
}

/// The headers that name no field of the first record.
fn missing(headers: &[String], records: &Value) -> Vec<String> {
    let first = records
        .as_array()
        .and_then(|r| r.first())
        .and_then(Value::as_object)
        .expect("the fixture yields at least one record");
    headers.iter().filter(|h| !first.contains_key(h.as_str())).cloned().collect()
}

fn assert_frame_matches(frame: &str, topic: &str, records: Value) {
    let headers = frame_table(frame, topic);
    assert!(!headers.is_empty(), "{frame}: the table on {topic} declares no headers");
    let absent = missing(&headers, &records);
    assert!(absent.is_empty(), "{frame} declares {absent:?}, which its importer never emits — blank columns");
}

/// The topic a POD.workers worker answers on, which is where each frame's
/// table listens since the frames post their file as a job
/// (LOOSE-datamigration-frames-to-workers). The worker's envelope carries the
/// importer's own result, so the records checked here are the ones it draws.
fn worker_result(token: &str) -> String {
    format!("APK.audio/System/Protocols/{token}/incoming/result")
}

fn to_value<T: serde::Serialize>(v: T) -> Value {
    serde_json::to_value(v).expect("a record serialises")
}

/// The worker token in a `…/Protocols/<token>/incoming/result` topic.
fn worker_of(topic: &str) -> Option<&str> {
    topic.strip_prefix("APK.audio/System/Protocols/")?.strip_suffix("/incoming/result")
}

/// The records a worker publishes, parsed from its importer's own fixture.
/// `None` for a worker this test does not know — which fails
/// `every_table_topic_is_a_checked_worker`, so a new topic cannot slip past.
fn worker_records(token: &str) -> Option<Value> {
    Some(match token {
        "wireless_ias" => {
            to_value(ias_importer::parse_ias_report(fixture("DATATYPE.wireless.IAS", "report.html")).unwrap().channels)
        }
        "wireless_soundbase" => to_value(
            soundbase_importer::parse_soundbase_report(fixture("DATATYPE.wireless.SOUNDbase", "SB PDF.csv"))
                .unwrap()
                .channels,
        ),
        "wireless_pdf" => to_value(
            apkaudio_wireless_soundbase_pdf::parse_soundbase_pdf_file(pdf_fixture("SB PDF.pdf")).unwrap().channels,
        ),
        "wireless_wwb" => to_value(
            wwb_importer::parse_wwb_file(fixture("DATATYPE.wireless.SHUREwirelessWorkbench", "venue.shw"))
                .unwrap()
                .channels,
        ),
        "wireless_csv" => {
            to_value(csv_importer::parse_and_classify_csv(fixture("DATATYPE.wireless.CSV", "venue.csv")).unwrap().records)
        }
        _ => return None,
    })
}

/// DATATYPE.wireless.PDF keeps its fixture beside its importer crate.
fn pdf_fixture(name: &str) -> PathBuf {
    here().join("../../DATATYPE/DATATYPE.wireless.PDF/SRC/importer/fixtures").join(name)
}

/// Every worker-result topic of every Data Migration table — `topic` and each
/// of its `also_topics` — names a worker this test parses, and the table's
/// headers are fields of that worker's record.
#[test]
fn every_table_topic_is_a_checked_worker() {
    let mut frames: Vec<PathBuf> = std::fs::read_dir(frames_dir())
        .expect("the Data Migration frames")
        .flatten()
        .filter(|e| e.path().is_dir())
        .flat_map(|d| std::fs::read_dir(d.path()).into_iter().flatten().flatten())
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    frames.sort();
    let mut checked = std::collections::BTreeSet::new();
    for path in &frames {
        let frame = path.strip_prefix(frames_dir()).unwrap().to_string_lossy().to_string();
        for table in frame_tables(&frame) {
            for topic in table.topics() {
                // Not every Data Migration frame is an importer (the
                // Repository Manager's table is its own topic); only a
                // worker's result is a parsed record.
                let Some(token) = worker_of(topic) else { continue };
                let records =
                    worker_records(token).unwrap_or_else(|| panic!("{frame}: no importer is checked for worker {token}"));
                let absent = missing(&table.headers, &records);
                assert!(absent.is_empty(), "{frame} declares {absent:?}, which {token} never emits — blank columns");
                checked.insert(token.to_string());
            }
        }
    }
    // The walker sees `also_topics`: wireless_pdf is reached only through the
    // SoundBase table's.
    let expected = ["wireless_csv", "wireless_ias", "wireless_pdf", "wireless_soundbase", "wireless_wwb"];
    assert_eq!(checked.iter().map(String::as_str).collect::<Vec<_>>(), expected);
}

/// One frame's table, checked against one worker's records.
fn check(frame: &str, token: &str) {
    assert_frame_matches(frame, &worker_result(token), worker_records(token).expect("a checked worker"));
}

#[test]
fn ias_frame_headers_are_ias_fields() {
    check("1_IAS/IAS_Importer.json", "wireless_ias");
}

#[test]
fn soundbase_frame_headers_are_soundbase_fields() {
    check("2_Soundbase/Soundbase_Importer.json", "wireless_soundbase");
}

/// The SoundBase table also draws the wireless_pdf worker's records (its
/// `also_topics`): a PDF report fills the same columns as the CSV export.
#[test]
fn soundbase_frame_headers_are_soundbase_pdf_fields() {
    check("2_Soundbase/Soundbase_Importer.json", "wireless_pdf");
}

#[test]
fn wwb_frame_headers_are_wwb_fields() {
    check("3_Wireless_Workbench/Wireless_Workbench_Importer.json", "wireless_wwb");
}

/// The wireless_csv worker's `result` is this importer's `CsvRecognizedResult`
/// (DATATYPE.wireless.CSV/SRC/src/lib.rs).
#[test]
fn csv_frame_headers_are_csv_record_fields() {
    check("4_CSV/CSV_Importer.json", "wireless_csv");
}

/// The check itself: a header with no field is reported, not passed.
#[test]
fn a_header_with_no_field_is_caught() {
    let records = serde_json::json!([{ "name": "a", "zone": "b" }]);
    let headers = vec!["name".to_string(), "zone".to_string(), "band_info".to_string()];
    assert_eq!(missing(&headers, &records), vec!["band_info".to_string()]);
}
