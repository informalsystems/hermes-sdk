use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use hermes_prelude::*;

#[cgp_component {
  name: PayloadHeaderTypeComponent,
  provider: ProvidePayloadHeaderType,
  context: Chain,
}]
pub trait HasPayloadHeaderType<Counterparty>: Async {
    type PayloadHeader: Async;
}

#[cgp_provider(PayloadHeaderTypeComponent)]
impl<Chain, Counterparty, Provider, PayloadHeader> ProvidePayloadHeaderType<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, PayloadHeaderTypeComponent, Type = PayloadHeader>,
    PayloadHeader: Async,
{
    type PayloadHeader = PayloadHeader;
}
