use hermes_core::chain_components::traits::{
    ClientStatus, ClientStatusMethods, ClientStatusMethodsComponent, ClientStatusTypeComponent,
    HasClientStatusType, ProvideClientStatusType,
};
use hermes_prelude::*;

pub struct ProvideAnyClientStatus;

#[cgp_provider(ClientStatusTypeComponent)]
impl<Chain, Counterparty> ProvideClientStatusType<Chain, Counterparty> for ProvideAnyClientStatus
where
    Chain: Async,
{
    type ClientStatus = ClientStatus;
}

#[cgp_provider(ClientStatusMethodsComponent)]
impl<Chain, Counterparty> ClientStatusMethods<Chain, Counterparty> for ProvideAnyClientStatus
where
    Chain: HasClientStatusType<Counterparty, ClientStatus = ClientStatus>,
{
    fn client_status_is_active(client_status: &ClientStatus) -> bool {
        client_status == &ClientStatus::Active
    }

    fn client_status_is_expired(client_status: &ClientStatus) -> bool {
        client_status == &ClientStatus::Expired
    }

    fn client_status_is_frozen(client_status: &ClientStatus) -> bool {
        client_status == &ClientStatus::Frozen
    }
}
