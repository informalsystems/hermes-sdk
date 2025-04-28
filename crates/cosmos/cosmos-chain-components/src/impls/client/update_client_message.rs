use cgp::prelude::*;
use hermes_core::relayer_components::chain::traits::{
    HasIbcChainTypes, HasUpdateClientPayloadType, UpdateClientMessageBuilder,
    UpdateClientMessageBuilderComponent,
};
use ibc::core::host::types::identifiers::ClientId;
use ibc_proto::google::protobuf::Any as IbcProtoAny;
use prost_types::Any;

use crate::traits::{CosmosMessage, ToCosmosMessage};
use crate::types::{CosmosUpdateClientMessage, CosmosUpdateClientPayload};

pub struct BuildCosmosUpdateClientMessage;

#[cgp_provider(UpdateClientMessageBuilderComponent)]
impl<Chain, Counterparty> UpdateClientMessageBuilder<Chain, Counterparty>
    for BuildCosmosUpdateClientMessage
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId, Message = CosmosMessage>
        + HasAsyncErrorType,
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
                let header_any: IbcProtoAny = header.into();

                let message = CosmosUpdateClientMessage {
                    client_id: client_id.clone(),
                    header: Any {
                        type_url: header_any.type_url,
                        value: header_any.value,
                    },
                };

                message.to_cosmos_message()
            })
            .collect();

        Ok(messages)
    }
}
