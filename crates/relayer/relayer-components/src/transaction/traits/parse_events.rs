use alloc::vec::Vec;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;

use crate::transaction::traits::types::tx_response::HasTxResponseType;

#[cgp_component {
  provider: TxMessageResponseParser,
  context: Chain,
}]
pub trait CanParseTxMessageResponse:
    HasTxResponseType + HasMessageResponseType + HasAsyncErrorType
{
    fn parse_tx_message_response(
        response: Self::TxResponse,
    ) -> Result<Vec<Self::MessageResponse>, Self::Error>;
}
