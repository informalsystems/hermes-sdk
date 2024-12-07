use core::marker::PhantomData;

use cgp::core::component::UseContext;
use cgp::prelude::*;
use toml::Value;

#[cgp_component {
  name: CometConfigModifierComponent,
  provider: CometConfigModifier,
  context: Bootstrap,
}]
#[async_trait]
pub trait CanModifyCometConfig: HasErrorType {
    fn modify_comet_config(&self, comet_config: &mut Value) -> Result<(), Self::Error>;
}

impl<Bootstrap, Modifier> CometConfigModifier<Bootstrap> for UseContext
where
    Bootstrap: HasErrorType + HasField<symbol!("comet_config_modifier"), Value = Modifier>,
    Modifier: Fn(&mut Value) -> Result<(), Bootstrap::Error> + Send + Sync + 'static,
{
    fn modify_comet_config(
        bootstrap: &Bootstrap,
        comet_config: &mut Value,
    ) -> Result<(), Bootstrap::Error> {
        bootstrap.get_field(PhantomData)(comet_config)
    }
}
