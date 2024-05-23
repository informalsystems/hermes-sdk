use alloc::vec::Vec;
use core::marker::PhantomData;

use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;

use crate::chain::impls::wait_chain_reach_height::CanWaitChainReachHeight;
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::types::aliases::HeightOf;
use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::target::{ChainTarget, CounterpartyChainOf};
use crate::relay::traits::update_client_message_builder::TargetUpdateClientMessageBuilder;

/**
   Wait for the chain to reach a height that is greater than or equal the required height,
   so that the update client proof can be built.
*/
pub struct WaitUpdateClient<InUpdateClient>(PhantomData<InUpdateClient>);

pub enum LogWaitUpdateClientHeightStatus<'a, Relay, Target>
where
    Relay: HasRelayChains,
    Target: ChainTarget<Relay>,
{
    Waiting {
        relay: &'a Relay,
        target_height: &'a HeightOf<CounterpartyChainOf<Relay, Target>>,
    },
    HeightReached {
        relay: &'a Relay,
        current_height: &'a HeightOf<CounterpartyChainOf<Relay, Target>>,
        target_height: &'a HeightOf<CounterpartyChainOf<Relay, Target>>,
    },
}

impl<Relay, Target, InUpdateClient, TargetChain, CounterpartyChain>
    TargetUpdateClientMessageBuilder<Relay, Target> for WaitUpdateClient<InUpdateClient>
where
    Relay: HasRelayChains + HasLogger,
    Target: ChainTarget<Relay, TargetChain = TargetChain, CounterpartyChain = CounterpartyChain>,
    InUpdateClient: TargetUpdateClientMessageBuilder<Relay, Target>,
    TargetChain: HasIbcChainTypes<CounterpartyChain>,
    CounterpartyChain: CanWaitChainReachHeight + HasIbcChainTypes<TargetChain>,
    Relay::Logger: for<'a> CanLog<LogWaitUpdateClientHeightStatus<'a, Relay, Target>>,
{
    async fn build_target_update_client_messages(
        relay: &Relay,
        target: Target,
        target_height: &CounterpartyChain::Height,
    ) -> Result<Vec<TargetChain::Message>, Relay::Error> {
        let counterparty_chain = Target::counterparty_chain(relay);
        let logger = relay.logger();

        logger
            .log(
                "waiting for counterparty chain to reach height",
                &LogWaitUpdateClientHeightStatus::Waiting {
                    relay,
                    target_height,
                },
            )
            .await;

        // We wait for the chain to reach the target height, which may have not been reached
        // when IBC messages are built. This is because proofs build at a latest height would
        // require the chain to progress at least one more height before the update client
        // message can be built.
        let current_height = counterparty_chain
            .wait_chain_reach_height(target_height)
            .await
            .map_err(Target::counterparty_chain_error)?;

        logger
            .log(
                "counterparty chain's height is now greater than or equal to target height",
                &LogWaitUpdateClientHeightStatus::HeightReached {
                    relay,
                    target_height,
                    current_height: &current_height,
                },
            )
            .await;

        InUpdateClient::build_target_update_client_messages(relay, target, target_height).await
    }
}
