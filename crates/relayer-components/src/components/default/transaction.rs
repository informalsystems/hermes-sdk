use core::marker::PhantomData;

use cgp_core::prelude::*;

use crate::chain::traits::components::message_sender::MessageSenderComponent;
use crate::chain::traits::types::chain_id::{ChainIdGetterComponent, ChainIdTypeProviderComponent};
use crate::chain::traits::types::event::EventTypeProviderComponent;
use crate::chain::traits::types::message::MessageTypeProviderComponent;
use crate::logger::traits::has_logger::{LoggerFieldComponent, LoggerTypeComponent};
use crate::transaction::components::allocate_nonce_with_mutex::AllocateNonceWithMutex;
use crate::transaction::components::estimate_fees_and_send_tx::EstimateFeesAndSendTx;
use crate::transaction::components::poll_tx_response::PollTxResponse;
use crate::transaction::components::send_messages_as_tx::SendMessagesAsTx;
use crate::transaction::traits::components::nonce_allocater::NonceAllocatorComponent;
use crate::transaction::traits::components::nonce_querier::NonceQuerierComponent;
use crate::transaction::traits::components::send_message_with_signer_and_nonce::MessagesWithSignerAndNonceSenderComponent;
use crate::transaction::traits::components::tx_encoder::TxEncoderComponent;
use crate::transaction::traits::components::tx_fee_estimater::TxFeeEstimatorComponent;
use crate::transaction::traits::components::tx_response_poller::TxResponsePollerComponent;
use crate::transaction::traits::components::tx_response_querier::TxResponseQuerierComponent;
use crate::transaction::traits::components::tx_submitter::TxSubmitterComponent;

pub struct DefaultTxComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_components!(
    DefaultTxComponents<BaseComponents>;
    MessageSenderComponent: SendMessagesAsTx,
    MessagesWithSignerAndNonceSenderComponent: EstimateFeesAndSendTx,
    NonceAllocatorComponent: AllocateNonceWithMutex,
    TxResponsePollerComponent: PollTxResponse,
    [
        LoggerTypeComponent,
        LoggerFieldComponent,
        ChainIdTypeProviderComponent,
        ChainIdGetterComponent,
        MessageTypeProviderComponent,
        EventTypeProviderComponent,
        NonceQuerierComponent,
        TxEncoderComponent,
        TxFeeEstimatorComponent,
        TxSubmitterComponent,
        TxResponseQuerierComponent,
    ]:
        BaseComponents,
);
