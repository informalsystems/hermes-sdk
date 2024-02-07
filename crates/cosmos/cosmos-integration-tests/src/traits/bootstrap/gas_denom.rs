use cgp_core::prelude::*;

#[derive_component(GasDenomGetterComponent, GasDenomGetter<Bootstrap>)]
pub trait HasGasDenom: Async {
    fn gas_denom(&self) -> &str;
}
