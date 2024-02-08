use cgp_core::HasErrorType;
use hermes_relayer_components::chain::traits::components::create_client_payload_builder::CreateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientOptionsType, HasCreateClientPayloadType,
};
use ibc_core::client::types::Height;
use ibc_core::host::types::identifiers::ChainId;
use ibc_relayer::chain::client::ClientSettings;
use sov_ibc_mocks::sovereign::{dummy_sov_client_state, dummy_sov_consensus_state};

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
        // TODO: This will be replaced by data queried from the Roll-Up

        //let chain_id = chain.chain_id();
        //let latest_height = chain.query_chain_height().await?;

        let chain_id = ChainId::new("private").unwrap();
        let latest_height = Height::new(1, 10).unwrap();

        let client_state = dummy_sov_client_state(chain_id.clone(), latest_height);
        let consensus_state = dummy_sov_consensus_state();

        let code_hash =
            hex::decode("2469f43c3ca20d476442bd3d98cbd97a180776ab37332aa7b02cae5a620acfc6")
                .unwrap();

        Ok(SovereignCreateClientPayload {
            client_state: client_state.inner().clone(),
            consensus_state,
            code_hash,
            latest_height,
        })
    }
}
