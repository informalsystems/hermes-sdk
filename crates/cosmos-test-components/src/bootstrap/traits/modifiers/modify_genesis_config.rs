use cgp_core::prelude::*;
use serde_json::Value;

#[derive_component(CosmosGenesisConfigModifierComponent, CosmosGenesisConfigModifier<Bootstrap>)]
pub trait CanModifyCosmosGenesisConfig: HasErrorType {
    fn modify_genesis_config(&self, config: &mut Value) -> Result<(), Self::Error>;
}
