use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::Value;
use std::{path::Path, fs};
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use serde_json::{to_value, Map};
use sha256::digest;

pub struct Zk4log;

imple Zk4log {
    pub fn zk4log(&self, call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        Ok(Value::nothing(call.head))
    }
}