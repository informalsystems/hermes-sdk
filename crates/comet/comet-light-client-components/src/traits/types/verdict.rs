use cgp::prelude::*;

#[derive_component(VerdictTypeComponent, ProvideVerdictType<Client>)]
pub trait HasVerdictType: Async {
    type Verdict: Async;
}
