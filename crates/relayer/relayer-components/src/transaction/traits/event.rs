use alloc::vec::Vec;
use cgp_core::HasErrorType;

use crate::chain::traits::types::event::HasEventType;
use crate::transaction::traits::types::HasTxResponseType;

pub trait CanParseTxResponseAsEvents: HasTxResponseType + HasEventType + HasErrorType {
    fn parse_tx_response_as_events(
        response: Self::TxResponse,
    ) -> Result<Vec<Vec<Self::Event>>, Self::Error>;
}
