use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[derive_component(PayloadHeaderTypeComponent, ProvidePayloadHeaderType<Chain>)]
pub trait HasPayloadHeaderType<Counterparty>: Async {
    type PayloadHeader: Async;
}

impl<Chain, Counterparty, Provider, PayloadHeader> ProvidePayloadHeaderType<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, PayloadHeaderTypeComponent, Type = PayloadHeader>,
    PayloadHeader: Async,
{
    type PayloadHeader = PayloadHeader;
}
