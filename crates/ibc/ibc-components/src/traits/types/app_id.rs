use cgp::core::component::WithProvider;
use cgp::core::types::traits::HasType;
use cgp::prelude::*;

#[derive_component(AppIdTypeComponent, ProvideAppIdType<Chain>)]
pub trait HasAppIdType<Counterparty>: Async {
    type AppId: Async;
}

impl<Chain, Counterparty, Provider, AppId> ProvideAppIdType<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: Async,
    Provider: HasType<AppIdTypeComponent, Type = AppId>,
    AppId: Async,
{
    type AppId = AppId;
}
