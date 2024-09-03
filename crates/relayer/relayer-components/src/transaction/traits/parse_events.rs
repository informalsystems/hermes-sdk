use alloc::vec::Vec;

use cgp::prelude::*;

use crate::chain::traits::types::event::HasEventType;
use crate::transaction::traits::types::tx_response::HasTxResponseType;

#[derive_component(TxResponseAsEventsParserComponent, TxResponseAsEventsParser<Chain>)]
pub trait CanParseTxResponseAsEvents: HasTxResponseType + HasEventType + HasErrorType {
    fn parse_tx_response_as_events(
        response: Self::TxResponse,
    ) -> Result<Vec<Vec<Self::Event>>, Self::Error>;
}
