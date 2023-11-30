use cgp_core::HasErrorType;
use serde_json::Value;

pub trait CanUpdateGenesisJsonConfig: HasErrorType {
    fn update_genesis_json_config(&self, json_value: &mut Value) -> Result<(), Self::Error>;
}
