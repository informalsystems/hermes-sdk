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
use hermes_relayer_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::contexts::sovereign_chain::SovereignChain;

pub struct CosmosToSovereignRelay {
    pub runtime: HermesRuntime,
    pub src_chain: CosmosChain,
    pub dst_chain: SovereignChain,
    pub src_client_id: ClientId,
    pub dst_client_id: ClientId,
    // TODO: Relay fields
}

pub struct CosmosToSovereignRelayComponents;

impl HasComponents for CosmosToSovereignRelay {
    type Components = CosmosToSovereignRelayComponents;
}

delegate_components! {
    CosmosToSovereignRelayComponents {
        ErrorTypeComponent: ProvideEyreError,
        ErrorRaiserComponent: RaiseDebugError,
        RuntimeTypeComponent: ProvideHermesRuntime,
        [
            LoggerTypeComponent,
            LoggerFieldComponent,
        ]:
            ProvideTracingLogger,
    }
}

impl ProvideRuntime<CosmosToSovereignRelay> for CosmosToSovereignRelayComponents {
    fn runtime(relay: &CosmosToSovereignRelay) -> &HermesRuntime {
        &relay.runtime
    }
}

impl ProvideRelayChains<CosmosToSovereignRelay> for CosmosToSovereignRelayComponents {
    type SrcChain = CosmosChain;

    type DstChain = SovereignChain;

    type Packet = Packet;

    fn src_chain(relay: &CosmosToSovereignRelay) -> &CosmosChain {
        &relay.src_chain
    }

    fn dst_chain(relay: &CosmosToSovereignRelay) -> &SovereignChain {
        &relay.dst_chain
    }

    fn src_client_id(relay: &CosmosToSovereignRelay) -> &ClientId {
        &relay.src_client_id
    }

    fn dst_client_id(relay: &CosmosToSovereignRelay) -> &ClientId {
        &relay.dst_client_id
    }
}
