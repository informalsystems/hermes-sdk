use cgp_core::prelude::HasErrorType;
use hermes_cosmos_chain_components::types::payloads::client::CosmosUpdateClientPayload;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::types::message::SovereignMessage;

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
        _client_id: &ClientId,
        _payload: CosmosUpdateClientPayload,
    ) -> Result<Vec<SovereignMessage>, Chain::Error> {
        todo!()
    }
}
