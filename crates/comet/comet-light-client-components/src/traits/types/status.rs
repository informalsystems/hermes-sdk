use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use hermes_prelude::*;

#[cgp_component {
  name: VerificationStatusTypeComponent,
  provider: ProvideVerificationStatusType,
  context: Client,
}]
pub trait HasVerificationStatusType: Async {
    type VerificationStatus: Async;
}

#[cgp_provider(VerificationStatusTypeComponent)]
impl<Client: Async, Provider, VerificationStatus> ProvideVerificationStatusType<Client>
    for WithProvider<Provider>
where
    Provider: ProvideType<Client, VerificationStatusTypeComponent, Type = VerificationStatus>,
    VerificationStatus: Async,
{
    type VerificationStatus = VerificationStatus;
}
