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

/// Every `(topic, headers)` table in a frame file.
fn tables(v: &Value, out: &mut Vec<(String, Vec<String>)>) {
    match v {
        Value::Object(map) => {
            if let Some(Value::Array(headers)) = map.get("headers") {
                let topic = map.get("topic").and_then(Value::as_str).unwrap_or_default().to_string();
                let headers = headers.iter().filter_map(Value::as_str).map(str::to_string).collect();
                out.push((topic, headers));
            }
            map.values().for_each(|child| tables(child, out));
        }
        Value::Array(items) => items.iter().for_each(|child| tables(child, out)),
        _ => {}
    }
}

fn frame_table(frame: &str, topic: &str) -> Vec<String> {
    let path = frames_dir().join(frame);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let json: Value = serde_json::from_str(&text).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let mut found = Vec::new();
    tables(&json, &mut found);
    found
        .into_iter()
        .find(|(t, _)| t == topic)
        .map(|(_, h)| h)
        .unwrap_or_else(|| panic!("{frame} has no table on {topic}; found {:?}", {
            let mut f = Vec::new();
            tables(&json, &mut f);
            f.into_iter().map(|(t, _)| t).collect::<Vec<_>>()
        }))
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

fn to_value<T: serde::Serialize>(v: T) -> Value {
    serde_json::to_value(v).expect("a record serialises")
}

#[test]
fn ias_frame_headers_are_ias_fields() {
    let r = ias_importer::parse_ias_report(fixture("DATATYPE.wireless.IAS", "report.html")).unwrap();
    assert_frame_matches("1_IAS/IAS_Importer.json", &ias_importer::ias_mqtt_topic(), to_value(r.channels));
}

#[test]
fn soundbase_frame_headers_are_soundbase_fields() {
    let r = soundbase_importer::parse_soundbase_report(fixture("DATATYPE.wireless.SOUNDbase", "SB PDF.csv")).unwrap();
    assert_frame_matches(
        "2_Soundbase/Soundbase_Importer.json",
        &soundbase_importer::soundbase_mqtt_topic(),
        to_value(r.channels),
    );
}

#[test]
fn wwb_frame_headers_are_wwb_fields() {
    let r = wwb_importer::parse_wwb_file(fixture("DATATYPE.wireless.SHUREwirelessWorkbench", "venue.shw")).unwrap();
    assert_frame_matches(
        "3_Wireless_Workbench/Wireless_Workbench_Importer.json",
        &wwb_importer::wwb_mqtt_topic(),
        to_value(r.channels),
    );
}

/// The CSV frame reads the wireless_csv worker's answer, whose `result` is this
/// importer's `CsvRecognizedResult` (DATATYPE.wireless.CSV/SRC/src/lib.rs).
#[test]
fn csv_frame_headers_are_csv_record_fields() {
    let r = csv_importer::parse_and_classify_csv(fixture("DATATYPE.wireless.CSV", "venue.csv")).unwrap();
    assert_frame_matches(
        "4_CSV/CSV_Importer.json",
        "APK.audio/System/Protocols/wireless_csv/incoming/result",
        to_value(r.records),
    );
}

/// The check itself: a header with no field is reported, not passed.
#[test]
fn a_header_with_no_field_is_caught() {
    let records = serde_json::json!([{ "name": "a", "zone": "b" }]);
    let headers = vec!["name".to_string(), "zone".to_string(), "band_info".to_string()];
    assert_eq!(missing(&headers, &records), vec!["band_info".to_string()]);
}
