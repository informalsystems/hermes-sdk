use cgp::prelude::*;
use hermes_chain_type_components::traits::fields::message_response_events::HasMessageResponseEvents;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_runtime_components::traits::sleep::CanSleep;
use hermes_runtime_components::traits::time::HasTime;

use crate::chain::traits::send_message::{CanSendMessages, MessageSenderComponent};
use crate::chain::traits::types::chain_id::HasChainId;
use crate::chain::traits::types::event::HasEventType;
use crate::chain::traits::types::message::HasMessageType;
use crate::error::traits::retry::HasRetryableError;
use crate::transaction::impls::allocate_nonce_and_send_messages::AllocateNonceAndSendMessages;
use crate::transaction::impls::allocate_nonce_with_mutex::AllocateNonceWithMutex;
use crate::transaction::impls::estimate_fees_and_send_tx::{
    EstimateFeesAndSendTx, LogSendMessagesWithSignerAndNonce,
};
use crate::transaction::impls::poll_tx_response::{
    HasPollTimeout, LogRetryQueryTxResponse, PollTxResponse, TxNoResponseError,
};
use crate::transaction::impls::send_messages_with_default_signer::SendMessagesWithDefaultSigner;
use crate::transaction::traits::default_signer::HasDefaultSigner;
use crate::transaction::traits::encode_tx::{CanEncodeTx, TxEncoder};
use crate::transaction::traits::estimate_tx_fee::{CanEstimateTxFee, TxFeeEstimator};
use crate::transaction::traits::nonce::allocate_nonce::{
    CanAllocateNonce, NonceAllocatorComponent,
};
use crate::transaction::traits::nonce::nonce_guard::HasNonceGuard;
use crate::transaction::traits::nonce::nonce_mutex::HasMutexForNonceAllocation;
use crate::transaction::traits::nonce::query_nonce::{CanQueryNonce, NonceQuerier};
use crate::transaction::traits::parse_events::CanParseTxMessageResponse;
use crate::transaction::traits::poll_tx_response::{CanPollTxResponse, TxResponsePollerComponent};
use crate::transaction::traits::query_tx_response::{CanQueryTxResponse, TxResponseQuerier};
use crate::transaction::traits::send_messages_with_signer::{
    CanSendMessagesWithSigner, MessagesWithSignerSenderComponent,
};
use crate::transaction::traits::send_messages_with_signer_and_nonce::{
    CanSendMessagesWithSignerAndNonce, MessagesWithSignerAndNonceSenderComponent,
};
use crate::transaction::traits::simulation_fee::HasFeeForSimulation;
use crate::transaction::traits::submit_tx::{CanSubmitTx, TxSubmitter};
use crate::transaction::traits::types::fee::HasFeeType;
use crate::transaction::traits::types::nonce::HasNonceType;
use crate::transaction::traits::types::signer::HasSignerType;
use crate::transaction::traits::types::transaction::HasTransactionType;
use crate::transaction::traits::types::tx_hash::HasTransactionHashType;
use crate::transaction::traits::types::tx_response::HasTxResponseType;

cgp_preset! {
    DefaultTxComponents {
        MessageSenderComponent: SendMessagesWithDefaultSigner,
        MessagesWithSignerSenderComponent: AllocateNonceAndSendMessages,
        MessagesWithSignerAndNonceSenderComponent: EstimateFeesAndSendTx,
        NonceAllocatorComponent: AllocateNonceWithMutex,
        TxResponsePollerComponent: PollTxResponse,
    }
}

pub trait CanUseDefaultTxComponents: UseDefaultTxComponents {}

pub trait UseDefaultTxComponents:
    CanSendMessages
    + CanSendMessagesWithSigner
    + CanSendMessagesWithSignerAndNonce
    + CanAllocateNonce
    + CanPollTxResponse
    + CanQueryNonce
    + CanEncodeTx
    + CanEstimateTxFee
    + CanSubmitTx
    + CanQueryTxResponse
{
}

impl<Chain, Components, Logger> UseDefaultTxComponents for Chain
where
    Chain: HasAsyncErrorType
        + HasMessageType
        + HasEventType
        + HasTransactionType
        + HasNonceType
        + HasFeeType
        + HasSignerType
        + HasTransactionHashType
        + HasTxResponseType
        + HasMessageResponseEvents
        + HasDefaultSigner
        + HasNonceGuard
        + HasChainId
        + HasFeeForSimulation
        + HasMutexForNonceAllocation
        + HasPollTimeout
        + HasRetryableError
        + HasLogger<Logger = Logger>
        + CanParseTxMessageResponse
        + for<'a> CanRaiseAsyncError<TxNoResponseError<'a, Chain>>
        + HasComponents<Components = Components>,
    Chain::Runtime: HasTime + CanSleep,
    Logger: for<'a> CanLog<LogSendMessagesWithSignerAndNonce<'a, Chain>>
        + for<'a> CanLog<TxNoResponseError<'a, Chain>>
        + for<'a> CanLog<LogRetryQueryTxResponse<'a, Chain>>,
    Components: DelegatesToDefaultTxComponents
        + TxEncoder<Chain>
        + TxFeeEstimator<Chain>
        + NonceQuerier<Chain>
        + TxSubmitter<Chain>
        + TxResponseQuerier<Chain>,
{
}
