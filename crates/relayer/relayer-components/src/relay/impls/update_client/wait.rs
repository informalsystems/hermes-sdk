use alloc::vec::Vec;
use core::marker::PhantomData;

use cgp::prelude::CanRaiseError;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::message::HasMessageType;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;

use crate::chain::impls::wait_chain_reach_height::CanWaitChainReachHeight;
use crate::chain::types::aliases::HeightOf;
use crate::relay::traits::target::{
    CounterpartyChainOf, HasTargetChainTypes, HasTargetChains, RelayTarget,
};
use crate::relay::traits::update_client_message_builder::TargetUpdateClientMessageBuilder;

/**
   Wait for the chain to reach a height that is greater than or equal the required height,
   so that the update client proof can be built.
*/
pub struct WaitUpdateClient<InUpdateClient>(PhantomData<InUpdateClient>);

pub enum LogWaitUpdateClientHeightStatus<'a, Relay, Target>
where
    Target: RelayTarget,
    Relay: HasTargetChainTypes<Target, CounterpartyChain: HasHeightType>,
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
    Target: RelayTarget,
    Relay: HasLogger
        + HasTargetChains<Target, TargetChain = TargetChain, CounterpartyChain = CounterpartyChain>
        + CanRaiseError<CounterpartyChain::Error>,
    InUpdateClient: TargetUpdateClientMessageBuilder<Relay, Target>,
    TargetChain: HasMessageType,
    CounterpartyChain: CanWaitChainReachHeight + HasHeightType,
    Relay::Logger: for<'a> CanLog<LogWaitUpdateClientHeightStatus<'a, Relay, Target>>,
{
    async fn build_target_update_client_messages(
        relay: &Relay,
        target: Target,
        target_height: &CounterpartyChain::Height,
    ) -> Result<Vec<TargetChain::Message>, Relay::Error> {
        let counterparty_chain = relay.counterparty_chain();
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
            .map_err(Relay::raise_error)?;

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
