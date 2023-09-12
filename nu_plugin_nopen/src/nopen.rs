use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::Value;
use std::path::Path;
use clap::{App, Arg};
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use serde_json::{json, to_value, Map};
use std::fs;

pub struct Nopen;

impl Nopen {
    pub fn nopen(&self, call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        // 引数の文字列を取得する
        let path: String = call.req(0)?;
        eprintln!("path: {}", path);

        // pathのファイルが存在するかどうかを確認する
        let path = Path::new(&path);
        if !path.exists() {
            eprintln!("test");
            eprintln!("File not found: {}", path.display());

            return Err(LabeledError {
                label: "File not found".into(),
                msg: "file not found".into(),
                span: Some(call.head),
            });
        }

        eprintln!("Open file: {}", path.display());

        let data = fs::read_to_string(path).expect("Unable to read file");
        let json_data: serde_json::Value = serde_json::from_str(&data).expect("Invalid JSON format");

        let mut map_keys: Vec<String> = Vec::new();
        if let serde_json::Value::Object(ref map) = json_data {
            map_keys = map.keys().cloned().collect();
        }

        let selections = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select the columns you want to sha256 hash")
            .items(&map_keys)
            .interact()
            .unwrap();

        Ok(nu_protocol::Value::Nothing { internal_span: call.head })
    }
}
        

