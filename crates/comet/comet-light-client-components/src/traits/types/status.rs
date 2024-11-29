use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[derive_component(VerificationStatusTypeComponent, ProvideVerificationStatusType<Client>)]
pub trait HasVerificationStatusType: Async {
    type VerificationStatus: Async;
}

impl<Client: Async, Provider, VerificationStatus> ProvideVerificationStatusType<Client>
    for WithProvider<Provider>
where
    Provider: ProvideType<Client, VerificationStatusTypeComponent, Type = VerificationStatus>,
    VerificationStatus: Async,
{
    type VerificationStatus = VerificationStatus;
}
