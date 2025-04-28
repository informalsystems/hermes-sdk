#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;

    use crate::chain::traits::MessageSenderComponent;
    use crate::transaction::impls::{
        AllocateNonceAndSendMessages, AllocateNonceWithMutex, EstimateFeesAndSendTx,
        PollTxResponse, SendMessagesWithDefaultSigner,
    };
    use crate::transaction::traits::{
        MessagesWithSignerAndNonceSenderComponent, MessagesWithSignerSenderComponent,
        NonceAllocatorComponent, TxResponsePollerComponent,
    };

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
