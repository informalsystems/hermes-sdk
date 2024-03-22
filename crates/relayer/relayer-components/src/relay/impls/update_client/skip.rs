use alloc::vec::Vec;
use core::marker::PhantomData;

use cgp_core::async_trait;

use crate::chain::traits::queries::consensus_state::CanQueryConsensusStateWithLatestHeight;
use crate::chain::traits::types::consensus_state::HasConsensusStateType;
use crate::chain::traits::types::height::HasHeightType;
use crate::chain::types::aliases::HeightOf;
use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::target::{ChainTarget, CounterpartyChainOf};
use crate::relay::traits::update_client_message_builder::UpdateClientMessageBuilder;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;

pub struct SkipUpdateClient<InUpdateClient>(PhantomData<InUpdateClient>);

pub struct LogSkipBuildUpdateClientMessage<'a, Relay, Target>
where
    Relay: HasRelayChains,
    Target: ChainTarget<Relay>,
{
    pub relay: &'a Relay,
    pub target_height: &'a HeightOf<CounterpartyChainOf<Relay, Target>>,
}

#[async_trait]
impl<Relay, Target, InUpdateClient, TargetChain, CounterpartyChain>
    UpdateClientMessageBuilder<Relay, Target> for SkipUpdateClient<InUpdateClient>
where
    Relay: HasRelayChains + HasLogger,
    Target: ChainTarget<Relay, TargetChain = TargetChain, CounterpartyChain = CounterpartyChain>,
    InUpdateClient: UpdateClientMessageBuilder<Relay, Target>,
    CounterpartyChain: HasConsensusStateType<TargetChain> + HasHeightType,
    TargetChain: CanQueryConsensusStateWithLatestHeight<CounterpartyChain>,
    Relay::Logger: for<'a> CanLog<LogSkipBuildUpdateClientMessage<'a, Relay, Target>>,
{
    async fn build_update_client_messages(
        relay: &Relay,
        target: Target,
        target_height: &HeightOf<Target::CounterpartyChain>,
    ) -> Result<Vec<TargetChain::Message>, Relay::Error> {
        let target_chain = Target::target_chain(relay);
        let target_client_id = Target::target_client_id(relay);

        let consensus_state = target_chain
            .query_consensus_state_with_latest_height(target_client_id, target_height)
            .await;

        match consensus_state {
            Ok(_) => {
                relay.logger().log(
                    "skip building update client message, as the target chain already has one at given height",
                    &LogSkipBuildUpdateClientMessage {
                        relay,
                        target_height,
                    }
                ).await;

                Ok(Vec::new())
            }
            Err(_) => {
                InUpdateClient::build_update_client_messages(relay, target, target_height).await
            }
        }
    }
}
