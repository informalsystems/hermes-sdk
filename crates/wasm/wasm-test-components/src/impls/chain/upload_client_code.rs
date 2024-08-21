use core::str::{from_utf8, FromStr, Utf8Error};
use std::num::ParseIntError;

use cgp_core::error::CanRaiseError;
use hermes_cosmos_chain_components::types::event::AbciEvent;
use hermes_relayer_components::chain::traits::send_message::CanSendSingleMessage;
use hermes_test_components::chain::traits::proposal::types::proposal_id::HasProposalIdType;

use crate::traits::chain::messages::store_code::CanBuildStoreCodeMessage;
use crate::traits::chain::upload_client_code::WasmClientCodeUploader;

pub struct SendStoreCodeProposalMessage;

#[derive(Debug)]
pub struct ProposalIdNotFound;

impl<Chain> WasmClientCodeUploader<Chain> for SendStoreCodeProposalMessage
where
    Chain: CanBuildStoreCodeMessage
        + CanSendSingleMessage
        + HasProposalIdType<ProposalId = u64>
        + CanRaiseError<Utf8Error>
        + CanRaiseError<ParseIntError>
        + CanRaiseError<ProposalIdNotFound>,
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

        let events = chain.send_message(message).await?;

        for event in events {
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
