use core::marker::PhantomData;

use cgp::core::component::UseContext;
use cgp::prelude::*;
use serde_json::Value;

#[cgp_component {
  provider: CosmosGenesisConfigModifier,
  context: Bootstrap,
}]
pub trait CanModifyCosmosGenesisConfig: HasAsyncErrorType {
    fn modify_genesis_config(&self, genesis_config: &mut Value) -> Result<(), Self::Error>;
}

impl<Bootstrap, Modifier> CosmosGenesisConfigModifier<Bootstrap> for UseContext
where
    Bootstrap: HasAsyncErrorType + HasField<symbol!("genesis_config_modifier"), Value = Modifier>,
    Modifier: Fn(&mut Value) -> Result<(), Bootstrap::Error> + Send + Sync + 'static,
{
    fn modify_genesis_config(
        bootstrap: &Bootstrap,
        genesis_config: &mut Value,
    ) -> Result<(), Bootstrap::Error> {
        bootstrap.get_field(PhantomData)(genesis_config)
    }
}
