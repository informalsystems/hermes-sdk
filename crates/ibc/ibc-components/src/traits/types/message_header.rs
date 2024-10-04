use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[derive_component(IbcMessageHeaderTypeComponent, ProvideIbcMessageHeaderType<Chain>)]
pub trait HasIbcMessageHeaderType<Counterparty>: Async {
    type IbcMessageHeader: Async;
}

impl<Chain, Counterparty, Provider, IbcMessageHeader>
    ProvideIbcMessageHeaderType<Chain, Counterparty> for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, IbcMessageHeaderTypeComponent, Type = IbcMessageHeader>,
    IbcMessageHeader: Async,
{
    type IbcMessageHeader = IbcMessageHeader;
}
