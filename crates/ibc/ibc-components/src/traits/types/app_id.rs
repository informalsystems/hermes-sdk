use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[derive_component(AppIdTypeComponent, ProvideAppIdType<Chain>)]
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
