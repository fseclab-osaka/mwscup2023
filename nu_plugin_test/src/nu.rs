use crate::Test;
use nu_plugin::{LabeledError, Plugin};
use nu_protocol::{Category, PluginExample, PluginSignature, SyntaxShape};

impl Plugin for Test {
    fn signature(&self) -> Vec<nu_protocol::PluginSignature> {
        vec![
            PluginSignature::build("test-dayo")
                .usage("PluginSignature test 1 for plugin. Returns Value::Nothing")
                .required("a", SyntaxShape::Int, "required integer value")
                .required("b", SyntaxShape::String, "required string value")
                .switch("flag", "a flag for the signature", Some('f'))
                .optional("opt", SyntaxShape::Int, "Optional number")
                .named("named", SyntaxShape::String, "named string", Some('n'))
                .rest("rest", SyntaxShape::String, "rest value string")
                .plugin_examples(vec![PluginExample {
                    example: "test 3 bb".into(),
                    description: "running example with an int value and string value".into(),
                    result: None,
                }])
                .category(Category::Experimental)
        ]
    }

    fn run(
            &mut self,
            name: &str,
            call: &nu_plugin::EvaluatedCall,
            input: &nu_protocol::Value,
        ) -> Result<nu_protocol::Value, nu_plugin::LabeledError> {
            match name {
                "test-dayo" => self.test(call, input),
                _ => Err(LabeledError {
                    label: "Plugin call with wrong name".into(),
                    msg: "the signature used to call the plugin does not match any name in the plugin signature vector".into(),
                    span: Some(call.head),
                }),
            }
    }
}