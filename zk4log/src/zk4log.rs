use crate::zk::{self, prove, verify};
use bellman::groth16::{self, PreparedVerifyingKey, Proof, VerifyingKey};
use bls12_381::Bls12;
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use hex;
use nu_path::expand_tilde;
use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::Value;
use rand::Rng;
use serde_json::{from_str, to_value, Map};
use sha2::{Digest, Sha256};
use sha256::digest;
use std::{
    fs,
    io::{self, Read, Write},
    process::Command,
};

pub struct Zk4log;

impl Zk4log {
    pub fn hide(&self, call: &EvaluatedCall, _input: &Value) -> Result<Value, LabeledError> {
        // 引数の文字列を取得する
        let path: String = call.req(0)?;
        let output = call.get_flag("output")?;

        // pathのファイルが存在するかどうかを確認する
        Self::check_file_exists(&path, call)?;
        eprintln!("Open file: {}", path);

        let data = fs::read_to_string(path).expect("Unable to read file");
        let json_value: serde_json::Value =
            serde_json::from_str(&data).expect("Invalid JSON format");
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

        let salt: String = Self::gen_salt();

        // パラメタと検証鍵を生成
        let (params, pvk) = zk::setup();

        // ログを秘匿化しつつ ZKP を生成する
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
                        // ハッシュ化のために、入力ログデータを
                        // (Stringではなく) 固定長のu8 配列で表現する
                        let preimage_str = value.clone().to_string() + &salt;
                        let mut preimage_bytes: [u8; 80] = [0; 80];
                        preimage_bytes[..preimage_str.len()]
                            .copy_from_slice(preimage_str.as_bytes());

                        // u8 配列であるハッシュ値を、
                        // ファイル書き出し用に16進数文字列に変換
                        let hash_bytes = Sha256::digest(&Sha256::digest(&preimage_bytes));
                        let hash_str = hash_bytes
                            .iter()
                            .map(|b| format!("{:02x}", b))
                            .collect::<Vec<String>>()
                            .join("");

                        new_json_data.insert(key.clone(), to_value(hash_str).unwrap());

                        let proof = prove(params.clone(), preimage_str);
                        let mut file = fs::OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open("out.proof")
                            .unwrap();

                        fs::write("out.proof", "").unwrap(); // clear
                        let data = format!("{}::{}::", i, key);
                        file.write_all(data.as_bytes()).unwrap();
                        proof.write(&mut file).unwrap();
                        file.write_all("::".as_bytes()).unwrap();

                        let file = fs::File::create("key.pub").unwrap();
                        let vk: VerifyingKey<Bls12> = params.vk.clone();
                        vk.write(file).unwrap();
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
                serde_json::to_string_pretty(&new_json_datas[0])
                    .expect("Failed to serialize to JSON")
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

    pub fn open(&self, call: &EvaluatedCall, _input: &Value) -> Result<Value, LabeledError> {
        let path: String = call.req(0)?;

        // pathのファイルが存在するかどうかを確認する
        Self::check_file_exists(&path, call)?;

        let data = fs::read_to_string(&path).expect("Unable to read file");
        let json_value: serde_json::Value =
            serde_json::from_str(&data).expect("Invalid JSON format");
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
            .with_prompt("Select the columns you want to open")
            .items(&map_keys)
            .interact()
            .unwrap();

        let mut open_json_columns: Vec<String> = Vec::new();
        for selection in selections {
            open_json_columns.push(map_keys.get(selection).unwrap().clone());
        }

        let open_json_columns = open_json_columns.join(" ");
        let command_str = format!("open {} | select -i {}", &path, open_json_columns);
        eprintln!("{}", command_str);
        let output = Command::new("nu")
            .arg("-c")
            .arg(command_str)
            .output()
            .expect("Failed to execute command");
        eprintln!("{}", String::from_utf8_lossy(&output.stdout));

        return Ok(Value::nothing(call.head));
    }

    pub fn verify(&self, call: &EvaluatedCall, _input: &Value) -> Result<Value, LabeledError> {
        // log, proof, key があるかを確認
        let log: String = call.get_flag_value("json").unwrap().as_string().unwrap();
        Self::check_file_exists(&log, call)?;
        let proof: String = call.get_flag_value("proof").unwrap().as_string().unwrap();
        Self::check_file_exists(&proof, call)?;
        let key: String = call.get_flag_value("key").unwrap().as_string().unwrap();
        Self::check_file_exists(&key, call)?;
        
        // log をロード
        let log = fs::read_to_string(&log).unwrap();
        let log_json: serde_json::Value = from_str(&log).unwrap();

        // proof をロード
        let mut proof_buf = Vec::new();
        let mut file_reader = io::BufReader::new(fs::File::open(&proof).unwrap());
        file_reader.read_to_end(&mut proof_buf).unwrap();

        // key をロード
        let pvk: PreparedVerifyingKey<Bls12> = groth16::prepare_verifying_key(
            &VerifyingKey::read(io::BufReader::new(fs::File::open(&key).unwrap())).unwrap(),
        );

        // proof は `<json_record_no>::<json_key>::<proof_data>` という形式。
        // proof のパースには `split("::")` ではなく ascii 版の[58, 58] を使う。
        // <proof_data> が String ではないことにより、u8 配列として proof を
        // 扱うためである。

        let delimiter: [u8; 2] = [58, 58]; // "::" のバイト表現

        let mut start = 0;
        let mut buf = Vec::new();
        for (i, byte) in proof_buf.iter().enumerate() {
            if byte == &delimiter[0] && i + 1 < proof_buf.len() && proof_buf[i + 1] == delimiter[1]
            {
                // delimiter を見つけた場合、区切り位置までの部分を処理
                let chunk = &proof_buf[start..i];

                // バイト配列から文字列への変換
                buf.push(chunk);

                // 次の区切り位置の開始位置を更新
                start = i + 2; // delimiter の長さ分進める
            }
        }

        // 最後の区切り位置から終端までの部分を処理
        let chunk = &proof_buf[start..];
        buf.push(chunk);

        let i: usize = std::str::from_utf8(buf[0]).unwrap().parse().unwrap();
        let k = std::str::from_utf8(buf[1]).unwrap().trim_matches('"');
        let proof: Proof<Bls12> = Proof::read(io::Cursor::new(buf[2])).unwrap();
        let hash = log_json[i][k].as_str().unwrap().trim_matches('"');
        let hash = &hex::decode(hash).unwrap();

        zk::verify(&pvk, &hash, &proof);

        return Ok(Value::nothing(call.head));
    }

    // gen_salt: 16 文字の乱数を生成する
    fn gen_salt() -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
        let mut rng = rand::thread_rng();

        (0..16)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    fn check_file_exists(file: &str, call: &EvaluatedCall) -> Result<(), LabeledError> {
        let path = expand_tilde(&file);
        if !path.exists() {
            eprintln!("File not found: {}", path.display());

            return Err(LabeledError {
                label: "File not found".into(),
                msg: "file not found".into(),
                span: Some(call.head),
            });
        }

        Ok(())
    }

    pub fn zk4log(&self, call: &EvaluatedCall, _input: &Value) -> Result<Value, LabeledError> {
        Err(LabeledError {
            label: "subcommand in [\"hide\", \"verify\", \"open\"]".into(),
            msg: "zk4log <subcommand>".into(),
            span: Some(call.head),
        })
    }
}
