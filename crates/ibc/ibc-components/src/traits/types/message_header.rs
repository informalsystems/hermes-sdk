use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use hermes_prelude::*;

#[cgp_component {
  name: IbcMessageHeaderTypeComponent,
  provider: ProvideIbcMessageHeaderType,
  context: Chain,
}]
pub trait HasIbcMessageHeaderType<Counterparty>: Async {
    type IbcMessageHeader: Async;
}

#[cgp_provider(IbcMessageHeaderTypeComponent)]
impl<Chain, Counterparty, Provider, IbcMessageHeader>
    ProvideIbcMessageHeaderType<Chain, Counterparty> for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, IbcMessageHeaderTypeComponent, Type = IbcMessageHeader>,
    IbcMessageHeader: Async,
{
    type IbcMessageHeader = IbcMessageHeader;
}
