use cgp_core::prelude::HasErrorType;
use hermes_cosmos_chain_components::methods::encode::encode_to_any;
use hermes_cosmos_chain_components::types::messages::client::update::{
    ProtoMsgUpdateClient, TYPE_URL,
};
use hermes_cosmos_chain_components::types::payloads::client::CosmosUpdateClientPayload;
use hermes_cosmos_chain_components::types::tendermint::TendermintHeader;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use ibc_relayer_types::signer::Signer;

use crate::types::message::SovereignMessage;
use crate::types::messages::ibc::IbcMessageWithHeight;

pub struct BuildUpdateCosmosClientMessageOnSovereign;

impl<Chain, Counterparty> UpdateClientMessageBuilder<Chain, Counterparty>
    for BuildUpdateCosmosClientMessageOnSovereign
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId, Message = SovereignMessage>
        + HasErrorType,
    Counterparty:
        HasUpdateClientPayloadType<Chain, UpdateClientPayload = CosmosUpdateClientPayload>,
{
    async fn build_update_client_message(
        _chain: &Chain,
        client_id: &ClientId,
        payload: CosmosUpdateClientPayload,
    ) -> Result<Vec<SovereignMessage>, Chain::Error> {
        let messages = payload
            .headers
            .into_iter()
            .map(|header| encode_tendermint_header(client_id, header))
            .collect();

        Ok(messages)
    }
}

pub fn encode_tendermint_header(
    client_id: &ClientId,
    header: TendermintHeader,
) -> SovereignMessage {
    let proto_message = ProtoMsgUpdateClient {
        client_id: client_id.to_string(),
        client_message: Some(header.into()),
        signer: Signer::dummy().to_string(),
    };

    let any_message = encode_to_any(TYPE_URL, &proto_message);

    IbcMessageWithHeight::new(any_message, None).into()
}
