use cgp_core::prelude::{async_trait, HasErrorType};
use hermes_relayer_components::build::traits::birelay::HasBiRelayType;
use hermes_relayer_components::build::traits::components::relay_from_chains_builder::RelayFromChainsBuilder;
use hermes_relayer_components::build::traits::target::chain::ChainBuildTarget;
use hermes_relayer_components::build::traits::target::relay::RelayBuildTarget;
use hermes_relayer_components::build::types::aliases::{
    CounterpartyChainId, CounterpartyClientId, RelayError, TargetChain, TargetChainId,
    TargetChainRuntime, TargetClientId,
};
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains};
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_runtime_components::traits::channel::{
    CanCloneSender, CanCreateChannels, HasChannelTypes,
};
use hermes_runtime_components::traits::channel_once::HasChannelOnceTypes;
use hermes_runtime_components::traits::mutex::HasMutex;
use hermes_runtime_components::traits::runtime::HasRuntime;

use crate::batch::traits::config::HasBatchConfig;
use crate::batch::types::aliases::{MessageBatchReceiver, MessageBatchSender};
use crate::batch::worker::CanSpawnBatchMessageWorker;
use crate::build::traits::cache::HasBatchSenderCache;
use crate::build::traits::components::relay_with_batch_builder::CanBuildRelayWithBatch;

pub struct BuildRelayWithBatchWorker;

impl<Build, Target, Relay, SrcChain, DstChain, SrcRuntime, DstRuntime>
    RelayFromChainsBuilder<Build, Target> for BuildRelayWithBatchWorker
where
    Build: HasBiRelayType
        + HasRuntime
        + HasBatchConfig
        + HasErrorType
        + CanBuildRelayWithBatch<Target>,
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
pub trait CanBuildBatchChannel<Target>: HasBiRelayType + HasErrorType
where
    Target: ChainBuildTarget<Self>,
    TargetChain<Self, Target>: HasRuntime,
    TargetChainRuntime<Self, Target>: HasChannelTypes + HasChannelOnceTypes,
{
    async fn build_batch_channel(
        &self,
        target: Target,
        chain_id: &TargetChainId<Self, Target>,
        counterparty_chain_id: &CounterpartyChainId<Self, Target>,
        client_id: &TargetClientId<Self, Target>,
        counterparty_client_id: &CounterpartyClientId<Self, Target>,
    ) -> Result<
        (
            MessageBatchSender<TargetChain<Self, Target>, RelayError<Self>>,
            Option<MessageBatchReceiver<TargetChain<Self, Target>, RelayError<Self>>>,
        ),
        Self::Error,
    >;
}

impl<Build, Target, Chain, Counterparty, Runtime> CanBuildBatchChannel<Target> for Build
where
    Build: HasBiRelayType + HasRuntime + HasErrorType,
    Target: ChainBuildTarget<Build, TargetChain = Chain, CounterpartyChain = Counterparty>,
    Chain: HasIbcChainTypes<Counterparty> + HasRuntime<Runtime = Runtime>,
    Counterparty: HasIbcChainTypes<Chain>,
    Runtime: CanCreateChannels + HasChannelOnceTypes + CanCloneSender + HasErrorType,
    Build: HasBatchSenderCache<Target, RelayError<Build>>,
    Build::Runtime: HasMutex,
    Chain::ChainId: Ord + Clone,
    Counterparty::ChainId: Ord + Clone,
    Chain::ClientId: Ord + Clone,
    Counterparty::ClientId: Ord + Clone,
{
    async fn build_batch_channel(
        &self,
        target: Target,
        chain_id: &Chain::ChainId,
        counterparty_chain_id: &Counterparty::ChainId,
        client_id: &Chain::ClientId,
        counterparty_client_id: &Counterparty::ClientId,
    ) -> Result<
        (
            MessageBatchSender<Chain, RelayError<Self>>,
            Option<MessageBatchReceiver<Chain, RelayError<Self>>>,
        ),
        Self::Error,
    > {
        let mutex = self.batch_sender_cache(target);

        let mut sender_cache = Build::Runtime::acquire_mutex(mutex).await;

        let cache_key = (
            chain_id.clone(),
            counterparty_chain_id.clone(),
            client_id.clone(),
            counterparty_client_id.clone(),
        );

        if let Some(sender) = sender_cache.get(&cache_key) {
            Ok((Runtime::clone_sender(sender), None))
        } else {
            let (sender, receiver) = Runtime::new_channel();
            sender_cache.insert(cache_key, Runtime::clone_sender(&sender));
            Ok((sender, Some(receiver)))
        }
    }
}
