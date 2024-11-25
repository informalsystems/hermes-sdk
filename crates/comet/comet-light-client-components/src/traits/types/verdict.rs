use cgp::prelude::*;

#[derive_component(VerdictTypeComponent, ProvideVerdictType<Chain>)]
pub trait HasVerdictType: Async {
    type Verdict: Async;
}
