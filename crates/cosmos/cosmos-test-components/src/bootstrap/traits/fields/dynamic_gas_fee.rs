use core::marker::PhantomData;

use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_cosmos_chain_components::types::config::gas::dynamic_gas_config::DynamicGasConfig;

#[derive_component(DynamicGasGetterComponent, DynamicGasGetter<Bootstrap>)]
pub trait HasDynamicGas: Async {
    fn dynamic_gas(&self) -> &Option<DynamicGasConfig>;
}

impl<Bootstrap> DynamicGasGetter<Bootstrap> for UseContext
where
    Bootstrap: Async + HasField<symbol!("dynamic_gas"), Field = Option<DynamicGasConfig>>,
{
    fn dynamic_gas(bootstrap: &Bootstrap) -> &Option<DynamicGasConfig> {
        bootstrap.get_field(PhantomData)
    }
}

pub struct ReturnNoDynamicGas;

impl<Bootstrap> DynamicGasGetter<Bootstrap> for ReturnNoDynamicGas
where
    Bootstrap: Async,
{
    fn dynamic_gas(_bootstrap: &Bootstrap) -> &Option<DynamicGasConfig> {
        &None
    }
}
