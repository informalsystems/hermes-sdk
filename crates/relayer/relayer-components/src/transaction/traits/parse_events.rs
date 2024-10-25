use alloc::vec::Vec;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;

use crate::transaction::traits::types::tx_response::HasTxResponseType;

#[derive_component(TxMessageeResponseParserComponent, TxMessageResponse<Chain>)]
pub trait CanParseTxMessageResponse:
    HasTxResponseType + HasMessageResponseType + HasErrorType
{
    fn parse_tx_message_response(
        response: Self::TxResponse,
    ) -> Result<Vec<Self::MessageResponse>, Self::Error>;
}
