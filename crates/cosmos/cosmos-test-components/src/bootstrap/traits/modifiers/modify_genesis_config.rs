use core::marker::PhantomData;

use cgp::prelude::*;
use serde_json::Value;

#[cgp_component {
  provider: CosmosGenesisConfigModifier,
  context: Bootstrap,
}]
pub trait CanModifyCosmosGenesisConfig: HasAsyncErrorType {
    fn modify_genesis_config(&self, genesis_config: &mut Value) -> Result<(), Self::Error>;
}

#[cgp_provider(CosmosGenesisConfigModifierComponent)]
impl<Bootstrap, Tag> CosmosGenesisConfigModifier<Bootstrap> for UseField<Tag>
where
    Bootstrap: HasAsyncErrorType + HasField<Tag>,
    Bootstrap::Value: Fn(&mut Value) -> Result<(), Bootstrap::Error> + Send + Sync + 'static,
{
    fn modify_genesis_config(
        bootstrap: &Bootstrap,
        genesis_config: &mut Value,
    ) -> Result<(), Bootstrap::Error> {
        bootstrap.get_field(PhantomData)(genesis_config)
    }
}
