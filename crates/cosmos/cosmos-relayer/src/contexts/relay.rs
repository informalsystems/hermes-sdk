use alloc::collections::BTreeSet;
use alloc::sync::Arc;
use core::ops::Deref;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::field::impls::use_field::UseField;
use cgp::core::types::impls::WithType;
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
use hermes_relayer_components::multi::traits::client_id_at::ClientIdAtGetterComponent;
use hermes_relayer_components::multi::types::tags::{Dst, Src};
use hermes_relayer_components::relay::impls::packet_lock::{
    PacketMutex, PacketMutexGetter, ProvidePacketLockWithMutex,
};
use hermes_relayer_components::relay::traits::auto_relayer::CanAutoRelay;
use hermes_relayer_components::relay::traits::chains::HasRelayClientIds;
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::packet_filter::PacketFilter;
use hermes_relayer_components::relay::traits::packet_lock::PacketLockComponent;
use hermes_relayer_components::relay::traits::target::{
    DestinationTarget, HasDestinationTargetChainTypes, HasSourceTargetChainTypes, SourceTarget,
};
use hermes_relayer_components_extra::batch::traits::channel::MessageBatchSenderGetterComponent;
use hermes_relayer_components_extra::batch::traits::types::{
    CanUseMessageBatchChannel, MessageBatchSenderOf,
};
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

#[derive(Clone)]
pub struct CosmosRelay {
    pub fields: Arc<dyn HasCosmosRelayFields>,
}

#[derive(HasField)]
pub struct CosmosRelayFields {
    pub runtime: HermesRuntime,
    pub src_chain: CosmosChain,
    pub dst_chain: CosmosChain,
    pub src_client_id: ClientId,
    pub dst_client_id: ClientId,
    pub packet_filter: PacketFilterConfig,
    pub packet_lock_mutex: Arc<Mutex<BTreeSet<(ChannelId, PortId, ChannelId, PortId, Sequence)>>>,
    pub src_chain_message_batch_sender: MessageBatchSenderOf<CosmosRelay, Src>,
    pub dst_chain_message_batch_sender: MessageBatchSenderOf<CosmosRelay, Dst>,
}

pub trait HasCosmosRelayFields: Send + Sync + 'static {
    fn cosmos_relay_fields(&self) -> &CosmosRelayFields;
}

impl HasCosmosRelayFields for CosmosRelayFields {
    fn cosmos_relay_fields(&self) -> &CosmosRelayFields {
        self
    }
}

impl Deref for CosmosRelay {
    type Target = CosmosRelayFields;

    fn deref(&self) -> &CosmosRelayFields {
        &self.fields.cosmos_relay_fields()
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
        src_chain_message_batch_sender: MessageBatchSenderOf<CosmosRelay, Src>,
        dst_chain_message_batch_sender: MessageBatchSenderOf<CosmosRelay, Dst>,
    ) -> Self {
        let relay = Self {
            fields: Arc::new(CosmosRelayFields {
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
        RuntimeTypeComponent: WithType<HermesRuntime>,
        [
            ChainTypeAtComponent<Src>,
            ChainTypeAtComponent<Dst>,
        ]:
            WithType<CosmosChain>,
        RuntimeGetterComponent:
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
        ChainGetterAtComponent<Src>:
            UseField<symbol!("src_chain")>,
        ChainGetterAtComponent<Dst>:
            UseField<symbol!("dst_chain")>,
        ClientIdAtGetterComponent<Src, Dst>:
            UseField<symbol!("src_client_id")>,
        ClientIdAtGetterComponent<Dst, Src>:
            UseField<symbol!("dst_client_id")>,
        MessageBatchSenderGetterComponent<Src>:
            UseField<symbol!("src_chain_message_batch_sender")>,
        MessageBatchSenderGetterComponent<Dst>:
            UseField<symbol!("dst_chain_message_batch_sender")>,
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

pub trait CanUseCosmosRelay:
    HasRelayClientIds
    + CanAutoRelay<SourceTarget>
    + CanAutoRelay<DestinationTarget>
    + HasSourceTargetChainTypes
    + HasDestinationTargetChainTypes
    + CanCreateClient<SourceTarget>
    + CanCreateClient<DestinationTarget>
    + CanUseMessageBatchChannel<Src>
    + CanUseMessageBatchChannel<Dst>
{
}

impl CanUseCosmosRelay for CosmosRelay {}
