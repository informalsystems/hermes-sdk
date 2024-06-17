use cgp_core::CanRaiseError;
use hermes_cosmos_chain_components::traits::message::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_chain_components::types::messages::client::update::CosmosUpdateClientMessage;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use ibc::clients::tendermint::types::Header;
use ibc::clients::wasm_types::client_message::{ClientMessage, WASM_CLIENT_MESSAGE_TYPE_URL};
use ibc::core::client::types::Height;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::lightclients::wasm::v1::ClientMessage as RawClientMessage;
use ibc_proto::Protobuf;
use ibc_relayer_types::clients::ics07_tendermint::header::Header as RelayerHeader;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use prost::Message;
use sov_celestia_client::types::client_message::test_util::dummy_sov_header;

use crate::sovereign::types::payloads::client::SovereignUpdateClientPayload;

pub struct BuildUpdateSovereignClientMessageOnCosmos;

impl<Chain, Counterparty> UpdateClientMessageBuilder<Chain, Counterparty>
    for BuildUpdateSovereignClientMessageOnCosmos
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId, Message = CosmosMessage>
        + CanRaiseError<&'static str>,
    Counterparty:
        HasUpdateClientPayloadType<Chain, UpdateClientPayload = SovereignUpdateClientPayload>,
{
    async fn build_update_client_message(
        _chain: &Chain,
        client_id: &ClientId,
        payload: SovereignUpdateClientPayload,
    ) -> Result<Vec<CosmosMessage>, Chain::Error> {
        // FIXME: allow multiple DA update headers to be embedded in the update client message
        let [header]: [RelayerHeader; 1] = <[RelayerHeader; 1]>::try_from(payload.datachain_header)
            .map_err(|_| Chain::raise_error("the relayer currently only supports building UpdateClient message with exactly one DA update header"))?;

        let header = Header {
            signed_header: header.signed_header,
            validator_set: header.validator_set,
            trusted_height: Height::new(
                header.trusted_height.revision_number(),
                header.trusted_height.revision_height(),
            )
            .unwrap(),
            trusted_next_validator_set: header.trusted_validator_set,
        };

        let header = dummy_sov_header(
            header,
            payload.initial_state_height.revision_height(),
            payload.final_state_height.revision_height(),
            payload.final_user_hash.into(),
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

        Ok(vec![message.to_cosmos_message()])
    }
}
