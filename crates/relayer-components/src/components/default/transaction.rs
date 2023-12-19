use cgp_core::prelude::*;

use crate::chain::traits::components::message_sender::CanSendMessages;
use crate::chain::traits::components::message_sender::MessageSenderComponent;
use crate::chain::traits::types::chain_id::HasChainId;
use crate::logger::traits::has_logger::HasLogger;
use crate::logger::traits::level::HasBaseLogLevels;
use crate::runtime::traits::mutex::HasMutex;
use crate::runtime::traits::sleep::CanSleep;
use crate::runtime::traits::time::HasTime;
use crate::transaction::components::allocate_nonce_and_send_messages::AllocateNonceAndSendMessages;
use crate::transaction::components::allocate_nonce_with_mutex::AllocateNonceWithMutex;
use crate::transaction::components::estimate_fees_and_send_tx::EstimateFeesAndSendTx;
use crate::transaction::components::poll_tx_response::PollTxResponse;
use crate::transaction::components::poll_tx_response::{CanRaiseNoTxResponseError, HasPollTimeout};
use crate::transaction::components::send_messages_with_default_signer::SendMessagesWithDefaultSigner;
use crate::transaction::traits::components::nonce_allocater::CanAllocateNonce;
use crate::transaction::traits::components::nonce_allocater::NonceAllocatorComponent;
use crate::transaction::traits::components::nonce_querier::{CanQueryNonce, NonceQuerier};
use crate::transaction::traits::components::send_messages_with_signer::MessagesWithSignerSenderComponent;
use crate::transaction::traits::components::send_messages_with_signer_and_nonce::CanSendMessagesWithSignerAndNonce;
use crate::transaction::traits::components::send_messages_with_signer_and_nonce::MessagesWithSignerAndNonceSenderComponent;
use crate::transaction::traits::components::tx_encoder::{CanEncodeTx, TxEncoder};
use crate::transaction::traits::components::tx_fee_estimater::{CanEstimateTxFee, TxFeeEstimator};
use crate::transaction::traits::components::tx_response_poller::CanPollTxResponse;
use crate::transaction::traits::components::tx_response_poller::TxResponsePollerComponent;
use crate::transaction::traits::components::tx_response_querier::{
    CanQueryTxResponse, TxResponseQuerier,
};
use crate::transaction::traits::components::tx_submitter::{CanSubmitTx, TxSubmitter};
use crate::transaction::traits::event::CanParseTxResponseAsEvents;
use crate::transaction::traits::fee::HasFeeForSimulation;
use crate::transaction::traits::logs::nonce::CanLogNonce;
use crate::transaction::traits::nonce::guard::HasNonceGuard;
use crate::transaction::traits::nonce::mutex::HasMutexForNonceAllocation;
use crate::transaction::traits::signer::HasDefaultSigner;
use crate::transaction::traits::types::HasTxTypes;

pub struct DefaultTxComponents;

delegate_components!(
    #[mark_component(IsDefaultTxComponents)]
    #[mark_delegate(DelegatesToDefaultTxComponents)]
    DefaultTxComponents {
        MessageSenderComponent: SendMessagesWithDefaultSigner,
        MessagesWithSignerSenderComponent: AllocateNonceAndSendMessages,
        MessagesWithSignerAndNonceSenderComponent: EstimateFeesAndSendTx,
        NonceAllocatorComponent: AllocateNonceWithMutex,
        TxResponsePollerComponent: PollTxResponse,
    }
);
pub trait CanUseDefaultTxComponents: UseDefaultTxComponents {}

pub trait UseDefaultTxComponents:
    CanSendMessages
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

impl<Chain, Components> UseDefaultTxComponents for Chain
where
    Chain: HasErrorType
        + HasTxTypes
        + HasDefaultSigner
        + HasNonceGuard
        + HasChainId
        + HasFeeForSimulation
        + HasMutexForNonceAllocation
        + HasPollTimeout
        + HasLogger
        + CanLogNonce
        + CanParseTxResponseAsEvents
        + CanRaiseNoTxResponseError
        + HasComponents<Components = Components>,
    Chain::Runtime: HasMutex + HasTime + CanSleep,
    Chain::Logger: HasBaseLogLevels,
    Components: DelegatesToDefaultTxComponents
        + TxEncoder<Chain>
        + TxFeeEstimator<Chain>
        + NonceQuerier<Chain>
        + TxSubmitter<Chain>
        + TxResponseQuerier<Chain>,
{
}
