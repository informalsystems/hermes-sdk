#[cgp::re_export_imports]
mod preset {
    use hermes_prelude::*;

    use crate::chain::traits::MessageSenderComponent;
    use crate::transaction::impls::{
        AllocateNonceAndSendMessages, AllocateNonceWithMutex, EstimateFeesAndSendTx,
        PollTxResponse, SendMessagesWithRoundRobinSigner,
    };
    use crate::transaction::traits::{
        MessagesWithSignerAndNonceSenderComponent, MessagesWithSignerSenderComponent,
        NonceAllocatorComponent, TxResponsePollerComponent,
    };

    cgp_preset! {
        DefaultTxComponents {
            MessageSenderComponent: SendMessagesWithRoundRobinSigner,
            MessagesWithSignerSenderComponent: AllocateNonceAndSendMessages,
            MessagesWithSignerAndNonceSenderComponent: EstimateFeesAndSendTx,
            NonceAllocatorComponent: AllocateNonceWithMutex,
            TxResponsePollerComponent: PollTxResponse,
        }
    }
}
