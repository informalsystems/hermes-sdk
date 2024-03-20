use crate::logger::traits::level::HasBaseLogLevels;
use crate::transaction::traits::encode_tx::CanEncodeTx;
use crate::transaction::traits::estimate_tx_fee::CanEstimateTxFee;
use crate::transaction::traits::logs::logger::CanLogTx;
use crate::transaction::traits::logs::nonce::CanLogNonce;
use crate::transaction::traits::poll_tx_response::CanPollTxResponse;
use crate::transaction::traits::send_messages_with_signer_and_nonce::MessagesWithSignerAndNonceSender;
use crate::transaction::traits::simulation_fee::HasFeeForSimulation;
use crate::transaction::traits::submit_tx::CanSubmitTx;

pub struct EstimateFeesAndSendTx;

impl<Context> MessagesWithSignerAndNonceSender<Context> for EstimateFeesAndSendTx
where
    Context: HasFeeForSimulation
        + CanEncodeTx
        + CanEstimateTxFee
        + CanSubmitTx
        + CanPollTxResponse
        + CanLogTx
        + CanLogNonce,
{
    async fn send_messages_with_signer_and_nonce(
        context: &Context,
        signer: &Context::Signer,
        nonce: &Context::Nonce,
        messages: &[Context::Message],
    ) -> Result<Context::TxResponse, Context::Error> {
        context.log_tx(
            Context::Logger::LEVEL_TRACE,
            "encoding tx for simulation",
            |log| {
                log.field("nonce", Context::log_nonce(nonce));
            },
        );

        let fee_for_simulation = context.fee_for_simulation();

        let simulate_tx = context
            .encode_tx(signer, nonce, fee_for_simulation, messages)
            .await?;

        context.log_tx(
            Context::Logger::LEVEL_TRACE,
            "estimating fee with tx for simulation",
            |log| {
                log.field("none", Context::log_nonce(nonce));
            },
        );

        let tx_fee = context.estimate_tx_fee(&simulate_tx).await?;

        context.log_tx(
            Context::Logger::LEVEL_TRACE,
            "encoding tx for submission",
            |log| {
                log.field("none", Context::log_nonce(nonce));
            },
        );

        let tx = context.encode_tx(signer, nonce, &tx_fee, messages).await?;

        context.log_tx(
            Context::Logger::LEVEL_TRACE,
            "submitting tx to chain",
            |log| {
                log.field("none", Context::log_nonce(nonce));
            },
        );

        let tx_hash = context.submit_tx(&tx).await?;

        context.log_tx(
            Context::Logger::LEVEL_TRACE,
            "waiting for tx hash response",
            |log| {
                log.field("none", Context::log_nonce(nonce));
            },
        );

        let response = context.poll_tx_response(&tx_hash).await?;

        context.log_tx(
            Context::Logger::LEVEL_TRACE,
            "received tx hash response",
            |log| {
                log.field("none", Context::log_nonce(nonce));
            },
        );

        Ok(response)
    }
}
