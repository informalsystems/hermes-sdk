use cgp_core::HasErrorType;
use serde_json::Value;

pub trait CanModifyCosmosGenesisConfig: HasErrorType {
    fn modify_genesis_config(&self, config: &mut Value) -> Result<(), Self::Error>;
}
