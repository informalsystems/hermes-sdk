use cgp::prelude::*;
use hermes_logging_components::traits::logger::CanLog;

use crate::chain::traits::types::message::HasMessageType;
use crate::transaction::traits::encode_tx::CanEncodeTx;
use crate::transaction::traits::estimate_tx_fee::CanEstimateTxFee;
use crate::transaction::traits::poll_tx_response::CanPollTxResponse;
use crate::transaction::traits::send_messages_with_signer_and_nonce::{
    MessagesWithSignerAndNonceSender, MessagesWithSignerAndNonceSenderComponent,
};
use crate::transaction::traits::simulation_fee::HasFeeForSimulation;
use crate::transaction::traits::submit_tx::CanSubmitTx;
use crate::transaction::traits::types::nonce::HasNonceType;
use crate::transaction::traits::types::signer::HasSignerType;

pub struct EstimateFeesAndSendTx;

pub struct LogSendMessagesWithSignerAndNonce<'a, Chain>
where
    Chain: HasSignerType + HasNonceType + HasMessageType,
{
    pub signer: &'a Chain::Signer,
    pub nonce: &'a Chain::Nonce,
    pub messages: &'a [Chain::Message],
}

#[cgp_provider(MessagesWithSignerAndNonceSenderComponent)]
impl<Chain> MessagesWithSignerAndNonceSender<Chain> for EstimateFeesAndSendTx
where
    Chain: HasFeeForSimulation
        + CanEncodeTx
        + CanEstimateTxFee
        + CanSubmitTx
        + CanPollTxResponse
        + for<'a> CanLog<LogSendMessagesWithSignerAndNonce<'a, Chain>>,
{
    async fn send_messages_with_signer_and_nonce(
        chain: &Chain,
        signer: &Chain::Signer,
        nonce: &Chain::Nonce,
        messages: &[Chain::Message],
    ) -> Result<Chain::TxResponse, Chain::Error> {
        let details = LogSendMessagesWithSignerAndNonce {
            signer,
            nonce,
            messages,
        };

        chain.log("encoding tx for simulation", &details).await;

        let fee_for_simulation = chain.fee_for_simulation();

        let simulate_tx = chain
            .encode_tx(signer, nonce, fee_for_simulation, messages)
            .await?;

        chain
            .log("estimating fee with tx for simulation", &details)
            .await;

        let tx_fee = chain.estimate_tx_fee(&simulate_tx).await?;

        chain.log("encoding tx for submission", &details).await;

        let tx = chain.encode_tx(signer, nonce, &tx_fee, messages).await?;

        chain.log("submitting tx to chain", &details).await;

        let tx_hash = chain.submit_tx(&tx).await?;

        chain.log("waiting for tx hash response", &details).await;

        let response = chain.poll_tx_response(&tx_hash).await?;

        chain.log("received tx hash response", &details).await;

        Ok(response)
    }
}
