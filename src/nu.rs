use crate::Copy;
use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{PluginSignature, SyntaxShape, Value};

impl Plugin for Copy {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![
            PluginSignature::build("copy")
                    .usage("Copy one column or full row log to clipboard selected log data with the given sn")
                    .required("path", SyntaxShape::String, "path to the json file")
                    .required_named("sn", SyntaxShape::String, "serial number of the log", Some('s'))
                    .named("column", SyntaxShape::String, "column name of the log", Some('c'))
        ]
    }

    fn run (&mut self, name: &str, call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        match name {
            "copy" => self.copy(call, input),
            _ => Err(LabeledError {
                label: "Plugin call with wrong name".into(),
                msg: "the signature used to call the plugin does not match any name in the plugin signature vector".into(),
                span: Some(call.head),
            }),
        }
    }
}
