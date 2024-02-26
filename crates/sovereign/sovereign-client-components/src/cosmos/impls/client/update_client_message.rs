use cgp_core::prelude::HasErrorType;
use hermes_cosmos_client_components::traits::message::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_client_components::types::messages::client::update::CosmosUpdateClientMessage;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use ibc::clients::tendermint::types::TENDERMINT_HEADER_TYPE_URL;
use ibc_proto::google::protobuf::Any;
use ibc_proto_new::google::protobuf::Any as NewAny;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use sov_celestia_client::types::client_message::test_util::dummy_sov_header;

use crate::sovereign::traits::chain::data_chain::HasDataChain;
use crate::sovereign::types::payloads::client::SovereignUpdateClientPayload;

pub struct BuildUpdateSovereignClientMessageOnCosmos;

impl<Chain, Counterparty> UpdateClientMessageBuilder<Chain, Counterparty>
    for BuildUpdateSovereignClientMessageOnCosmos
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId, Message = CosmosMessage>
        + HasErrorType
        + HasDataChain,
    Counterparty:
        HasUpdateClientPayloadType<Chain, UpdateClientPayload = SovereignUpdateClientPayload>,
{
    async fn build_update_client_message(
        _chain: &Chain,
        client_id: &ClientId,
        payload: SovereignUpdateClientPayload,
    ) -> Result<Vec<CosmosMessage>, Chain::Error> {
        let messages = payload
            .datachain_header
            .into_iter()
            .map(|da_header| {
                let header = dummy_sov_header(
                    da_header.clone(),
                    payload.initial_state_height,
                    payload.final_state_height,
                );
                let new_any_header = NewAny::from(header);
                let any_header = Any {
                    type_url: TENDERMINT_HEADER_TYPE_URL.to_owned(),
                    value: new_any_header.value,
                };
                let message = CosmosUpdateClientMessage {
                    client_id: client_id.clone(),
                    header: any_header,
                };

                message.to_cosmos_message()
            })
            .collect();

        Ok(messages)
    }
}
