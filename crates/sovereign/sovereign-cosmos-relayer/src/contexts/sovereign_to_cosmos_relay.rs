use cgp_core::prelude::*;
use cgp_core::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_relayer_components::logger::traits::has_logger::{
    LoggerFieldComponent, LoggerTypeComponent,
};
use hermes_relayer_components::relay::traits::chains::ProvideRelayChains;
use hermes_relayer_components::runtime::traits::runtime::{ProvideRuntime, RuntimeTypeComponent};
use hermes_relayer_runtime::impls::logger::components::ProvideTracingLogger;
use hermes_relayer_runtime::impls::types::runtime::ProvideTokioRuntimeType;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::contexts::sovereign_chain::SovereignChain;

pub struct SovereignToCosmosRelay {
    pub runtime: HermesRuntime,
    pub src_chain: SovereignChain,
    pub dst_chain: CosmosChain,
    pub src_client_id: ClientId,
    pub dst_client_id: ClientId,
    // TODO: Relay fields
}

pub struct SovereignToCosmosRelayComponents;

impl HasComponents for SovereignToCosmosRelay {
    type Components = SovereignToCosmosRelayComponents;
}

delegate_components! {
    SovereignToCosmosRelayComponents {
        ErrorTypeComponent: ProvideEyreError,
        ErrorRaiserComponent: RaiseDebugError,
        RuntimeTypeComponent: ProvideTokioRuntimeType,
        [
            LoggerTypeComponent,
            LoggerFieldComponent,
        ]:
            ProvideTracingLogger,
    }
}

impl ProvideRuntime<SovereignToCosmosRelay> for SovereignToCosmosRelayComponents {
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
