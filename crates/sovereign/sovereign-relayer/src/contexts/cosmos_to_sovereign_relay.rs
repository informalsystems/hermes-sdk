use std::collections::BTreeSet;
use std::sync::Arc;

use cgp_core::prelude::*;
use cgp_core::{delegate_all, ErrorRaiserComponent, ErrorTypeComponent};
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use futures::lock::Mutex;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_relayer::contexts::logger::ProvideCosmosLogger;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_relayer_components::components::default::closures::relay::packet_relayer::CanUseDefaultPacketRelayer;
use hermes_relayer_components::components::default::relay::{
    DefaultRelayComponents, IsDefaultRelayComponent,
};
use hermes_relayer_components::relay::impls::packet_filters::allow_all::AllowAll;
use hermes_relayer_components::relay::impls::packet_lock::PacketMutex;
use hermes_relayer_components::relay::impls::packet_lock::{
    PacketMutexGetter, ProvidePacketLockWithMutex,
};
use hermes_relayer_components::relay::impls::packet_relayers::general::full_relay::FullCycleRelayer;
use hermes_relayer_components::relay::traits::chains::{
    CanRaiseRelayChainErrors, HasRelayChains, ProvideRelayChains,
};
use hermes_relayer_components::relay::traits::channel::open_handshake::CanRelayChannelOpenHandshake;
use hermes_relayer_components::relay::traits::channel::open_init::CanInitChannel;
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::connection::open_handshake::CanRelayConnectionOpenHandshake;
use hermes_relayer_components::relay::traits::connection::open_init::CanInitConnection;
use hermes_relayer_components::relay::traits::ibc_message_sender::{CanSendIbcMessages, MainSink};
use hermes_relayer_components::relay::traits::packet::HasRelayPacketFields;
use hermes_relayer_components::relay::traits::packet_filter::PacketFilterComponent;
use hermes_relayer_components::relay::traits::packet_lock::{HasPacketLock, PacketLockComponent};
use hermes_relayer_components::relay::traits::packet_relayer::{CanRelayPacket, PacketRelayer};
use hermes_relayer_components::relay::traits::packet_relayers::ack_packet::CanRelayAckPacket;
use hermes_relayer_components::relay::traits::packet_relayers::receive_packet::CanRelayReceivePacket;
use hermes_relayer_components::relay::traits::packet_relayers::timeout_unordered_packet::CanRelayTimeoutUnorderedPacket;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_relayer_components::relay::traits::update_client_message_builder::CanBuildTargetUpdateClientMessage;
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{RuntimeGetter, RuntimeTypeComponent};
use ibc_relayer_types::core::ics04_channel::packet::{Packet, Sequence};
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, ClientId, PortId};

use crate::contexts::sovereign_chain::SovereignChain;

pub struct CosmosToSovereignRelay {
    pub runtime: HermesRuntime,
    pub src_chain: CosmosChain,
    pub dst_chain: SovereignChain,
    pub src_client_id: ClientId,
    pub dst_client_id: ClientId,
    pub packet_lock_mutex: Arc<Mutex<BTreeSet<(ChannelId, PortId, ChannelId, PortId, Sequence)>>>,
}

pub trait CanUseCosmosToSovereignRelay:
    HasRelayChains<SrcChain = CosmosChain, DstChain = SovereignChain>
    + HasRelayPacketFields
    + CanRaiseRelayChainErrors
    + CanCreateClient<SourceTarget>
    + CanCreateClient<DestinationTarget>
    + CanBuildTargetUpdateClientMessage<DestinationTarget>
    + CanBuildTargetUpdateClientMessage<SourceTarget>
    + CanInitConnection
    + CanInitChannel
    + CanSendIbcMessages<MainSink, SourceTarget>
    + CanSendIbcMessages<MainSink, DestinationTarget>
    + CanRelayConnectionOpenHandshake
    + CanRelayChannelOpenHandshake
    + CanRelayReceivePacket
    + CanRelayAckPacket
    + CanRelayTimeoutUnorderedPacket
    + HasPacketLock
    + CanRelayPacket
{
}

impl CanUseDefaultPacketRelayer for CosmosToSovereignRelay {}

impl CanUseCosmosToSovereignRelay for CosmosToSovereignRelay {}

pub trait CanUsePacketRelayer: PacketRelayer<CosmosToSovereignRelay> {}

impl CanUsePacketRelayer for FullCycleRelayer {}

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
            ProvideCosmosLogger,
        PacketLockComponent:
            ProvidePacketLockWithMutex,
        PacketFilterComponent:
            AllowAll,
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

impl PacketMutexGetter<CosmosToSovereignRelay> for CosmosToSovereignRelayComponents {
    fn packet_mutex(relay: &CosmosToSovereignRelay) -> &PacketMutex<CosmosToSovereignRelay> {
        &relay.packet_lock_mutex
    }
}
