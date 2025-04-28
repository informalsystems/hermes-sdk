use alloc::collections::BTreeSet;
use alloc::sync::Arc;
use core::ops::Deref;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent};
use cgp::core::field::Index;
use cgp::core::types::WithType;
use futures::lock::Mutex;
use hermes_core::logging_components::traits::LoggerComponent;
use hermes_core::relayer_components::error::traits::{CanPerformRetry, RetryableErrorComponent};
use hermes_core::relayer_components::multi::traits::chain_at::{
    ChainAt, ChainGetterAtComponent, ChainTypeProviderAtComponent,
};
use hermes_core::relayer_components::multi::traits::client_id_at::ClientIdAtGetterComponent;
use hermes_core::relayer_components::multi::traits::relay_at::ClientIdAt;
use hermes_core::relayer_components::multi::types::tags::{Dst, Src};
use hermes_core::relayer_components::relay::impls::{
    PacketMutexGetterComponent, PacketMutexOf, SelectRelayAToB,
};
use hermes_core::relayer_components::relay::traits::{
    CanCreateClient, DestinationTarget, HasDestinationTargetChainTypes, HasRelayClientIds,
    HasSourceTargetChainTypes, SourceTarget, TargetAutoRelayerComponent,
};
use hermes_core::relayer_components_extra::batch::traits::channel::MessageBatchSenderGetterComponent;
use hermes_core::relayer_components_extra::batch::traits::types::{
    CanUseMessageBatchChannel, MessageBatchSenderOf,
};
use hermes_core::relayer_components_extra::components::extra::relay::ExtraRelayPreset;
use hermes_core::runtime_components::traits::{
    HasRuntime, RuntimeGetterComponent, RuntimeOf, RuntimeTypeProviderComponent,
};
use hermes_prelude::*;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_tracing_logging_components::contexts::TracingLogger;
use ibc::core::host::types::identifiers::ClientId;

use crate::contexts::CosmosChain;
use crate::impls::HandleCosmosError;

#[cgp_context(CosmosRelayComponents: ExtraRelayPreset)]
#[derive(Clone)]
pub struct CosmosRelay {
    pub fields: Arc<dyn HasCosmosRelayFields>,
}

#[derive(HasField)]
pub struct CosmosRelayFields {
    pub runtime: RuntimeOf<CosmosRelay>,
    pub chain_a: ChainAt<CosmosRelay, Index<0>>,
    pub chain_b: ChainAt<CosmosRelay, Index<1>>,
    pub client_id_a: ClientIdAt<CosmosRelay, Index<0>, Index<1>>,
    pub client_id_b: ClientIdAt<CosmosRelay, Index<1>, Index<0>>,
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

delegate_components! {
    CosmosRelayComponents {
        [
            ErrorTypeProviderComponent,
            ErrorRaiserComponent,
            ErrorWrapperComponent,
            RetryableErrorComponent,
        ]:
            HandleCosmosError,
        RuntimeTypeProviderComponent: UseType<HermesRuntime>,
        RuntimeGetterComponent: UseField<symbol!("runtime")>,
        LoggerComponent: TracingLogger,
        [
            ChainTypeProviderAtComponent<Index<0>>,
            ChainTypeProviderAtComponent<Index<1>>,
        ]:
            WithType<CosmosChain>,
        ChainGetterAtComponent<Index<0>>:
            UseField<symbol!("chain_a")>,
        ChainGetterAtComponent<Index<1>>:
            UseField<symbol!("chain_b")>,
        ClientIdAtGetterComponent<Index<0>, Index<1>>:
            UseField<symbol!("client_id_a")>,
        ClientIdAtGetterComponent<Index<1>, Index<0>>:
            UseField<symbol!("client_id_b")>,
        PacketMutexGetterComponent:
            UseField<symbol!("packet_lock_mutex")>,
        MessageBatchSenderGetterComponent<Index<0>>:
            UseField<symbol!("message_batch_sender_a")>,
        MessageBatchSenderGetterComponent<Index<1>>:
            UseField<symbol!("message_batch_sender_b")>,
        [
            ChainTypeProviderAtComponent<Src>,
            ChainTypeProviderAtComponent<Dst>,
            ChainGetterAtComponent<Src>,
            ChainGetterAtComponent<Dst>,
            ClientIdAtGetterComponent<Src, Dst>,
            ClientIdAtGetterComponent<Dst, Src>,
            MessageBatchSenderGetterComponent<Src>,
            MessageBatchSenderGetterComponent<Dst>,
        ]:
            SelectRelayAToB,
    }
}

pub trait CanUseCosmosRelay:
    HasRelayClientIds
    + HasRuntime
    + CanPerformRetry
    + CanUseComponent<TargetAutoRelayerComponent, SourceTarget>
    + CanUseComponent<TargetAutoRelayerComponent, DestinationTarget>
    + HasSourceTargetChainTypes
    + HasDestinationTargetChainTypes
    + CanCreateClient<SourceTarget>
    + CanCreateClient<DestinationTarget>
    + CanUseMessageBatchChannel<Src>
    + CanUseMessageBatchChannel<Dst>
{
}

impl CanUseCosmosRelay for CosmosRelay {}
