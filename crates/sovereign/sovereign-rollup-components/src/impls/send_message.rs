use core::marker::PhantomData;

use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::send_message::MessageSender;
use hermes_relayer_components::chain::traits::types::event::HasEventType;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;

/**
   As a workaround of Sovereign SDK allowing only one message per transaction,
   we are sending one message at a time to the rollup, and only send the next
   message if the previous message succeeded. Although it is very inefficient,
   this helps us continue development without having to handle multi-transaction
   failure.

   Although Sovereign SDK allows multiple transactions to be submitted at the
   same time, the semantics is subtly different from sending multiple messages
   per transaction. In particular, it is very challenging to recover from
   faiures, in case only some of the transactions succeed. There are all kinds
   of corner cases and race conditions that we would have to deal with, to ensure
   that subsequent transactions do not conflict with the supposingly failed
   transaction, which could in fact succeeded later without the relayer knowing.

   Because of this, we are deferring in handling such complexity, and opt for the
   simpler semantics of sending one message at a time for now.
*/
pub struct SendMessagesInSequence<InSender>(pub PhantomData<InSender>);

impl<Chain, InSender> MessageSender<Chain> for SendMessagesInSequence<InSender>
where
    Chain: HasMessageType + HasEventType + CanRaiseError<&'static str>,
    InSender: MessageSender<Chain>,
{
    async fn send_messages(
        chain: &Chain,
        messages: Vec<Chain::Message>,
    ) -> Result<Vec<Vec<Chain::Event>>, Chain::Error> {
        let mut events = Vec::new();

        for message in messages {
            let in_events = InSender::send_messages(chain, vec![message]).await?;

            let [in_events] = <[Vec<Chain::Event>; 1]>::try_from(in_events).map_err(|_| {
                Chain::raise_error(
                    "expected inner message sender to return exactly one list of events",
                )
            })?;

            events.push(in_events);
        }

        Ok(events)
    }
}
