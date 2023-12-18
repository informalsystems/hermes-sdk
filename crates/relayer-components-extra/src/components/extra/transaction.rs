use core::marker::PhantomData;

use cgp_core::{delegate_components, ErrorRaiserComponent, ErrorTypeComponent};
use ibc_relayer_components::chain::traits::components::message_sender::MessageSenderComponent;
use ibc_relayer_components::chain::traits::types::chain_id::{
    ChainIdGetterComponent, ChainIdTypeProviderComponent,
};
use ibc_relayer_components::chain::traits::types::event::EventTypeProviderComponent;
use ibc_relayer_components::chain::traits::types::message::MessageTypeProviderComponent;
use ibc_relayer_components::components::default::transaction::DefaultTxComponents;
use ibc_relayer_components::logger::traits::has_logger::{
    LoggerFieldComponent, LoggerTypeComponent,
};
use ibc_relayer_components::runtime::traits::runtime::{RuntimeComponent, RuntimeTypeComponent};
use ibc_relayer_components::transaction::traits::components::nonce_allocater::NonceAllocatorComponent;
use ibc_relayer_components::transaction::traits::components::nonce_querier::NonceQuerierComponent;
use ibc_relayer_components::transaction::traits::components::send_messages_with_signer::MessagesWithSignerSenderComponent;
use ibc_relayer_components::transaction::traits::components::send_messages_with_signer_and_nonce::MessagesWithSignerAndNonceSenderComponent;
use ibc_relayer_components::transaction::traits::components::tx_encoder::TxEncoderComponent;
use ibc_relayer_components::transaction::traits::components::tx_fee_estimater::TxFeeEstimatorComponent;
use ibc_relayer_components::transaction::traits::components::tx_response_poller::TxResponsePollerComponent;
use ibc_relayer_components::transaction::traits::components::tx_response_querier::TxResponseQuerierComponent;
use ibc_relayer_components::transaction::traits::components::tx_submitter::TxSubmitterComponent;

pub struct ExtraTxComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_components!(
    ExtraTxComponents<BaseComponents>;
    [
        ErrorTypeComponent,
        ErrorRaiserComponent,
        RuntimeTypeComponent,
        RuntimeComponent,
        LoggerTypeComponent,
        LoggerFieldComponent,
        ChainIdTypeProviderComponent,
        ChainIdGetterComponent,
        MessageTypeProviderComponent,
        EventTypeProviderComponent,
        MessageSenderComponent,
        MessagesWithSignerSenderComponent,
        MessagesWithSignerAndNonceSenderComponent,
        NonceQuerierComponent,
        NonceAllocatorComponent,
        TxEncoderComponent,
        TxFeeEstimatorComponent,
        TxSubmitterComponent,
        TxResponsePollerComponent,
        TxResponseQuerierComponent,
    ]:  DefaultTxComponents<BaseComponents>,
);
