use core::fmt::Debug;

use hermes_relayer_components::chain::traits::types::message::HasMessageType;

/**
   Sovereign SDK currently only supports sending one message per transaction.
   However it will eventually support sending multiple messages per transaction
   in the future.

   The existing transaction interfaces and abstract implementations assume the
   ability to send multiple messages per transaction. Although there are
   workarounds such as defining new components, we would avoid that since
   such effort would be wasted once Sovereign SDK supports multi-message
   transaction.

   It is also not possible to wrap multiple concrete transactions and
   pretend that they are a single transaction. This is because doing so
   would result in different semantics as compared to a real transaction,
   which is that all messages either succeed or fail atomically.

   For the short term, we rely on dynamically raising this error, if the
   transaction context receives multiple messages. We then define a
   middleware component that splits incoming messages and submit them
   as individual transactions.
*/
pub struct MultiMessageUnsupportedError<'a, Chain>
where
    Chain: HasMessageType,
{
    pub messages: &'a [Chain::Message],
}

impl<'a, Chain> Debug for MultiMessageUnsupportedError<'a, Chain>
where
    Chain: HasMessageType,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "Sovereign rollups currently only support sending exactly one message per transaction, however {} messages are provided",
            self.messages.len()
        )
    }
}
