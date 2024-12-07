/*!
   Trait definitions for [`CanSendMessages`] and [`MessageSender`].
*/
use alloc::vec;
use alloc::vec::Vec;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;

use crate::traits::types::message::HasMessageType;

/**
    This is a simplified interface offered by a chain context or a transaction
    context for atomically sending a list of messages to a chain.

    Behind the scene, the chain context may implement this by encoding the
    given messages into a transaction, and then sending it to the chain.

    Before the given messages are submitted as a transaction, the chain context
    may also perform additional operations, such as batching messages sent from
    other tasks into the same transaction.

    A chain context may also use other strategies of submitting messages. For
    example, a mock chain context can provide a mock implementation of sending
    messages, without mocking the part for submitting the messages as
    transactions.

    The implementation of `send_messages` _must_ treat the sending of messages
    as an atomic operation. i.e. the messages must all sent successfully, or all failed.

    In case if the total size of a given list of messages exceed some underlying
    transaction limit, the implementation _must not_ attempt to split the given
    messages into multiple transactions. This is because doing so could cause
    partial failure, which violates the atomicity constraint. Instead, the
    chain implementation should return an error and leave the task of splitting
    the messages to smaller batch to the caller.

    For example, the `MaxTxSizeExceededError` error is returned from the
    `CheckEncodedTxSize` component if the total message size exceeds a given
    transaction size limit. A component like `CanSpawnBatchMessageWorker`
    can then try and match against the error, and split the sent messages into
    smaller batches.
*/
#[cgp_component {
  name: MessageSenderComponent,
  provider: MessageSender,
  context: Chain,
}]
#[async_trait]
pub trait CanSendMessages: HasMessageType + HasMessageResponseType + HasErrorType {
    /**
        Given a list of [messages](HasMessageType::Message), submit the messages
        atomically to the chain.

        On success, the method returns a list of
        [message responses](HasMessageResponseType::MessageResponse).
        The length of the list must match the length of the input message list.
        Each message response may contain to the events emitted from processing
        the message at the same position in the input message list.

        On failure, the method returns an
        [error](cgp_core::error::HasErrorType::Error).
        Note that since the message sending must be atomic, the sending of
        messages must either all succeed or all failed. i.e. partial failure
        is forbidden.
    */
    async fn send_messages(
        &self,
        messages: Vec<Self::Message>,
    ) -> Result<Vec<Self::MessageResponse>, Self::Error>;
}

#[async_trait]
pub trait CanSendSingleMessage: HasMessageType + HasMessageResponseType + HasErrorType {
    async fn send_message(
        &self,
        message: Self::Message,
    ) -> Result<Self::MessageResponse, Self::Error>;
}

#[derive(Debug)]
pub struct EmptyMessageResponse;

impl<Chain> CanSendSingleMessage for Chain
where
    Chain: CanSendMessages + CanRaiseError<EmptyMessageResponse>,
{
    async fn send_message(
        &self,
        message: Chain::Message,
    ) -> Result<Chain::MessageResponse, Chain::Error> {
        let responses = self.send_messages(vec![message]).await?;

        let response = responses
            .into_iter()
            .next()
            .ok_or_else(|| Chain::raise_error(EmptyMessageResponse))?;

        Ok(response)
    }
}
