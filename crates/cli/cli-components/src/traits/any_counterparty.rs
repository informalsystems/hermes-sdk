use cgp_core::prelude::*;

#[derive_component(AnyCounterpartyComponent, ProvideAnyCounterparty<App>)]
pub trait HasAnyCounterparty: Async {
    type AnyCounterparty: Async;
}
