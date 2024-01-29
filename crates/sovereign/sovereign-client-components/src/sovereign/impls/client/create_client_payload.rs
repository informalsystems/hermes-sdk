use cgp_core::HasErrorType;
use hermes_relayer_components::chain::traits::components::create_client_payload_builder::CreateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientOptionsType, HasCreateClientPayloadType,
};
use ibc_relayer::chain::client::ClientSettings;

use crate::sovereign::types::payloads::client::SovereignCreateClientPayload;

/**
   Build a create client payload from a Sovereign rollup, to be
   used as a create message to a Cosmos counterparty chain
*/
pub struct BuildSovereignCreateClientPayload;

impl<Chain, Counterparty> CreateClientPayloadBuilder<Chain, Counterparty>
    for BuildSovereignCreateClientPayload
where
    Chain: HasCreateClientOptionsType<Counterparty, CreateClientOptions = ClientSettings>
        + HasCreateClientPayloadType<Counterparty, CreateClientPayload = SovereignCreateClientPayload>
        + HasErrorType, // TODO: Add chain dependencies for create client payload here
{
    async fn build_create_client_payload(
        _chain: &Chain,
        _create_client_options: &ClientSettings,
    ) -> Result<SovereignCreateClientPayload, Chain::Error> {
        todo!()
    }
}
