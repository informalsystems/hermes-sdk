use cgp_core::prelude::*;

#[derive_component(DataChainTypeComponent, ProvideDataChainType<Chain>)]
pub trait HasDataChainType: Async {
    type DataChain: Async;
}

#[derive_component(DataChainGetterComponent, DataChainGetter<Chain>)]
pub trait HasDataChain: HasDataChainType {
    fn data_chain(&self) -> &Self::DataChain;
}
