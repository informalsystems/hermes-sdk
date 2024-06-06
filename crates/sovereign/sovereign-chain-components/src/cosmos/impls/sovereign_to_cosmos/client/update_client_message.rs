use cgp_core::prelude::HasErrorType;
use hermes_cosmos_chain_components::traits::message::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_chain_components::types::messages::client::update::CosmosUpdateClientMessage;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use ibc::clients::wasm_types::client_message::{ClientMessage, WASM_CLIENT_MESSAGE_TYPE_URL};
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::lightclients::wasm::v1::ClientMessage as RawClientMessage;
use ibc_proto::Protobuf;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use prost::Message;
use sov_celestia_client::types::client_message::test_util::dummy_sov_header;

use crate::sovereign::types::payloads::client::SovereignUpdateClientPayload;

pub struct BuildUpdateSovereignClientMessageOnCosmos;

impl<Chain, Counterparty> UpdateClientMessageBuilder<Chain, Counterparty>
    for BuildUpdateSovereignClientMessageOnCosmos
where
    Chain:
        HasIbcChainTypes<Counterparty, ClientId = ClientId, Message = CosmosMessage> + HasErrorType,
    Counterparty:
        HasUpdateClientPayloadType<Chain, UpdateClientPayload = SovereignUpdateClientPayload>,
{
    async fn build_update_client_message(
        _chain: &Chain,
        client_id: &ClientId,
        payload: SovereignUpdateClientPayload,
    ) -> Result<Vec<CosmosMessage>, Chain::Error> {
        // TODO: Multiple DA headers should map to a single UpdateClient message
        // Maybe limit to a single DA header for now

        let messages = payload
            .datachain_header
            .into_iter()
            .map(|da_header| {
                let header = dummy_sov_header(
                    da_header.clone(),
                    payload.initial_state_height.revision_height(),
                    payload.final_state_height.revision_height(),
                    // FIXME: Use actual commitment root from payload
                    vec![0; 32].try_into().unwrap(),
                );
                // Convert Sovereign header to Any
                let any_header = Any::from(header);

                // Create Wasm ClientMessage containing the Sovereign
                // header converted to Any
                let wasm_message = ClientMessage {
                    data: any_header.encode_to_vec(),
                };

                // Convert Wasm ClientMessage to Any
                let any_wasm_message = Any {
                    type_url: WASM_CLIENT_MESSAGE_TYPE_URL.to_owned(),
                    value: Protobuf::<RawClientMessage>::encode_vec(wasm_message),
                };

                // Send the Wasm message converted to Any
                let message = CosmosUpdateClientMessage {
                    client_id: client_id.clone(),
                    header: any_wasm_message,
                };

                message.to_cosmos_message()
            })
            .collect();

        Ok(messages)
    }
}
