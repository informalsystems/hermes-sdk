use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[cgp_component {
  name: PayloadTypeComponent,
  provider: ProvidePayloadType,
  context: Chain,
}]
pub trait HasPayloadType<Counterparty>: Async {
    type Payload: Async;
}

impl<Chain, Counterparty, Provider, Payload> ProvidePayloadType<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, PayloadTypeComponent, Type = Payload>,
    Payload: Async,
{
    type Payload = Payload;
}
