use alloc::vec::Vec;

use crate::transaction::traits::types::HasTxTypes;

pub trait CanParseTxResponseAsEvents: HasTxTypes {
    fn parse_tx_response_as_events(
        response: Self::TxResponse,
    ) -> Result<Vec<Vec<Self::Event>>, Self::Error>;
}
