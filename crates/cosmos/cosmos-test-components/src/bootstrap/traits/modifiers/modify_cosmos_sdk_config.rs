use core::marker::PhantomData;

use cgp::core::component::UseContext;
use hermes_prelude::*;
use toml::Value;

#[cgp_component {
  provider: CosmosSdkConfigModifier,
  context: Bootstrap,
}]
#[async_trait]
pub trait CanModifyCosmosSdkConfig: HasAsyncErrorType {
    fn modify_cosmos_sdk_config(&self, cosmos_sdk_config: &mut Value) -> Result<(), Self::Error>;
}

#[cgp_provider(CosmosSdkConfigModifierComponent)]
impl<Bootstrap, Modifier> CosmosSdkConfigModifier<Bootstrap> for UseFields
where
    Bootstrap:
        HasAsyncErrorType + HasField<symbol!("cosmos_sdk_config_modifier"), Value = Modifier>,
    Modifier: Fn(&mut Value) -> Result<(), Bootstrap::Error> + Send + Sync + 'static,
{
    fn modify_cosmos_sdk_config(
        bootstrap: &Bootstrap,
        cosmos_sdk_config: &mut Value,
    ) -> Result<(), Bootstrap::Error> {
        bootstrap.get_field(PhantomData)(cosmos_sdk_config)
    }
}
