use core::marker::PhantomData;

use cgp::core::component::UseContext;
use cgp::prelude::*;
use serde_json::Value;

#[derive_component(CosmosGenesisConfigModifierComponent, CosmosGenesisConfigModifier<Bootstrap>)]
pub trait CanModifyCosmosGenesisConfig: HasErrorType {
    fn modify_genesis_config(&self, genesis_config: &mut Value) -> Result<(), Self::Error>;
}

impl<Bootstrap, Modifier> CosmosGenesisConfigModifier<Bootstrap> for UseContext
where
    Bootstrap: HasErrorType + HasField<symbol!("genesis_config_modifier"), Field = Modifier>,
    Modifier: Fn(&mut Value) -> Result<(), Bootstrap::Error> + Send + Sync + 'static,
{
    fn modify_genesis_config(
        bootstrap: &Bootstrap,
        genesis_config: &mut Value,
    ) -> Result<(), Bootstrap::Error> {
        bootstrap.get_field(PhantomData)(genesis_config)
    }
}
