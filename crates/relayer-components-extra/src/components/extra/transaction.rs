use core::marker::PhantomData;

use cgp_core::delegate_components;
use ibc_relayer_components::chain::traits::components::message_sender::MessageSenderComponent;
use ibc_relayer_components::chain::traits::types::chain_id::ChainIdTypeProviderComponent;
use ibc_relayer_components::chain::traits::types::event::EventTypeProviderComponent;
use ibc_relayer_components::chain::traits::types::message::MessageTypeProviderComponent;
use ibc_relayer_components::components::default::transaction::DefaultTxComponents;
use ibc_relayer_components::transaction::traits::components::message_as_tx_sender::MessageAsTxSenderComponent;
use ibc_relayer_components::transaction::traits::components::nonce_allocater::NonceAllocatorComponent;
use ibc_relayer_components::transaction::traits::components::nonce_querier::NonceQuerierComponent;
use ibc_relayer_components::transaction::traits::components::tx_encoder::TxEncoderComponent;
use ibc_relayer_components::transaction::traits::components::tx_fee_estimater::TxFeeEstimatorComponent;
use ibc_relayer_components::transaction::traits::components::tx_response_poller::TxResponsePollerComponent;
use ibc_relayer_components::transaction::traits::components::tx_response_querier::TxResponseQuerierComponent;
use ibc_relayer_components::transaction::traits::components::tx_submitter::TxSubmitterComponent;

pub struct ExtraTxComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_components!(
    ExtraTxComponents<BaseComponents>;
    [
        ChainIdTypeProviderComponent,
        MessageTypeProviderComponent,
        EventTypeProviderComponent,
        MessageSenderComponent,
        MessageAsTxSenderComponent,
        NonceQuerierComponent,
        NonceAllocatorComponent,
        TxEncoderComponent,
        TxFeeEstimatorComponent,
        TxSubmitterComponent,
        TxResponsePollerComponent,
        TxResponseQuerierComponent,
    ]:  DefaultTxComponents<BaseComponents>,
);
