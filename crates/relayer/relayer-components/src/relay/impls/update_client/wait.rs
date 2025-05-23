use alloc::vec::Vec;

use hermes_chain_components::traits::{HasHeightType, HasMessageType};
use hermes_logging_components::traits::CanLog;
use hermes_prelude::{CanRaiseAsyncError, *};

use crate::chain::impls::CanWaitChainReachHeight;
use crate::chain::types::aliases::HeightOf;
use crate::relay::traits::{
    CounterpartyChainOf, HasTargetChainTypes, HasTargetChains, RelayTarget,
    TargetUpdateClientMessageBuilder, TargetUpdateClientMessageBuilderComponent,
};

pub enum LogWaitUpdateClientHeightStatus<'a, Relay, Target>
where
    Target: RelayTarget,
    Relay: HasTargetChainTypes<Target, CounterpartyChain: HasHeightType>,
{
    Waiting {
        target_height: &'a HeightOf<CounterpartyChainOf<Relay, Target>>,
    },
    HeightReached {
        current_height: &'a HeightOf<CounterpartyChainOf<Relay, Target>>,
        target_height: &'a HeightOf<CounterpartyChainOf<Relay, Target>>,
    },
}

/**
   Wait for the chain to reach a height that is greater than or equal the required height,
   so that the update client proof can be built.
*/
#[cgp_new_provider(TargetUpdateClientMessageBuilderComponent)]
impl<Relay, Target, InUpdateClient, TargetChain, CounterpartyChain>
    TargetUpdateClientMessageBuilder<Relay, Target> for WaitUpdateClient<InUpdateClient>
where
    Target: RelayTarget,
    Relay: HasTargetChains<Target, TargetChain = TargetChain, CounterpartyChain = CounterpartyChain>
        + for<'a> CanLog<LogWaitUpdateClientHeightStatus<'a, Relay, Target>>
        + CanRaiseAsyncError<CounterpartyChain::Error>,
    InUpdateClient: TargetUpdateClientMessageBuilder<Relay, Target>,
    TargetChain: HasMessageType,
    CounterpartyChain: CanWaitChainReachHeight + HasHeightType,
{
    async fn build_target_update_client_messages(
        relay: &Relay,
        target: Target,
        target_height: &CounterpartyChain::Height,
    ) -> Result<Vec<TargetChain::Message>, Relay::Error> {
        let counterparty_chain = relay.counterparty_chain();

        relay
            .log(
                "waiting for counterparty chain to reach height",
                &LogWaitUpdateClientHeightStatus::Waiting { target_height },
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

        relay
            .log(
                "counterparty chain's height is now greater than or equal to target height",
                &LogWaitUpdateClientHeightStatus::HeightReached {
                    target_height,
                    current_height: &current_height,
                },
            )
            .await;

        InUpdateClient::build_target_update_client_messages(relay, target, target_height).await
    }
}
