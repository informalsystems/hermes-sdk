/*!
   Trait definition for [`HasChainTypes`].
*/

use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;

use crate::traits::types::chain_id::HasChainIdType;
use crate::traits::types::event::HasEventType;
use crate::traits::types::height::HasHeightType;
use crate::traits::types::message::HasMessageType;
use crate::traits::types::timestamp::{HasTimeType, HasTimeoutType};

/**
   This covers the minimal abstract types that are used inside a chain context.

   A chain context have the following abstract types:

   -    [`Height`](HasHeightType::Height) - the height of a chain, which should
        like natural numbers.

   -    [`Timestamp`](HasTimestampType::Timestamp) - the timestamp of a chain, which should
        increment monotonically.

   -    [`Message`](HasMessageType::Message) - the messages being submitted
        to a chain.

   -    [`Event`](HasEventType::Event) - the events that are emitted after
        a transaction is committed to a chain.

    This trait only covers chain types that involve a single chain. For IBC
    chain types that involve _two_ chains, the abstract types are defined
    in [`HasIbcChainTypes`](super::ibc::HasIbcChainTypes).

    Notice that a chain context do not contain a `Transaction` abstract
    type. This is because we separate the concerns of normal chain operations
    from the special concerns of assembling chain messages into transactions
    and broadcasting it to the blockchain. See the
    [`transaction`](crate::transaction) module for more information
    about the transaction context.
*/
pub trait HasChainTypes:
    HasHeightType
    + HasMessageType
    + HasMessageResponseType
    + HasEventType
    + HasChainIdType
    + HasTimeType
    + HasTimeoutType
{
}

impl<Chain> HasChainTypes for Chain where
    Chain: HasHeightType
        + HasMessageType
        + HasMessageResponseType
        + HasEventType
        + HasChainIdType
        + HasTimeType
        + HasTimeoutType
{
}
