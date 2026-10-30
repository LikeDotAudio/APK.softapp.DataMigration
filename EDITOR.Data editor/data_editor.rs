// Part of the APK.audio project — http://APK.audio — made by Anthony Kuzub
// MIT Licence. Free, for everyone, for ever. Full text in LICENSE at the root.
//! Data Editor, Appender & Marker Converter written in Rust.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataEditCommand {
    pub target_file: String,
    pub operation: String, // "append", "modify", "delete"
    pub row_id: String,
    pub payload: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataEditResult {
    pub status: String,
    pub target_file: String,
    pub records_modified: usize,
}

/// The field a record is found by. A dataset is a JSON array of objects and
/// `row_id` names the record whose `"id"` equals it.
const ID_FIELD: &str = "id";

/// Applies an append, modify or delete to a JSON-array dataset on disk.
///
/// Every outcome is the file's real state: `records_modified` counts records
/// actually written, the file is rewritten (temp file + rename, so a crash never
/// leaves half a dataset) only when something changed, and anything this cannot
/// do — a missing file, a file that is not a JSON array, an unknown operation,
/// a `modify`/`delete` of a row that is not there, a duplicate `append` — is an
/// `Err`, never a "success" that wrote nothing.
pub fn process_data_editor_command(cmd: DataEditCommand) -> Result<DataEditResult, String> {
    let path = Path::new(&cmd.target_file);
    if !path.exists() {
        return Err(format!("Target dataset file {:?} does not exist", path));
    }
    let text = std::fs::read_to_string(path).map_err(|e| format!("read {:?}: {}", path, e))?;
    let mut doc: Value =
        serde_json::from_str(&text).map_err(|e| format!("{:?} is not JSON: {}", path, e))?;
    let rows = doc
        .as_array_mut()
        .ok_or_else(|| format!("{:?} is not a JSON array of records", path))?;
    let position = |rows: &Vec<Value>| {
        rows.iter()
            .position(|r| r.get(ID_FIELD).map_or(false, |v| id_matches(v, &cmd.row_id)))
    };

    match cmd.operation.as_str() {
        "append" => {
            if position(rows).is_some() {
                return Err(format!("row {:?} already exists; use modify", cmd.row_id));
            }
            let mut record = match &cmd.payload {
                Value::Object(map) => map.clone(),
                other => return Err(format!("append payload must be an object, got {}", other)),
            };
            record
                .entry(ID_FIELD.to_string())
                .or_insert_with(|| Value::String(cmd.row_id.clone()));
            rows.push(Value::Object(record));
        }
        "modify" => {
            let at = position(rows).ok_or_else(|| format!("row {:?} not found", cmd.row_id))?;
            let fields = cmd
                .payload
                .as_object()
                .ok_or_else(|| format!("modify payload must be an object, got {}", cmd.payload))?;
            let record = rows[at]
                .as_object_mut()
                .ok_or_else(|| format!("row {:?} is not an object", cmd.row_id))?;
            for (k, v) in fields {
                record.insert(k.clone(), v.clone());
            }
        }
        "delete" => {
            let at = position(rows).ok_or_else(|| format!("row {:?} not found", cmd.row_id))?;
            rows.remove(at);
        }
        other => return Err(format!("unknown operation {:?} (append, modify, delete)", other)),
    }

    let out = serde_json::to_string_pretty(&doc).map_err(|e| format!("serialise: {}", e))?;
    let tmp = path.with_extension("edit.tmp");
    std::fs::write(&tmp, out + "\n").map_err(|e| format!("write {:?}: {}", tmp, e))?;
    std::fs::rename(&tmp, path).map_err(|e| format!("replace {:?}: {}", path, e))?;

    Ok(DataEditResult {
        status: "success".to_string(),
        target_file: cmd.target_file,
        records_modified: 1,
    })
}

/// A record's id may be stored as a string or a number; `row_id` arrives as text.
fn id_matches(v: &Value, row_id: &str) -> bool {
    match v {
        Value::String(s) => s == row_id,
        Value::Number(n) => n.to_string() == row_id,
        _ => false,
    }
}

/// Converts CSV marker files to MQTT JSON payloads.
pub fn convert_marker_csv_to_mqtt_json(csv_content: &str) -> Result<String, String> {
    let mut lines = csv_content.lines();
    let _header = lines.next().unwrap_or("");
    let mut markers = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 2 {
            markers.push(serde_json::json!({
                "marker_id": parts[0].trim(),
                "value": parts[1].trim()
            }));
        }
    }

    serde_json::to_string_pretty(&markers).map_err(|e| format!("JSON serialization error: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn dataset(name: &str, body: &str) -> String {
        let dir = std::env::temp_dir().join(format!("apk-data-editor-{}-{}", std::process::id(), name));
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join("set.json");
        std::fs::write(&p, body).unwrap();
        p.to_string_lossy().into_owned()
    }

    fn cmd(file: &str, op: &str, id: &str, payload: Value) -> DataEditCommand {
        DataEditCommand { target_file: file.into(), operation: op.into(), row_id: id.into(), payload }
    }

    fn read(file: &str) -> Value {
        serde_json::from_str(&std::fs::read_to_string(file).unwrap()).unwrap()
    }

    #[test]
    fn append_modify_delete_change_the_file() {
        let f = dataset("amd", r#"[{"id":"a","freq":500}]"#);
        let r = process_data_editor_command(cmd(&f, "append", "b", json!({"freq": 600}))).unwrap();
        assert_eq!(r.records_modified, 1);
        assert_eq!(read(&f), json!([{"id":"a","freq":500},{"id":"b","freq":600}]));
        process_data_editor_command(cmd(&f, "modify", "a", json!({"freq": 510, "name": "IEM"}))).unwrap();
        assert_eq!(read(&f)[0], json!({"id":"a","freq":510,"name":"IEM"}));
        process_data_editor_command(cmd(&f, "delete", "b", Value::Null)).unwrap();
        assert_eq!(read(&f), json!([{"id":"a","freq":510,"name":"IEM"}]));
    }

    #[test]
    fn nothing_written_is_never_success() {
        let f = dataset("err", r#"[{"id":1}]"#);
        assert!(process_data_editor_command(cmd(&f, "modify", "2", json!({"x":1}))).is_err());
        assert!(process_data_editor_command(cmd(&f, "delete", "2", Value::Null)).is_err());
        assert!(process_data_editor_command(cmd(&f, "append", "1", json!({}))).is_err());
        assert!(process_data_editor_command(cmd(&f, "frobnicate", "1", Value::Null)).is_err());
        assert_eq!(read(&f), json!([{"id":1}]));
        let g = dataset("notarray", r#"{"id":1}"#);
        assert!(process_data_editor_command(cmd(&g, "append", "2", json!({}))).is_err());
    }
}
