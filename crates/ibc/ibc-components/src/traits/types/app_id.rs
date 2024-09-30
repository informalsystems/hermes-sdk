use cgp::prelude::*;

#[derive_component(AppIdTypeComponent, ProvideAppIdType<Chain>)]
pub trait HasAppIdType<Counterparty>: Async {
    type AppId: Async;
}
