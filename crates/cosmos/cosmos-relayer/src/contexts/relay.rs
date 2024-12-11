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
use hermes_relayer_components::multi::types::index::Index;
use hermes_relayer_components::multi::types::tags::{Dst, Src};
use hermes_relayer_components::relay::impls::packet_lock::{
    PacketMutexGetterComponent, PacketMutexOf, ProvidePacketLockWithMutex,
};
use hermes_relayer_components::relay::impls::selector::SelectRelayAToB;
use hermes_relayer_components::relay::traits::auto_relayer::CanAutoRelay;
use hermes_relayer_components::relay::traits::chains::HasRelayClientIds;
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::packet_lock::PacketLockComponent;
use hermes_relayer_components::relay::traits::target::{
    DestinationTarget, HasDestinationTargetChainTypes, HasSourceTargetChainTypes, SourceTarget,
};
use hermes_relayer_components_extra::batch::traits::channel::MessageBatchSenderGetterComponent;
use hermes_relayer_components_extra::batch::traits::types::{
    CanUseMessageBatchChannel, MessageBatchSenderOf,
};
use hermes_relayer_components_extra::components::extra::closures::relay::auto_relayer::CanUseExtraAutoRelayer;
use hermes_relayer_components_extra::components::extra::relay::{
    ExtraRelayPreset, IsExtraRelayPreset,
};
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
    pub chain_a: CosmosChain,
    pub chain_b: CosmosChain,
    pub client_id_a: ClientId,
    pub client_id_b: ClientId,
    pub packet_lock_mutex: PacketMutexOf<CosmosRelay>,
    pub message_batch_sender_a: MessageBatchSenderOf<CosmosRelay, Index<0>>,
    pub message_batch_sender_b: MessageBatchSenderOf<CosmosRelay, Index<1>>,
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
                chain_a: src_chain,
                chain_b: dst_chain,
                client_id_a: src_client_id,
                client_id_b: dst_client_id,
                message_batch_sender_a: src_chain_message_batch_sender,
                message_batch_sender_b: dst_chain_message_batch_sender,
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
            ChainTypeAtComponent<Index<0>>,
            ChainTypeAtComponent<Index<1>>,
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
        ChainGetterAtComponent<Index<0>>:
            UseField<symbol!("chain_a")>,
        ChainGetterAtComponent<Index<1>>:
            UseField<symbol!("chain_b")>,
        ClientIdAtGetterComponent<Src, Dst>:
            UseField<symbol!("client_id_a")>,
        ClientIdAtGetterComponent<Dst, Src>:
            UseField<symbol!("client_id_b")>,
        PacketMutexGetterComponent:
            UseField<symbol!("packet_lock_mutex")>,
        MessageBatchSenderGetterComponent<Index<0>>:
            UseField<symbol!("message_batch_sender_a")>,
        MessageBatchSenderGetterComponent<Index<1>>:
            UseField<symbol!("message_batch_sender_b")>,
        [
            ChainTypeAtComponent<Src>,
            ChainTypeAtComponent<Dst>,
            ChainGetterAtComponent<Src>,
            ChainGetterAtComponent<Dst>,
            MessageBatchSenderGetterComponent<Src>,
            MessageBatchSenderGetterComponent<Dst>,
        ]:
            SelectRelayAToB,
    }
}

impl<Name> DelegateComponent<Name> for CosmosRelayComponents
where
    Self: IsExtraRelayPreset<Name>,
{
    type Delegate = ExtraRelayPreset;
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
