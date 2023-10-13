use core::marker::PhantomData;

use cgp_core::prelude::*;

use crate::chain::traits::components::message_sender::MessageSenderComponent;
use crate::chain::traits::types::event::EventTypeProviderComponent;
use crate::chain::traits::types::message::MessageTypeProviderComponent;
use crate::transaction::components::message_as_tx::EstimateFeesAndSendTx;
use crate::transaction::components::message_sender::send_as_tx::SendMessagesAsTx;
use crate::transaction::components::nonce::mutex::AllocateNonceWithMutex;
use crate::transaction::components::poll::PollTxResponse;
use crate::transaction::traits::components::message_as_tx_sender::MessageAsTxSenderComponent;
use crate::transaction::traits::components::nonce_allocater::NonceAllocatorComponent;
use crate::transaction::traits::components::nonce_querier::NonceQuerierComponent;
use crate::transaction::traits::components::tx_encoder::TxEncoderComponent;
use crate::transaction::traits::components::tx_fee_estimater::TxFeeEstimatorComponent;
use crate::transaction::traits::components::tx_response_poller::TxResponsePollerComponent;
use crate::transaction::traits::components::tx_response_querier::TxResponseQuerierComponent;
use crate::transaction::traits::components::tx_submitter::TxSubmitterComponent;

pub struct DefaultTxComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_components!(
    DefaultTxComponents<BaseComponents>;
    MessageSenderComponent: SendMessagesAsTx,
    MessageAsTxSenderComponent: EstimateFeesAndSendTx,
    NonceAllocatorComponent: AllocateNonceWithMutex,
    TxResponsePollerComponent: PollTxResponse,
    [
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
