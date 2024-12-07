use cgp::prelude::*;

use crate::types::config::gas::gas_config::GasConfig;

#[cgp_component {
  name: GasConfigGetterComponent,
  provider: GasConfigGetter,
  context: Chain,
}]
pub trait HasGasConfig: Async {
    fn gas_config(&self) -> &GasConfig;
}
