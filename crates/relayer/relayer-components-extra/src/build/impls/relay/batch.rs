use cgp_core::error::ErrorOf;
use cgp_core::prelude::{async_trait, HasErrorType};
use cgp_core::Async;
use hermes_relayer_components::build::traits::builders::relay_from_chains_builder::RelayFromChainsBuilder;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::multi::traits::chain_at::{ChainIdAt, ChainTypeAt, HasChainTypeAt};
use hermes_relayer_components::multi::traits::relay_at::{ClientIdAt, HasRelayTypeAt};
use hermes_relayer_components::multi::types::index::Twindex;
use hermes_relayer_components::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains};
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_runtime_components::traits::channel::{
    CanCloneSender, CanCreateChannels, HasChannelTypes,
};
use hermes_runtime_components::traits::channel_once::HasChannelOnceTypes;
use hermes_runtime_components::traits::mutex::HasMutex;
use hermes_runtime_components::traits::runtime::HasRuntime;

use crate::batch::traits::channel::HasMessageBatchSenderType;
use crate::batch::traits::config::HasBatchConfig;
use crate::batch::types::aliases::{MessageBatchReceiver, MessageBatchSender};
use crate::batch::worker::CanSpawnBatchMessageWorker;
use crate::build::traits::cache::HasBatchSenderCache;
use crate::build::traits::relay_with_batch_builder::CanBuildRelayWithBatch;

pub struct BuildRelayWithBatchWorker;

impl<Build, const SRC: usize, const DST: usize> RelayFromChainsBuilder<Build, SRC, DST>
    for BuildRelayWithBatchWorker
where
    Build: CanBuildRelayWithBatch<SRC, DST> + HasRuntime + HasBatchConfig,
    Build:
        CanBuildBatchChannel<Target::SrcChainTarget> + CanBuildBatchChannel<Target::DstChainTarget>,
    Target: RelayBuildTarget<Build, TargetRelay = Relay>,
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain, Error = RelayError<Build>>,
    Relay: Clone
        + CanSpawnBatchMessageWorker<SourceTarget>
        + CanSpawnBatchMessageWorker<DestinationTarget>
        + CanRaiseRelayChainErrors,
    SrcChain: HasIbcChainTypes<DstChain> + HasErrorType,
    DstChain: HasIbcChainTypes<SrcChain> + HasErrorType,
    SrcChain: HasRuntime<Runtime = SrcRuntime> + HasChainId,
    DstChain: HasRuntime<Runtime = DstRuntime> + HasChainId,
    SrcRuntime: HasChannelTypes + HasChannelOnceTypes + HasErrorType,
    DstRuntime: HasChannelTypes + HasChannelOnceTypes + HasErrorType,
{
    async fn build_relay_from_chains(
        build: &Build,
        _target: Target,
        src_client_id: &SrcChain::ClientId,
        dst_client_id: &DstChain::ClientId,
        src_chain: SrcChain,
        dst_chain: DstChain,
    ) -> Result<Target::TargetRelay, Build::Error> {
        let src_chain_id = src_chain.chain_id();
        let dst_chain_id = dst_chain.chain_id();

        let (src_sender, m_src_receiver) = build
            .build_batch_channel(
                Target::SrcChainTarget::default(),
                src_chain_id,
                dst_chain_id,
                src_client_id,
                dst_client_id,
            )
            .await?;

        let (dst_sender, m_dst_receiver) = build
            .build_batch_channel(
                Target::DstChainTarget::default(),
                dst_chain_id,
                src_chain_id,
                dst_client_id,
                src_client_id,
            )
            .await?;

        let relay = build
            .build_relay_with_batch(
                Target::default(),
                src_client_id,
                dst_client_id,
                src_chain,
                dst_chain,
                src_sender,
                dst_sender,
            )
            .await?;

        if let Some(src_receiver) = m_src_receiver {
            relay.clone().spawn_batch_message_worker(
                SourceTarget,
                build.batch_config().clone(),
                src_receiver,
            );
        }

        if let Some(dst_receiver) = m_dst_receiver {
            relay.clone().spawn_batch_message_worker(
                DestinationTarget,
                build.batch_config().clone(),
                dst_receiver,
            );
        }

        Ok(relay)
    }
}

#[async_trait]
pub trait CanBuildBatchChannel<Error, const TARGET: usize, const COUNTERPARTY: usize>:
    HasChainTypeAt<TARGET, Chain: HasRuntime<Runtime: HasChannelTypes + HasChannelOnceTypes>>
    + HasChainTypeAt<COUNTERPARTY>
    + HasErrorType
{
    async fn build_batch_channel(
        &self,
        index: Twindex<TARGET, COUNTERPARTY>,
        chain_id: &ChainIdAt<Self, TARGET>,
        counterparty_chain_id: &ChainIdAt<Self, COUNTERPARTY>,
        client_id: &ClientIdAt<Self, TARGET, COUNTERPARTY>,
        counterparty_client_id: &ClientIdAt<Self, COUNTERPARTY, TARGET>,
    ) -> Result<
        (
            MessageBatchSender<ChainTypeAt<Self, TARGET>, Error>,
            Option<MessageBatchReceiver<ChainTypeAt<Self, TARGET>, Error>>,
        ),
        Self::Error,
    >;
}

impl<Build, Error, Chain, Counterparty, const TARGET: usize, const COUNTERPARTY: usize>
    CanBuildBatchChannel<Error, TARGET, COUNTERPARTY> for Build
where
    Build: HasChainTypeAt<TARGET, Chain = Chain>
        + HasChainTypeAt<COUNTERPARTY, Chain = Counterparty>
        + HasBatchSenderCache<Error, TARGET, COUNTERPARTY>
        + HasRuntime
        + HasErrorType,
    Chain: HasIbcChainTypes<Counterparty> + HasRuntime,
    Counterparty: HasIbcChainTypes<Chain>,
    Chain::Runtime: CanCreateChannels + HasChannelOnceTypes + CanCloneSender + HasErrorType,
    Build::Runtime: HasMutex,
    Chain::ChainId: Ord + Clone,
    Counterparty::ChainId: Ord + Clone,
    Chain::ClientId: Ord + Clone,
    Counterparty::ClientId: Ord + Clone,
    Error: Async,
{
    async fn build_batch_channel(
        &self,
        index: Twindex<TARGET, COUNTERPARTY>,
        chain_id: &Chain::ChainId,
        counterparty_chain_id: &Counterparty::ChainId,
        client_id: &Chain::ClientId,
        counterparty_client_id: &Counterparty::ClientId,
    ) -> Result<
        (
            MessageBatchSender<Chain, Error>,
            Option<MessageBatchReceiver<Chain, Error>>,
        ),
        Self::Error,
    > {
        let mutex = self.batch_sender_cache(index);

        let mut sender_cache = Build::Runtime::acquire_mutex(mutex).await;

        let cache_key = (
            chain_id.clone(),
            counterparty_chain_id.clone(),
            client_id.clone(),
            counterparty_client_id.clone(),
        );

        if let Some(sender) = sender_cache.get(&cache_key) {
            Ok((Chain::Runtime::clone_sender(sender), None))
        } else {
            let (sender, receiver) = Chain::Runtime::new_channel();
            sender_cache.insert(cache_key, Chain::Runtime::clone_sender(&sender));
            Ok((sender, Some(receiver)))
        }
    }
}
