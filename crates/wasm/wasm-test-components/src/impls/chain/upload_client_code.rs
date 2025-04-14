use core::str::{from_utf8, FromStr, Utf8Error};
use std::num::ParseIntError;

use cgp::prelude::*;
use hermes_chain_type_components::traits::HasMessageResponseEvents;
use hermes_cosmos_chain_components::types::AbciEvent;
use hermes_relayer_components::chain::traits::CanSendSingleMessage;
use hermes_test_components::chain::traits::HasProposalIdType;

use crate::traits::chain::messages::store_code::CanBuildStoreCodeMessage;
use crate::traits::chain::upload_client_code::{
    WasmClientCodeUploader, WasmClientCodeUploaderComponent,
};

pub struct SendStoreCodeProposalMessage;

#[derive(Debug)]
pub struct ProposalIdNotFound;

#[cgp_provider(WasmClientCodeUploaderComponent)]
impl<Chain> WasmClientCodeUploader<Chain> for SendStoreCodeProposalMessage
where
    Chain: CanBuildStoreCodeMessage
        + CanSendSingleMessage
        + HasMessageResponseEvents
        + HasProposalIdType<ProposalId = u64>
        + CanRaiseAsyncError<Utf8Error>
        + CanRaiseAsyncError<ParseIntError>
        + CanRaiseAsyncError<ProposalIdNotFound>,
    Chain::Event: AsRef<AbciEvent>,
{
    async fn upload_wasm_client_code(
        chain: &Chain,
        wasm_client_bytes: &Vec<u8>,
        title: &str,
        summary: &str,
        authority: &Chain::Address,
        deposit_amount: &Chain::Amount,
    ) -> Result<u64, Chain::Error> {
        let message = chain.build_store_code_message(
            wasm_client_bytes,
            title,
            summary,
            authority,
            deposit_amount,
        );

        let response = chain.send_message(message).await?;

        for event in Chain::message_response_events(&response) {
            if event.as_ref().kind == "submit_proposal" {
                for attribute in event.as_ref().attributes.iter() {
                    if attribute.key_bytes() == "proposal_id".as_bytes() {
                        let proposal_id_str =
                            from_utf8(attribute.value_bytes()).map_err(Chain::raise_error)?;

                        let proposal_id =
                            u64::from_str(proposal_id_str).map_err(Chain::raise_error)?;

                        return Ok(proposal_id);
                    }
                }
            }
        }

        Err(Chain::raise_error(ProposalIdNotFound))
    }
}
