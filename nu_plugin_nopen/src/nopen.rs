use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::Value;
use std::path::Path;

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
        Ok(nu_protocol::Value::Nothing { internal_span: call.head })
    }
}
        

