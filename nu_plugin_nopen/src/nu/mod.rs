use crate::Nopen;
use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{PluginSignature, SyntaxShape, Value};

impl Plugin for Nopen {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![
            PluginSignature::build("nopen")
                .usage("nopen [json file]")
                .required("file", SyntaxShape::String, "the file to open"),
        ]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        match name {
            "nopen" => self.nopen(call, input),
            _ => Err(LabeledError {
                label: "Plugin call with wrong name".into(),
                msg: "the signature used to call the plugin does not match any name in the plugin signature vector".into(),
                span: Some(call.head),
            }),
        }
    }
}
