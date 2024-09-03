use cgp::prelude::*;
use toml::Value;

#[derive_component(CometConfigModifierComponent, CometConfigModifier<Bootstrap>)]
#[async_trait]
pub trait CanModifyCometConfig: HasErrorType {
    fn modify_comet_config(&self, comet_config: &mut Value) -> Result<(), Self::Error>;
}
