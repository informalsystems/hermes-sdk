use cgp_core::HasErrorType;
use hermes_cosmos_chain_components::traits::message::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_chain_components::types::messages::client::update::CosmosUpdateClientMessage;
use hermes_cosmos_chain_components::types::payloads::client::CosmosUpdateClientPayload;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use ibc::clients::wasm_types::client_message::{ClientMessage, WASM_CLIENT_MESSAGE_TYPE_URL};
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::lightclients::wasm::v1::ClientMessage as RawClientMessage;
use ibc_proto::Protobuf;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use prost::Message;

pub struct BuildUpdateWasmTendermintClientMessage;

impl<Chain, Counterparty> UpdateClientMessageBuilder<Chain, Counterparty>
    for BuildUpdateWasmTendermintClientMessage
where
    Chain:
        HasIbcChainTypes<Counterparty, ClientId = ClientId, Message = CosmosMessage> + HasErrorType,
    Counterparty:
        HasUpdateClientPayloadType<Chain, UpdateClientPayload = CosmosUpdateClientPayload>,
{
    async fn build_update_client_message(
        _chain: &Chain,
        client_id: &ClientId,
        payload: CosmosUpdateClientPayload,
    ) -> Result<Vec<CosmosMessage>, Chain::Error> {
        let messages = payload
            .headers
            .into_iter()
            .map(|header| {
                let any_header = Any::from(header);

                let wasm_message = ClientMessage {
                    data: any_header.encode_to_vec(),
                };

                // Convert Wasm ClientMessage to Any
                let any_wasm_message = Any {
                    type_url: WASM_CLIENT_MESSAGE_TYPE_URL.to_owned(),
                    value: Protobuf::<RawClientMessage>::encode_vec(wasm_message),
                };

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
