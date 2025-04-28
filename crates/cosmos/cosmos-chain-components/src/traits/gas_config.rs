use hermes_prelude::*;

use crate::types::GasConfig;

#[cgp_component {
  provider: GasConfigGetter,
  context: Chain,
}]
pub trait HasGasConfig: Async {
    fn gas_config(&self) -> &GasConfig;
}
