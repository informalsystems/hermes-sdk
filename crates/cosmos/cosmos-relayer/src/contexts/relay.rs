use alloc::collections::BTreeSet;
use alloc::sync::Arc;
use core::ops::Deref;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::field::impls::use_field::{UseField, WithField};
use cgp::core::types::impls::WithType;
use cgp::prelude::*;
use futures::lock::Mutex;
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
use hermes_relayer_components::relay::impls::packet_filters::chain::FilterRelayPacketWithChains;
use hermes_relayer_components::relay::impls::packet_lock::{
    PacketMutexGetterComponent, PacketMutexOf, ProvidePacketLockWithMutex,
};
use hermes_relayer_components::relay::traits::auto_relayer::CanAutoRelay;
use hermes_relayer_components::relay::traits::chains::HasRelayClientIds;
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::packet_filter::RelayPacketFilterComponent;
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
use hermes_runtime_components::traits::runtime::{RuntimeGetterComponent, RuntimeTypeComponent};
use ibc::core::host::types::identifiers::ClientId;

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
    pub packet_lock_mutex: PacketMutexOf<CosmosRelay>,
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
        self.fields.cosmos_relay_fields()
    }
}

impl CosmosRelay {
    pub fn new(
        runtime: HermesRuntime,
        src_chain: CosmosChain,
        dst_chain: CosmosChain,
        src_client_id: ClientId,
        dst_client_id: ClientId,
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
        RuntimeGetterComponent: WithField<symbol!("runtime")>,
        [
            ChainTypeAtComponent<Src>,
            ChainTypeAtComponent<Dst>,
        ]:
            WithType<CosmosChain>,
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
        PacketMutexGetterComponent:
            UseField<symbol!("packet_lock_mutex")>,
        MessageBatchSenderGetterComponent<Src>:
            UseField<symbol!("src_chain_message_batch_sender")>,
        MessageBatchSenderGetterComponent<Dst>:
            UseField<symbol!("dst_chain_message_batch_sender")>,
        RelayPacketFilterComponent:
            FilterRelayPacketWithChains,
    }
}

with_extra_relay_components! {
    | Components | {
        delegate_components! {
            CosmosRelayComponents {
                Components: ExtraRelayComponents,
            }
        }
    }
}

impl HasComponents for CosmosRelay {
    type Components = CosmosRelayComponents;
}

impl CanUseExtraAutoRelayer for CosmosRelay {}

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
