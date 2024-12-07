use cgp::prelude::*;

#[cgp_component {
  name: AnyCounterpartyComponent,
  provider: ProvideAnyCounterparty,
  context: App,
}]
pub trait HasAnyCounterparty: Async {
    type AnyCounterparty: Async;
}
