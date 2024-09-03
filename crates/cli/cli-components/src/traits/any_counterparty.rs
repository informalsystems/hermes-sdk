use cgp::prelude::*;

#[derive_component(AnyCounterpartyComponent, ProvideAnyCounterparty<App>)]
pub trait HasAnyCounterparty: Async {
    type AnyCounterparty: Async;
}
