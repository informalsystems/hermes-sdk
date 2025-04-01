use cgp::prelude::*;
use hermes_cosmos_chain_components::types::config::gas::dynamic_gas_config::DynamicGasConfig;

#[cgp_getter {
    provider: DynamicGasGetter,
}]
pub trait HasDynamicGas {
    fn dynamic_gas(&self) -> &Option<DynamicGasConfig>;
}
