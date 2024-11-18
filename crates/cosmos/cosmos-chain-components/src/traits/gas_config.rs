use cgp::prelude::*;

use crate::types::gas::gas_config::GasConfig;

#[derive_component(GasConfigGetterComponent, GasConfigGetter<Chain>)]
pub trait HasGasConfig: Async {
    fn gas_config(&self) -> &GasConfig;
}
