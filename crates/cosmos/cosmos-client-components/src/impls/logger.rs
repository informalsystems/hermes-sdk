use cgp_core::Async;
use core::fmt::Debug;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::log::traits::logger::Logger;
use hermes_relayer_components::transaction::impls::estimate_fees_and_send_tx::LogSendMessagesWithSignerAndNonce;
use hermes_relayer_components::transaction::traits::types::nonce::HasNonceType;
use hermes_relayer_components::transaction::traits::types::signer::HasSignerType;
use tracing::trace;

pub struct HandleCosmosLogs;

impl<'a, Logging, Chain> Logger<Logging, LogSendMessagesWithSignerAndNonce<'a, Chain>>
    for HandleCosmosLogs
where
    Logging: Async,
    Chain: HasSignerType + HasNonceType + HasMessageType + HasChainId,
    Chain::Signer: Debug,
    Chain::Nonce: Debug,
{
    async fn log(
        _logging: &Logging,
        message: &str,
        details: &LogSendMessagesWithSignerAndNonce<'a, Chain>,
    ) {
        trace!(
            chain_id = %details.chain.chain_id(),
            nonce = ?details.nonce,
            signer = ?details.signer,
            "{message}",
        );
    }
}
