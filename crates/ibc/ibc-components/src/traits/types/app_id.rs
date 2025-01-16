use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use cgp::prelude::*;

#[cgp_component {
  name: AppIdTypeComponent,
  provider: ProvideAppIdType,
  context: Chain,
}]
pub trait HasAppIdType<Counterparty>: Sized + Async {
    type AppId: Async;
}

impl<Chain, Counterparty, Provider, AppId> ProvideAppIdType<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, AppIdTypeComponent, Type = AppId>,
    AppId: Async,
{
    type AppId = AppId;
}
