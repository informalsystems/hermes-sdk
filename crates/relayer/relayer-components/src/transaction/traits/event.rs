use alloc::vec::Vec;

use cgp_core::prelude::*;

use crate::chain::traits::types::event::HasEventType;
use crate::transaction::traits::types::HasTxResponseType;

#[derive_component(TxResponseAsEventsParserComponent, TxResponseAsEventsParser<Chain>)]
pub trait CanParseTxResponseAsEvents: HasTxResponseType + HasEventType + HasErrorType {
    fn parse_tx_response_as_events(
        response: Self::TxResponse,
    ) -> Result<Vec<Vec<Self::Event>>, Self::Error>;
}
