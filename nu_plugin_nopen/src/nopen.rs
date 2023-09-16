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
        let output = call.get_flag("output")?;

        // output for debug
        eprintln!("output: {:?}", output);
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
                        return Err(LabeledError {
                            label: "Invalid JSON format".into(),
                            msg: "Invalid JSON format".into(),
                            span: Some(call.head),
                        });
                    }
                }
            }
            serde_json::Value::Object(obj) => {
                json_datas.push(serde_json::Value::Object(obj));
            }
            _ => {
                return Err(LabeledError {
                    label: "Invalid JSON format".into(),
                    msg: "Invalid JSON format".into(),
                    span: Some(call.head),
                });
            }
        }

        let mut map_keys: Vec<String> = Vec::new();
        for json_data in json_datas.iter() {
            if let serde_json::Value::Object(ref map) = json_data {
                for key in map.keys() {
                    if !map_keys.contains(key) {
                        map_keys.push(key.clone());
                    }
                }
            }
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
                for i in 0..map.len() {
                    let key = map.keys().nth(i).unwrap();
                    let value = map.get(key).unwrap();

                    // map_keysにおけるインデックスを求める
                    let mut index = 0;
                    for j in 0..map_keys.len() {
                        if map_keys.get(j).unwrap() == key {
                            index = j;
                            break;
                        }
                    }
                    let key = map_keys.get(index).unwrap();

                    if selections.contains(&index) {
                        new_json_data.insert(
                            key.clone(),
                            // saltを追加する
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
        let output_name = {
            if let Some(output) = output {
                output
            } else {
                "output.json".to_string()
            }
        };
        fs::write(output_name, output_data).expect("Unable to write to file");

        Ok(Value::nothing(call.head))
    }
}
        

