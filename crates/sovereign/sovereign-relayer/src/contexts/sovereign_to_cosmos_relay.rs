use cgp_core::prelude::*;
use cgp_core::{delegate_all, ErrorRaiserComponent, ErrorTypeComponent};
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_relayer_components::components::default::relay::{
    DefaultRelayComponents, IsDefaultRelayComponent,
};
use hermes_relayer_components::relay::traits::chains::{
    CanRaiseRelayChainErrors, HasRelayChains, ProvideRelayChains,
};
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{RuntimeGetter, RuntimeTypeComponent};
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::contexts::logger::ProvideSovereignLogger;
use crate::contexts::sovereign_chain::SovereignChain;

pub struct SovereignToCosmosRelay {
    pub runtime: HermesRuntime,
    pub src_chain: SovereignChain,
    pub dst_chain: CosmosChain,
    pub src_client_id: ClientId,
    pub dst_client_id: ClientId,
    // TODO: Relay fields
}

pub trait CanUseSovereignToCosmosRelay:
    HasRelayChains<SrcChain = SovereignChain, DstChain = CosmosChain>
    + CanRaiseRelayChainErrors
    + CanCreateClient<SourceTarget>
    + CanCreateClient<DestinationTarget>
{
}

impl CanUseSovereignToCosmosRelay for SovereignToCosmosRelay {}

pub struct SovereignToCosmosRelayComponents;

impl HasComponents for SovereignToCosmosRelay {
    type Components = SovereignToCosmosRelayComponents;
}

delegate_all!(
    IsDefaultRelayComponent,
    DefaultRelayComponents,
    SovereignToCosmosRelayComponents,
);

delegate_components! {
    SovereignToCosmosRelayComponents {
        ErrorTypeComponent: ProvideEyreError,
        ErrorRaiserComponent: RaiseDebugError,
        RuntimeTypeComponent: ProvideHermesRuntime,
        [
            LoggerTypeComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            ProvideSovereignLogger,
    }
}

impl RuntimeGetter<SovereignToCosmosRelay> for SovereignToCosmosRelayComponents {
    fn runtime(relay: &SovereignToCosmosRelay) -> &HermesRuntime {
        &relay.runtime
    }
}

impl ProvideRelayChains<SovereignToCosmosRelay> for SovereignToCosmosRelayComponents {
    type SrcChain = SovereignChain;

    type DstChain = CosmosChain;

    type Packet = Packet;

    fn src_chain(relay: &SovereignToCosmosRelay) -> &SovereignChain {
        &relay.src_chain
    }

    fn dst_chain(relay: &SovereignToCosmosRelay) -> &CosmosChain {
        &relay.dst_chain
    }

    fn src_client_id(relay: &SovereignToCosmosRelay) -> &ClientId {
        &relay.src_client_id
    }

    fn dst_client_id(relay: &SovereignToCosmosRelay) -> &ClientId {
        &relay.dst_client_id
    }
}
