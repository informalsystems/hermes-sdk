use hermes_protobuf_encoding_components::types::Any;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AnyResponse {
    pub type_url: String,
    pub value: Vec<u8>,
}

impl Into<Any> for AnyResponse {
    fn into(self) -> Any {
        Any {
            type_url: self.type_url,
            value: self.value,
        }
    }
}
