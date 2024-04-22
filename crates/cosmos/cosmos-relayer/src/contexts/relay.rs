use alloc::collections::BTreeSet;
use alloc::sync::Arc;

use cgp_core::prelude::*;
use cgp_core::{delegate_all, ErrorRaiserComponent, ErrorTypeComponent};
use futures::lock::Mutex;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_relayer_components::error::impls::retry::ReturnMaxRetry;
use hermes_relayer_components::error::traits::retry::{
    MaxErrorRetryGetterComponent, RetryableErrorComponent,
};
use hermes_relayer_components::relay::impls::packet_lock::PacketMutexGetter;
use hermes_relayer_components::relay::impls::packet_lock::ProvidePacketLockWithMutex;
use hermes_relayer_components::relay::traits::chains::ProvideRelayChains;
use hermes_relayer_components::relay::traits::packet_filter::PacketFilter;
use hermes_relayer_components::relay::traits::packet_lock::PacketLockComponent;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_relayer_components_extra::batch::traits::channel::MessageBatchSenderGetter;
use hermes_relayer_components_extra::components::extra::closures::relay::auto_relayer::CanUseExtraAutoRelayer;
use hermes_relayer_components_extra::components::extra::relay::{
    ExtraRelayComponents, IsExtraRelayComponent,
};
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::RuntimeGetter;
use hermes_runtime_components::traits::runtime::RuntimeTypeComponent;
use ibc_relayer::config::filter::PacketFilter as PacketFilterConfig;
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, ClientId, PortId};

use crate::types::error::Error;

use crate::contexts::logger::ProvideCosmosLogger;
use crate::impls::error::HandleCosmosError;

use crate::contexts::chain::CosmosChain;
use crate::types::batch::CosmosBatchSender;

#[derive(Clone)]
pub struct CosmosRelay {
    pub runtime: HermesRuntime,
    pub src_chain: CosmosChain,
    pub dst_chain: CosmosChain,
    pub src_client_id: ClientId,
    pub dst_client_id: ClientId,
    pub packet_filter: PacketFilterConfig,
    pub packet_lock_mutex: Arc<Mutex<BTreeSet<(ChannelId, PortId, ChannelId, PortId, Sequence)>>>,
    pub src_chain_message_batch_sender: CosmosBatchSender,
    pub dst_chain_message_batch_sender: CosmosBatchSender,
}

impl CosmosRelay {
    pub fn new(
        runtime: HermesRuntime,
        src_chain: CosmosChain,
        dst_chain: CosmosChain,
        src_client_id: ClientId,
        dst_client_id: ClientId,
        packet_filter: PacketFilterConfig,
        src_chain_message_batch_sender: CosmosBatchSender,
        dst_chain_message_batch_sender: CosmosBatchSender,
    ) -> Self {
        let relay = Self {
            runtime,
            src_chain,
            dst_chain,
            src_client_id,
            dst_client_id,
            packet_filter,
            src_chain_message_batch_sender,
            dst_chain_message_batch_sender,
            packet_lock_mutex: Arc::new(Mutex::new(BTreeSet::new())),
        };

        relay
    }
}

pub struct CosmosRelayComponents;

delegate_components! {
    CosmosRelayComponents {
        [
            ErrorTypeComponent,
            ErrorRaiserComponent,
            RetryableErrorComponent,
        ]:
            HandleCosmosError,
        RuntimeTypeComponent:
            ProvideHermesRuntime,
        [
            LoggerTypeComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            ProvideCosmosLogger,
        MaxErrorRetryGetterComponent:
            ReturnMaxRetry<3>,
        PacketLockComponent:
            ProvidePacketLockWithMutex,
    }
}

delegate_all!(
    IsExtraRelayComponent,
    ExtraRelayComponents,
    CosmosRelayComponents,
);

impl HasComponents for CosmosRelay {
    type Components = CosmosRelayComponents;
}

impl CanUseExtraAutoRelayer for CosmosRelay {}

impl ProvideRelayChains<CosmosRelay> for CosmosRelayComponents {
    type SrcChain = CosmosChain;

    type DstChain = CosmosChain;

    type Packet = Packet;

    fn src_chain(relay: &CosmosRelay) -> &CosmosChain {
        &relay.src_chain
    }

    fn dst_chain(relay: &CosmosRelay) -> &CosmosChain {
        &relay.dst_chain
    }

    fn src_client_id(relay: &CosmosRelay) -> &ClientId {
        &relay.src_client_id
    }

    fn dst_client_id(relay: &CosmosRelay) -> &ClientId {
        &relay.dst_client_id
    }
}

impl RuntimeGetter<CosmosRelay> for CosmosRelayComponents {
    fn runtime(relay: &CosmosRelay) -> &HermesRuntime {
        &relay.runtime
    }
}

impl PacketFilter<CosmosRelay> for CosmosRelayComponents {
    async fn should_relay_packet(relay: &CosmosRelay, packet: &Packet) -> Result<bool, Error> {
        Ok(relay
            .packet_filter
            .channel_policy
            .is_allowed(&packet.source_port, &packet.source_channel))
    }
}

impl PacketMutexGetter<CosmosRelay> for CosmosRelayComponents {
    fn packet_mutex(
        relay: &CosmosRelay,
    ) -> &hermes_relayer_components::relay::impls::packet_lock::PacketMutex<CosmosRelay> {
        &relay.packet_lock_mutex
    }
}

impl MessageBatchSenderGetter<CosmosRelay, SourceTarget> for CosmosRelayComponents {
    fn get_batch_sender(relay: &CosmosRelay) -> &CosmosBatchSender {
        &relay.src_chain_message_batch_sender
    }
}

impl MessageBatchSenderGetter<CosmosRelay, DestinationTarget> for CosmosRelayComponents {
    fn get_batch_sender(relay: &CosmosRelay) -> &CosmosBatchSender {
        &relay.dst_chain_message_batch_sender
    }
}
