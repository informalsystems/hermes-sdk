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
use hermes_relayer_components::relay::traits::update_client_message_builder::CanBuildTargetUpdateClientMessage;
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{RuntimeGetter, RuntimeTypeComponent};
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::contexts::logger::ProvideSovereignLogger;
use crate::contexts::sovereign_chain::SovereignChain;

pub struct CosmosToSovereignRelay {
    pub runtime: HermesRuntime,
    pub src_chain: CosmosChain,
    pub dst_chain: SovereignChain,
    pub src_client_id: ClientId,
    pub dst_client_id: ClientId,
    // TODO: Relay fields
}

pub trait CanUseCosmosToSovereignRelay:
    HasRelayChains<SrcChain = CosmosChain, DstChain = SovereignChain>
    + CanRaiseRelayChainErrors
    + CanCreateClient<SourceTarget>
    + CanCreateClient<DestinationTarget>
    + CanBuildTargetUpdateClientMessage<DestinationTarget>
{
}

impl CanUseCosmosToSovereignRelay for CosmosToSovereignRelay {}

// pub trait CanUseClientUpdateMessageBuilder:
//     TargetUpdateClientMessageBuilder<CosmosToSovereignRelay, SourceTarget>
// {
// }

// impl CanUseClientUpdateMessageBuilder for SkipUpdateClient<WaitUpdateClient<BuildUpdateClientMessages>> {}

pub struct CosmosToSovereignRelayComponents;

impl HasComponents for CosmosToSovereignRelay {
    type Components = CosmosToSovereignRelayComponents;
}

delegate_all!(
    IsDefaultRelayComponent,
    DefaultRelayComponents,
    CosmosToSovereignRelayComponents,
);

delegate_components! {
    CosmosToSovereignRelayComponents {
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

impl RuntimeGetter<CosmosToSovereignRelay> for CosmosToSovereignRelayComponents {
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
