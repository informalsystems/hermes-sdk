use alloc::collections::BTreeSet;
use alloc::vec::Vec;

use crate::chain::traits::types::height::CanIncrementHeight;
use crate::chain::traits::types::ibc::{HasCounterpartyMessageHeight, HasIbcChainTypes};
use crate::relay::traits::ibc_message_sender::IbcMessageSender;
use crate::relay::traits::target::ChainTarget;
use crate::relay::traits::update_client_message_builder::CanBuildTargetUpdateClientMessage;

pub struct SendIbcMessagesWithUpdateClient<Sender>(pub Sender);

impl<InSender, Relay, Sink, Target, TargetChain, CounterpartyChain>
    IbcMessageSender<Relay, Sink, Target> for SendIbcMessagesWithUpdateClient<InSender>
where
    Target: ChainTarget<Relay, TargetChain = TargetChain, CounterpartyChain = CounterpartyChain>,
    InSender: IbcMessageSender<Relay, Sink, Target>,
    TargetChain: HasIbcChainTypes<CounterpartyChain>,
    TargetChain: HasCounterpartyMessageHeight<CounterpartyChain>,
    CounterpartyChain: HasIbcChainTypes<TargetChain> + CanIncrementHeight,
    Relay: CanBuildTargetUpdateClientMessage<Target>,
{
    async fn send_messages(
        relay: &Relay,
        target: Target,
        messages: Vec<TargetChain::Message>,
    ) -> Result<Vec<Vec<TargetChain::Event>>, Relay::Error> {
        let update_heights: BTreeSet<CounterpartyChain::Height> = messages
            .iter()
            .flat_map(|message| {
                TargetChain::counterparty_message_height_for_update_client(message).into_iter()
            })
            .collect();

        let mut in_messages = Vec::new();

        for update_height in update_heights {
            let messages = relay
                .build_target_update_client_messages(Target::default(), &update_height)
                .await?;

            in_messages.extend(messages);
        }

        let update_messages_count = in_messages.len();

        in_messages.extend(messages);

        let in_events = InSender::send_messages(relay, target, in_messages).await?;

        let events = in_events.into_iter().skip(update_messages_count).collect();

        Ok(events)
    }
}
