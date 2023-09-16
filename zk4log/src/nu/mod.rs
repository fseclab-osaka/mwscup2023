use crate::Zk4log;
use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{PluginSignature, SyntaxShape, Value};

impl Plugin for Zk4log {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![
            PluginSignature::build("zk4log")
                .usage("zk4log")
                .required("mode", SyntaxShape::String, "mode") // hide, verify, or open
                .named("json", SyntaxShape::String, "json file name", Some('i'))
                .named("proof", SyntaxShape::String, "proof file name", Some('p'))
        ]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        match name {
            // 
        }
    }
}