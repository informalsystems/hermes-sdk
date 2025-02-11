/*!
   Trait definitions for [`HasIbcChainTypes`] and
  [`HasCounterpartyMessageHeight`].
*/

use cgp::core::component::UseDelegate;
use cgp::prelude::*;
pub use hermes_chain_type_components::traits::types::ibc::channel_id::*;
pub use hermes_chain_type_components::traits::types::ibc::client_id::*;
pub use hermes_chain_type_components::traits::types::ibc::connection_id::*;
pub use hermes_chain_type_components::traits::types::ibc::port_id::*;
pub use hermes_chain_type_components::traits::types::ibc::sequence::*;

use crate::traits::types::chain::HasChainTypes;
use crate::traits::types::height::HasHeightType;
use crate::traits::types::message::HasMessageType;

/**
   The abstract types for a chain context when it is used for IBC
   communication with a `Counterparty` chain context.

   In contrast to [`HasChainTypes`], this trait is parameterized by a
   `Counterparty` chain context. Additionally, the `Counterparty` chain context
   is arequired to implement
   [`HasChainTypes`].

   Because of the `Counterparty` parameter, the associated types
   in this trait are going to be different when used with different
   counterparty chain contexts. In other words, the type
   `<ChainA as HasIbcChainTypes<ChainB>>::ClientId` is different from
   `<ChainA as HasIbcChainTypes<ChainC>>::ClientId` if `ChainB` and `ChainC`
   are different.

   This is intentional, as we want to distinguish IBC identifiers associated
   with different chains and avoid accidentally mixing them up. This is
   particularly useful when implementing the relayer, because we cannot
   for example accidentally use a `ChannelId` from `SrcChain` to `DstChain`
   as a `ChannelId` from `DstChain` to `SrcChain`.

   Having the IBC chain types parameterized on the counterparty chain also
   allows a chain context to decide on different concrete types depending
   on which counterparty chain it is. For example, a Cosmos chain context
   connected with a non-Cosmos chain context may want to use different
   `ClientId` type, as compared to connecting to a Cosmos chain.

   Note that even when a chain context implements `HasIbcChainTypes`, it is
   _not_ expected to have access to resources on the counterparty chain. That
   would require access to the counterparty chain context, which is implemented
   separately from the self chain context. Instead, operations that require
   access to two chain contexts are handled by the
   [relay context](crate::relay).
*/
pub trait HasIbcChainTypes<Counterparty>:
    HasChainTypes
    + HasClientIdType<Counterparty>
    + HasConnectionIdType<Counterparty>
    + HasChannelIdType<Counterparty>
    + HasPortIdType<Counterparty>
    + HasSequenceType<Counterparty>
{
}

impl<Chain, Counterparty> HasIbcChainTypes<Counterparty> for Chain where
    Chain: HasChainTypes
        + HasClientIdType<Counterparty>
        + HasConnectionIdType<Counterparty>
        + HasChannelIdType<Counterparty>
        + HasPortIdType<Counterparty>
        + HasSequenceType<Counterparty>
{
}

#[cgp_component {
  provider: CounterpartyMessageHeightGetter,
  context: Chain,
}]
pub trait HasCounterpartyMessageHeight<Counterparty>: HasMessageType
where
    Counterparty: HasHeightType,
{
    /**
       Get the height of the counterparty chain which the UpdateClient message
       should be built. If the message is not IBC-related, this would return `None`.

       This is used by the `SendIbcMessagesWithUpdateClient`
       message sender middleware to attach `UpdateClient` messages to the
       front of the message batch before sending it to the downstream
       message sender.

       The way this works is as follows: recall that the relayer relays IBC
       packets by constructing messages from one chain and send it to
       the other chain. In this case, we have IBC events happening on
       the `Counterparty` chain, which the relayer would construct
       messages targetting this self chain. So any IBC message that the self
       chain received would correspond to events happening on the `Counterparty`
       chain. With this method, we are thus getting the
       [`Counterparty::Height`](crate::traits::types::height::HasHeightType::Height)
       and _not_ `Self::Height`.

       Note that the message height for UpdateClient is usually an increment
       of the height which the proofs are built.
    */
    fn counterparty_message_height_for_update_client(
        message: &Self::Message,
    ) -> Option<Counterparty::Height>;
}

#[cgp_provider(CounterpartyMessageHeightGetterComponent)]
impl<Chain, Counterparty, Components, Delegate> CounterpartyMessageHeightGetter<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasMessageType,
    Counterparty: HasHeightType,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
    Delegate: CounterpartyMessageHeightGetter<Chain, Counterparty>,
{
    fn counterparty_message_height_for_update_client(
        message: &Chain::Message,
    ) -> Option<Counterparty::Height> {
        Delegate::counterparty_message_height_for_update_client(message)
    }
}
