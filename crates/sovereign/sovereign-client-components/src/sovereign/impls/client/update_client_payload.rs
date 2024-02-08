use cgp_core::HasErrorType;
use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;

use crate::sovereign::types::client_state::SovereignClientState;
use crate::sovereign::types::height::RollupHeight;
use crate::sovereign::types::payloads::client::SovereignUpdateClientPayload;

/**
   Build an update client payload from a Sovereign rollup, to be used later
   for sending an update client message to a Cosmos counterparty chain.
*/
pub struct BuildSovereignUpdateClientPayload;

impl<Chain, Counterparty> UpdateClientPayloadBuilder<Chain, Counterparty>
    for BuildSovereignUpdateClientPayload
where
    Chain: HasHeightType<Height = RollupHeight>
        + HasUpdateClientPayloadType<Counterparty, UpdateClientPayload = SovereignUpdateClientPayload>
        + HasClientStateType<Counterparty, ClientState = SovereignClientState>
        + HasErrorType, // TODO: Add dependencies for update client payload here
{
    async fn build_update_client_payload(
        _chain: &Chain,
        _trusted_height: &RollupHeight,
        _target_height: &RollupHeight,
        _client_state: Chain::ClientState,
    ) -> Result<SovereignUpdateClientPayload, Chain::Error> {
        todo!()
    }
}
