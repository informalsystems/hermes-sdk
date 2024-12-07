use core::marker::PhantomData;

use cgp::core::component::UseContext;
use cgp::prelude::*;
use toml::Value;

#[cgp_component {
  name: CosmosSdkConfigModifierComponent,
  provider: CosmosSdkConfigModifier,
  context: Bootstrap,
}]
#[async_trait]
pub trait CanModifyCosmosSdkConfig: HasErrorType {
    fn modify_cosmos_sdk_config(&self, cosmos_sdk_config: &mut Value) -> Result<(), Self::Error>;
}

impl<Bootstrap, Modifier> CosmosSdkConfigModifier<Bootstrap> for UseContext
where
    Bootstrap: HasErrorType + HasField<symbol!("cosmos_sdk_config_modifier"), Value = Modifier>,
    Modifier: Fn(&mut Value) -> Result<(), Bootstrap::Error> + Send + Sync + 'static,
{
    fn modify_cosmos_sdk_config(
        bootstrap: &Bootstrap,
        cosmos_sdk_config: &mut Value,
    ) -> Result<(), Bootstrap::Error> {
        bootstrap.get_field(PhantomData)(cosmos_sdk_config)
    }
}
