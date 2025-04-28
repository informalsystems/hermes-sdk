use alloc::vec::Vec;

use hermes_chain_type_components::traits::HasMessageResponseType;
use hermes_prelude::*;

use crate::transaction::traits::HasTxResponseType;

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
