use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::Value;
use std::{path::Path, fs};
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use serde_json::{to_value, Map};
use sha256::digest;

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
        let json_value: serde_json::Value = serde_json::from_str(&data).expect("Invalid JSON format");
        let mut json_datas: Vec<serde_json::Value> = Vec::new();

        match json_value {
            serde_json::Value::Array(array_val) => {
                for item in array_val.iter() {
                    if let Some(obj) = item.as_object() {
                        json_datas.push(serde_json::Value::Object(obj.clone()));
                    } else {
                        // error
                        todo!();
                    }
                }
            }
            serde_json::Value::Object(obj) => {
                json_datas.push(serde_json::Value::Object(obj));
            }
            _ => {
                // error
                todo!();
            }
        }

        let mut map_keys: Vec<String> = Vec::new();
        if let serde_json::Value::Object(ref map) = json_datas[0] {
            map_keys = map.keys().cloned().collect();
        }

        let selections = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select the columns you want to sha256 hash")
            .items(&map_keys)
            .interact()
            .unwrap();

        let mut new_json_datas: Vec<serde_json::Value> = Vec::new();
        for json_data in json_datas {
            if let serde_json::Value::Object(ref map) = json_data {
                let mut new_json_data: Map<String, serde_json::Value> = Map::new();
                for i in 0..=(map_keys.len() - 1) {
                    let key = map_keys.get(i).unwrap();
                    let value = map.get(key).unwrap();

                    if selections.contains(&i) {
                        new_json_data.insert(
                            key.clone(),
                            to_value(digest(value.clone().to_string())).unwrap(),
                            );
                    } else {
                        new_json_data.insert(key.clone(), value.clone());
                    }
                }
                new_json_datas.push(serde_json::Value::Object(new_json_data));
            }
        }

        // 要素が1つの場合にも配列にならないようにする
        let output_data = {
            if new_json_datas.len() == 1 {
                serde_json::to_string_pretty(&new_json_datas[0]).expect("Failed to serialize to JSON")
            } else {
                serde_json::to_string_pretty(&new_json_datas).expect("Failed to serialize to JSON")
            }
        };
        fs::write("output.json", output_data).expect("Unable to write to file");

        Ok(nu_protocol::Value::Nothing { internal_span: call.head })
    }
}
        

