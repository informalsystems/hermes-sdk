use cgp::prelude::*;
use ibc_relayer::chain::cosmos::types::gas::GasConfig;

#[derive_component(GasConfigGetterComponent, GasConfigGetter<Chain>)]
pub trait HasGasConfig: Async {
    fn gas_config(&self) -> &GasConfig;
}
