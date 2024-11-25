use cgp::prelude::*;

#[derive_component(VerificationStatusTypeComponent, ProvideVerificationStatusType<Chain>)]
pub trait HasVerificationStatusType: Async {
    type VerificationStatus: Async;
}
