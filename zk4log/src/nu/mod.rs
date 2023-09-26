use crate::Zk4log;
use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{PluginSignature, SyntaxShape, Value};

impl Plugin for Zk4log {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![
            PluginSignature::build("zk4log hide")
                .usage("zk4log hide -h")
                .required("path", SyntaxShape::String, ""),
            PluginSignature::build("zk4log verify")
                .usage("zk4log verify -h")
                .required_named("json", SyntaxShape::String, "json file name", Some('i'))
                .required_named("proof", SyntaxShape::String, "proof file name", Some('p'))
                .required_named("key", SyntaxShape::String, "key file name", Some('k')),
            PluginSignature::build("zk4log open")
                .usage("zk4log open -h")
                .required("path", SyntaxShape::String, ""),
            PluginSignature::build("zk4log")
                .usage("Log Analysis Tool with ZKP")
                .required(
                    "subcommand",
                    SyntaxShape::String,
                    "\"hide\", \"open\" or \"verify\"",
                ),
        ]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        match name {
            "zk4log hide" => self.hide(call, input),
            "zk4log verify" => self.verify(call, input),
            "zk4log open" => self.open(call, input),
            "zk4log" => self.zk4log(call, input),
            _ => Err(LabeledError {
                label: "Plugin call with wrong name".into(),
                msg: "the signature used to call the plugin does not match any name in the plugin signature vector".into(),
                span: Some(call.head),
            }),
        }
    }
}
