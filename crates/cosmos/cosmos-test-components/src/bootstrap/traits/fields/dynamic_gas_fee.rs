use hermes_cosmos_chain_components::types::DynamicGasConfig;
use hermes_prelude::*;

#[cgp_getter {
    provider: DynamicGasGetter,
}]
pub trait HasDynamicGas {
    fn dynamic_gas(&self) -> &Option<DynamicGasConfig>;
}
