use alloc::sync::Arc;
use core::marker::PhantomData;
use core::time::Duration;

use cgp::core::Async;
use futures::channel::mpsc::unbounded;
use futures::lock::Mutex;
use hermes_chain_type_components::traits::{
    HasChainIdType, HasMessageResponseType, HasMessageType,
};
use hermes_prelude::*;
use hermes_relayer_components::build::traits::builders::relay_from_chains_builder::{
    RelayFromChainsBuilder, RelayFromChainsBuilderComponent,
};
use hermes_relayer_components::chain::traits::{HasChainId, HasClientIdType};
use hermes_relayer_components::chain::types::aliases::{ChainIdOf, ClientIdOf};
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_relayer_components::multi::traits::relay_at::HasRelayTypeAt;
use hermes_relayer_components::relay::traits::{
    CounterpartyChainOf, DestinationTarget, HasDestinationTargetChainTypes, HasRelayChainTypes,
    HasRelayClientIds, HasSourceTargetChainTypes, HasTargetChainTypes, RelayTarget, SourceTarget,
    TargetChainOf,
};

use crate::batch::traits::config::HasBatchConfig;
use crate::batch::traits::types::{
    CanUseMessageBatchChannel, HasMessageBatchChannelTypes, MessageBatchReceiverOf,
    MessageBatchSenderOf,
};
use crate::batch::worker::CanSpawnBatchMessageWorker;
use crate::build::traits::cache::{CanUseBatchSenderCache, HasBatchSenderCache};
use crate::build::traits::relay_with_batch_builder::CanBuildRelayWithBatch;

pub struct BuildRelayWithBatchWorker;

#[cgp_provider(RelayFromChainsBuilderComponent)]
impl<Build, SrcTag: Async, DstTag: Async, Relay, SrcChain, DstChain>
    RelayFromChainsBuilder<Build, SrcTag, DstTag> for BuildRelayWithBatchWorker
where
    Build: HasBatchConfig<SrcTag, SrcChain>
        + HasBatchConfig<DstTag, DstChain>
        + HasRelayTypeAt<SrcTag, DstTag, Relay = Relay>
        + HasChainTypeAt<SrcTag, Chain = SrcChain>
        + HasChainTypeAt<DstTag, Chain = DstChain>
        + HasAsyncErrorType
        + CanBuildRelayWithBatch<SrcTag, DstTag>
        + CanBuildBatchChannel<SrcTag, DstTag, SourceTarget>
        + CanBuildBatchChannel<SrcTag, DstTag, DestinationTarget>,
    Relay: Clone
        + HasRelayChainTypes<SrcChain = SrcChain, DstChain = DstChain>
        + HasSourceTargetChainTypes
        + HasDestinationTargetChainTypes
        + HasRelayClientIds
        + CanSpawnBatchMessageWorker<SourceTarget>
        + CanSpawnBatchMessageWorker<DestinationTarget>,
    SrcChain: HasChainId
        + HasClientIdType<DstChain>
        + HasMessageType
        + HasMessageResponseType
        + HasAsyncErrorType,
    DstChain: HasChainId
        + HasClientIdType<SrcChain>
        + HasMessageType
        + HasMessageResponseType
        + HasAsyncErrorType,
{
    async fn build_relay_from_chains(
        build: &Build,
        index: PhantomData<(SrcTag, DstTag)>,
        src_client_id: &SrcChain::ClientId,
        dst_client_id: &DstChain::ClientId,
        src_chain: SrcChain,
        dst_chain: DstChain,
        refresh_rate_a: Option<Duration>,
        refresh_rate_b: Option<Duration>,
    ) -> Result<Build::Relay, Build::Error> {
        let src_chain_id = src_chain.chain_id();
        let dst_chain_id = dst_chain.chain_id();

        let src_batch_config = build.batch_config(PhantomData::<SrcTag>, src_chain_id)?;
        let dst_batch_config = build.batch_config(PhantomData::<DstTag>, dst_chain_id)?;

        let (src_sender, m_src_receiver) = build
            .build_batch_channel(
                PhantomData::<(SrcTag, DstTag, SourceTarget)>,
                src_chain_id,
                dst_chain_id,
                src_client_id,
                dst_client_id,
            )
            .await?;

        let (dst_sender, m_dst_receiver) = build
            .build_batch_channel(
                PhantomData::<(SrcTag, DstTag, DestinationTarget)>,
                dst_chain_id,
                src_chain_id,
                dst_client_id,
                src_client_id,
            )
            .await?;

        let relay = build
            .build_relay_with_batch(
                index,
                src_client_id,
                dst_client_id,
                src_chain,
                dst_chain,
                src_sender,
                dst_sender,
                refresh_rate_a,
                refresh_rate_b,
            )
            .await?;

        if let Some(src_receiver) = m_src_receiver {
            relay
                .clone()
                .spawn_batch_message_worker(SourceTarget, src_batch_config, src_receiver);
        }

        if let Some(dst_receiver) = m_dst_receiver {
            relay.clone().spawn_batch_message_worker(
                DestinationTarget,
                dst_batch_config,
                dst_receiver,
            );
        }

        Ok(relay)
    }
}

#[async_trait]
pub trait CanBuildBatchChannel<SrcTag: Async, DstTag: Async, Target: RelayTarget>:
    HasAsyncErrorType + CanUseBatchSenderCache<SrcTag, DstTag, Target>
{
    async fn build_batch_channel(
        &self,
        index: PhantomData<(SrcTag, DstTag, Target)>,
        chain_id: &ChainIdOf<TargetChainOf<Self::Relay, Target>>,
        counterparty_chain_id: &ChainIdOf<CounterpartyChainOf<Self::Relay, Target>>,
        client_id: &ClientIdOf<
            TargetChainOf<Self::Relay, Target>,
            CounterpartyChainOf<Self::Relay, Target>,
        >,
        counterparty_client_id: &ClientIdOf<
            CounterpartyChainOf<Self::Relay, Target>,
            TargetChainOf<Self::Relay, Target>,
        >,
    ) -> Result<
        (
            MessageBatchSenderOf<Self::Relay, Target::Chain>,
            Option<MessageBatchReceiverOf<Self::Relay, Target::Chain>>,
        ),
        Self::Error,
    >;
}

impl<
        Build,
        SrcTag: Async,
        DstTag: Async,
        Target: RelayTarget,
        Relay,
        TargetChain,
        CounterpartyChain,
    > CanBuildBatchChannel<SrcTag, DstTag, Target> for Build
where
    Build: CanUseBatchSenderCache<SrcTag, DstTag, Target>
        + HasBatchSenderCache<SrcTag, DstTag, Target>
        + HasRelayTypeAt<SrcTag, DstTag, Relay = Relay>
        + HasAsyncErrorType,
    Relay: HasTargetChainTypes<
            Target,
            TargetChain = TargetChain,
            CounterpartyChain = CounterpartyChain,
        > + HasMessageBatchChannelTypes<Target::Chain>
        + CanUseMessageBatchChannel<Target::Chain>,
    TargetChain: HasChainIdType<ChainId: Ord + Clone>
        + HasClientIdType<CounterpartyChain, ClientId: Ord + Clone>
        + HasMessageType
        + HasMessageResponseType,
    CounterpartyChain:
        HasChainIdType<ChainId: Ord + Clone> + HasClientIdType<TargetChain, ClientId: Ord + Clone>,
{
    async fn build_batch_channel(
        &self,
        index: PhantomData<(SrcTag, DstTag, Target)>,
        chain_id: &TargetChain::ChainId,
        counterparty_chain_id: &CounterpartyChain::ChainId,
        client_id: &TargetChain::ClientId,
        counterparty_client_id: &CounterpartyChain::ClientId,
    ) -> Result<
        (
            Relay::MessageBatchSender,
            Option<Relay::MessageBatchReceiver>,
        ),
        Build::Error,
    > {
        let mut sender_cache = self.batch_sender_cache(index).lock().await;

        let cache_key = (
            chain_id.clone(),
            counterparty_chain_id.clone(),
            client_id.clone(),
            counterparty_client_id.clone(),
        );

        if let Some(sender) = sender_cache.get(&cache_key) {
            Ok((sender.clone(), None))
        } else {
            let (sender, receiver) = unbounded();
            let sender = Arc::new(Mutex::new(sender));

            sender_cache.insert(cache_key, sender.clone());
            Ok((sender, Some(receiver)))
        }
    }
}
