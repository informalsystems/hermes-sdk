use cgp::prelude::*;

#[derive_component(VerificationStatusTypeComponent, ProvideVerificationStatusType<Client>)]
pub trait HasVerificationStatusType: Async {
    type VerificationStatus: Async;
}
