use core::fmt::Debug;

use cgp::core::component::UseDelegate;
use hermes_prelude::*;

#[cgp_component {
  name: ClientStatusTypeComponent,
  provider: ProvideClientStatusType,
  context: Chain,
}]
pub trait HasClientStatusType<Counterparty>: Async {
    /**
        The client status of the `Self` chain's client on the `Counterparty` chain
    */
    type ClientStatus: Async + Debug;
}

#[cgp_provider(ClientStatusTypeComponent)]
impl<Chain, Counterparty, Components, Delegate> ProvideClientStatusType<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: Async,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
    Delegate: ProvideClientStatusType<Chain, Counterparty>,
{
    type ClientStatus = Delegate::ClientStatus;
}

#[cgp_provider(ClientStatusTypeComponent)]
impl<Chain, Counterparty, Provider, ClientStatus> ProvideClientStatusType<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, ClientStatusTypeComponent, Type = ClientStatus>,
    ClientStatus: Async + Debug,
{
    type ClientStatus = ClientStatus;
}

#[cgp_component {
  name: ClientStatusMethodsComponent,
  provider: ClientStatusMethods,
  context: Chain,
}]
pub trait HasClientStatusMethods<Counterparty>: HasClientStatusType<Counterparty> {
    /**
        Return if the client status is active
    */
    fn client_status_is_active(client_status: &Self::ClientStatus) -> bool;

    /**
        Return if the client status is expired
    */
    fn client_status_is_expired(client_status: &Self::ClientStatus) -> bool;

    /**
        Return if the client status is frozen
    */
    fn client_status_is_frozen(client_status: &Self::ClientStatus) -> bool;
}

#[cgp_provider(ClientStatusMethodsComponent)]
impl<Chain, Counterparty, Components, Delegate> ClientStatusMethods<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasClientStatusType<Counterparty>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
    Delegate: ClientStatusMethods<Chain, Counterparty>,
{
    /**
        Return if the client status is active
    */
    fn client_status_is_active(client_status: &Chain::ClientStatus) -> bool {
        Delegate::client_status_is_active(client_status)
    }

    /**
        Return if the client status is expired
    */
    fn client_status_is_expired(client_status: &Chain::ClientStatus) -> bool {
        Delegate::client_status_is_expired(client_status)
    }

    /**
        Return if the client status is frozen
    */
    fn client_status_is_frozen(client_status: &Chain::ClientStatus) -> bool {
        Delegate::client_status_is_frozen(client_status)
    }
}
