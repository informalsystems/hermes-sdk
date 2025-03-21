use core::marker::PhantomData;

use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_cosmos_chain_components::types::config::gas::dynamic_gas_config::DynamicGasConfig;

#[cgp_component {
  provider: DynamicGasGetter,
  context: Bootstrap,
}]
pub trait HasDynamicGas: Async {
    fn dynamic_gas(&self) -> &Option<DynamicGasConfig>;
}

#[cgp_provider(DynamicGasGetterComponent)]
impl<Bootstrap> DynamicGasGetter<Bootstrap> for UseContext
where
    Bootstrap: Async + HasField<symbol!("dynamic_gas"), Value = Option<DynamicGasConfig>>,
{
    fn dynamic_gas(bootstrap: &Bootstrap) -> &Option<DynamicGasConfig> {
        bootstrap.get_field(PhantomData)
    }
}
