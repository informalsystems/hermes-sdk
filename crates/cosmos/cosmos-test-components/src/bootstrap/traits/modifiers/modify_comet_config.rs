use core::marker::PhantomData;

use cgp::prelude::*;
use toml::Value;

#[cgp_component {
  provider: CometConfigModifier,
  context: Bootstrap,
}]
#[async_trait]
pub trait CanModifyCometConfig: HasAsyncErrorType {
    fn modify_comet_config(&self, comet_config: &mut Value) -> Result<(), Self::Error>;
}

#[cgp_provider(CometConfigModifierComponent)]
impl<Bootstrap, Tag> CometConfigModifier<Bootstrap> for UseField<Tag>
where
    Bootstrap: HasAsyncErrorType + HasField<Tag>,
    Bootstrap::Value: Fn(&mut Value) -> Result<(), Bootstrap::Error> + Send + Sync + 'static,
{
    fn modify_comet_config(
        bootstrap: &Bootstrap,
        comet_config: &mut Value,
    ) -> Result<(), Bootstrap::Error> {
        bootstrap.get_field(PhantomData)(comet_config)
    }
}
