#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;

    use crate::chain::traits::send_message::MessageSenderComponent;
    use crate::transaction::impls::allocate_nonce_and_send_messages::AllocateNonceAndSendMessages;
    use crate::transaction::impls::allocate_nonce_with_mutex::AllocateNonceWithMutex;
    use crate::transaction::impls::estimate_fees_and_send_tx::EstimateFeesAndSendTx;
    use crate::transaction::impls::poll_tx_response::PollTxResponse;
    use crate::transaction::impls::send_messages_with_default_signer::SendMessagesWithDefaultSigner;
    use crate::transaction::traits::nonce::allocate_nonce::NonceAllocatorComponent;
    use crate::transaction::traits::poll_tx_response::TxResponsePollerComponent;
    use crate::transaction::traits::send_messages_with_signer::MessagesWithSignerSenderComponent;
    use crate::transaction::traits::send_messages_with_signer_and_nonce::MessagesWithSignerAndNonceSenderComponent;

    cgp_preset! {
        DefaultTxComponents {
            MessageSenderComponent: SendMessagesWithDefaultSigner,
            MessagesWithSignerSenderComponent: AllocateNonceAndSendMessages,
            MessagesWithSignerAndNonceSenderComponent: EstimateFeesAndSendTx,
            NonceAllocatorComponent: AllocateNonceWithMutex,
            TxResponsePollerComponent: PollTxResponse,
        }
    }
}
