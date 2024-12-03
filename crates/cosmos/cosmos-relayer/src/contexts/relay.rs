use alloc::collections::BTreeSet;
use alloc::sync::Arc;
use core::ops::Deref;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::field::impls::use_field::UseField;
use cgp::prelude::*;
use futures::lock::Mutex;
use hermes_error::types::Error;
use hermes_logger::ProvideHermesLogger;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_relayer_components::error::impls::retry::ReturnMaxRetry;
use hermes_relayer_components::error::traits::retry::{
    MaxErrorRetryGetterComponent, RetryableErrorComponent,
};
use hermes_relayer_components::multi::traits::chain_at::{
    ChainGetterAtComponent, ChainTypeAtComponent,
};
use hermes_relayer_components::multi::types::tags::{Dst, Src};
use hermes_relayer_components::relay::impls::packet_lock::{
    PacketMutex, PacketMutexGetter, ProvidePacketLockWithMutex,
};
use hermes_relayer_components::relay::traits::chains::RelayClientIdGetter;
use hermes_relayer_components::relay::traits::packet_filter::PacketFilter;
use hermes_relayer_components::relay::traits::packet_lock::PacketLockComponent;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_relayer_components_extra::batch::traits::channel::MessageBatchSenderGetter;
use hermes_relayer_components_extra::components::extra::closures::relay::auto_relayer::CanUseExtraAutoRelayer;
use hermes_relayer_components_extra::components::extra::relay::*;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    ProvideDefaultRuntimeField, RuntimeGetterComponent, RuntimeTypeComponent,
};
use ibc_relayer::config::filter::PacketFilter as PacketFilterConfig;
use ibc_relayer_types::core::ics04_channel::packet::{Packet, Sequence};
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, ClientId, PortId};

use crate::contexts::chain::CosmosChain;
use crate::impls::error::HandleCosmosError;
use crate::types::batch::CosmosBatchSender;

#[derive(Clone)]
pub struct CosmosRelay {
    pub base_relay: Arc<BaseCosmosRelay>,
}

#[derive(HasField)]
pub struct BaseCosmosRelay {
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

impl Deref for CosmosRelay {
    type Target = BaseCosmosRelay;

    fn deref(&self) -> &BaseCosmosRelay {
        &self.base_relay
    }
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
            base_relay: Arc::new(BaseCosmosRelay {
                runtime,
                src_chain,
                dst_chain,
                src_client_id,
                dst_client_id,
                packet_filter,
                src_chain_message_batch_sender,
                dst_chain_message_batch_sender,
                packet_lock_mutex: Arc::new(Mutex::new(BTreeSet::new())),
            }),
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
        [
            RuntimeTypeComponent,
            RuntimeGetterComponent,
        ]:
            ProvideDefaultRuntimeField,
        [
            LoggerTypeComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            ProvideHermesLogger,
        MaxErrorRetryGetterComponent:
            ReturnMaxRetry<3>,
        PacketLockComponent:
            ProvidePacketLockWithMutex,
        [
            ChainTypeAtComponent<Src>,
            ChainGetterAtComponent<Src>,
        ]:
            UseField<symbol!("src_chain")>,
        [
            ChainTypeAtComponent<Dst>,
            ChainGetterAtComponent<Dst>,
        ]:
            UseField<symbol!("dst_chain")>,
    }
}

with_extra_relay_components! {
    delegate_components! {
        CosmosRelayComponents {
            @ExtraRelayComponents: ExtraRelayComponents,
        }
    }
}

impl HasComponents for CosmosRelay {
    type Components = CosmosRelayComponents;
}

impl CanUseExtraAutoRelayer for CosmosRelay {}

impl RelayClientIdGetter<CosmosRelay> for CosmosRelayComponents {
    fn src_client_id(relay: &CosmosRelay) -> &ClientId {
        &relay.src_client_id
    }

    fn dst_client_id(relay: &CosmosRelay) -> &ClientId {
        &relay.dst_client_id
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
    fn packet_mutex(relay: &CosmosRelay) -> &PacketMutex<CosmosRelay> {
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
