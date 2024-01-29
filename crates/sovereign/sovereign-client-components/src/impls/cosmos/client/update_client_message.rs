use cgp_core::prelude::HasErrorType;
use hermes_cosmos_client_components::traits::message::CosmosMessage;
use hermes_relayer_components::chain::traits::components::update_client_message_builder::UpdateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::types::payloads::client::SovereignUpdateClientPayload;

pub struct BuildUpdateUpdateSovereignClientMessageOnCosmos;

impl<Chain, Counterparty> UpdateClientMessageBuilder<Chain, Counterparty>
    for BuildUpdateUpdateSovereignClientMessageOnCosmos
where
    Chain:
        HasIbcChainTypes<Counterparty, ClientId = ClientId, Message = CosmosMessage> + HasErrorType,
    Counterparty:
        HasUpdateClientPayloadType<Chain, UpdateClientPayload = SovereignUpdateClientPayload>,
{
    async fn build_update_client_message(
        _chain: &Chain,
        _client_id: &ClientId,
        _payload: SovereignUpdateClientPayload,
    ) -> Result<Vec<CosmosMessage>, Chain::Error> {
        todo!()
    }
}
