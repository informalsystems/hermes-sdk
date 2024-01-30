use cgp_core::prelude::*;

#[derive_component(NonceTypeComponent, ProvideNonceType<Chain>)]
pub trait HasNonceType: Async {
    type Nonce: Async;
}

#[derive_component(SignerTypeComponent, ProvideSignerType<Chain>)]
pub trait HasSignerType: Async {
    type Signer: Async;
}

pub type SignerOf<Context> = <Context as HasSignerType>::Signer;

#[derive_component(TransactionTypeComponent, ProvideTransactionType<Chain>)]
pub trait HasTransactionType: Async {
    type Transaction: Async;

    fn tx_size(tx: &Self::Transaction) -> usize;
}

#[derive_component(FeeTypeComponent, ProvideFeeType<Chain>)]
pub trait HasFeeType: Async {
    type Fee: Async;
}

#[derive_component(TransactionHashTypeComponent, ProvideTransactionHashType<Chain>)]
pub trait HasTransactionHashType: Async {
    type TxHash: Async;
}

#[derive_component(TxResponseTypeComponent, ProvideTxResponseType<Chain>)]
pub trait HasTxResponseType: Async {
    type TxResponse: Async;
}

// pub trait HasTxTypes:
//     HasMessageType
//     + HasEventType
//     + HasTransactionType
//     + HasNonceType
//     + HasFeeType
//     + HasSignerType
//     + HasTransactionHashType
//     + HasErrorType
// {
// }

// impl<Chain> HasTxTypes for Chain where
//     Chain: HasMessageType
//         + HasEventType
//         + HasTransactionType
//         + HasNonceType
//         + HasFeeType
//         + HasSignerType
//         + HasTransactionHashType
//         + HasTxResponseType
//         + HasErrorType
// {
// }
