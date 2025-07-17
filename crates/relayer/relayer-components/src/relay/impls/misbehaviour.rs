use alloc::{format, vec};

use hermes_chain_components::traits::{
    CanBuildMisbehaviourMessage, CanCheckMisbehaviour, EmptyMessageResponse, HasEvidenceType,
    HasUpdateClientEvent,
};
use hermes_logging_components::traits::CanLog;
use hermes_logging_components::types::{LevelInfo, LevelTrace};
use hermes_prelude::*;

use crate::relay::traits::{
    CanSendIbcMessages, HasTargetChainTypes, MainSink, MisbehaviourCheckerAndSubmitter,
    MisbehaviourCheckerAndSubmitterComponent, RelayTarget,
};

pub struct CheckAndSubmitMisbehaviourWithChains;

#[cgp_provider(MisbehaviourCheckerAndSubmitterComponent)]
impl<Relay, Target, TargetChain, CounterpartyChain> MisbehaviourCheckerAndSubmitter<Relay, Target>
    for CheckAndSubmitMisbehaviourWithChains
where
    Target: RelayTarget,
    Relay: HasTargetChainTypes<
            Target,
            TargetChain = TargetChain,
            CounterpartyChain = CounterpartyChain,
        > + CanSendIbcMessages<MainSink, Target>
        + CanLog<LevelTrace>
        + CanLog<LevelInfo>
        + CanRaiseAsyncError<TargetChain::Error>
        + CanRaiseAsyncError<EmptyMessageResponse>,
    TargetChain: HasUpdateClientEvent
        + CanBuildMisbehaviourMessage<CounterpartyChain>
        + CanCheckMisbehaviour<CounterpartyChain>
        + HasEvidenceType
        + HasAsyncErrorType,
    CounterpartyChain: Async,
{
    async fn check_and_submit_misbehaviour(
        relay: &Relay,
        target: Target,
        target_chain: &TargetChain,
        _counterparty_chain: &CounterpartyChain,
        update_client_event: &TargetChain::UpdateClientEvent,
    ) -> Result<(), Relay::Error> {
        match target_chain.check_misbehaviour(update_client_event).await {
            Ok(Some(evidence)) => {
                let msg = target_chain
                    .build_misbehaviour_message(&evidence)
                    .await
                    .map_err(Relay::raise_error)?;

                let events = relay
                    .send_messages(target, vec![msg])
                    .await?
                    .into_iter()
                    .next()
                    .ok_or_else(|| Relay::raise_error(EmptyMessageResponse))?;

                // TODO: remove
                relay
                    .log(
                        &format!("misbehaviour submission response: {events:?}"),
                        &LevelInfo,
                    )
                    .await;
            }
            Ok(None) => {
                relay.log("no misbehaviour detected", &LevelTrace).await;
            }
            Err(e) => {
                return Err(Relay::raise_error(e));
            }
        }

        Ok(())
    }
}
