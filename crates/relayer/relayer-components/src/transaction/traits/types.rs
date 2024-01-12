use cgp_core::{Async, HasErrorType};

use crate::chain::traits::types::event::HasEventType;
use crate::chain::traits::types::message::HasMessageType;

pub trait HasNonceType: Async {
    type Nonce: Async;
}

pub trait HasSignerType: Async {
    type Signer: Async;
}

pub type SignerOf<Context> = <Context as HasSignerType>::Signer;

pub trait HasTxTypes:
    HasMessageType + HasEventType + HasNonceType + HasSignerType + HasErrorType
{
    type Transaction: Async;

    type Fee: Async;

    type TxHash: Async;

    type TxResponse: Async;

    fn tx_size(tx: &Self::Transaction) -> usize;
}
