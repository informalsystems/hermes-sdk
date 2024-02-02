use cgp_core::CanRaiseError;
use cgp_core::HasErrorType;
use eyre::eyre;
use hermes_cosmos_client_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_cosmos_client_components::types::payloads::client::CosmosCreateClientPayload;
use hermes_relayer_components::chain::traits::components::create_client_payload_builder::CreateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientOptionsType, HasCreateClientPayloadType,
};
use ibc_relayer::chain::client::ClientSettings;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::client_state::AnyClientState;
use ibc_relayer::consensus_state::AnyConsensusState;
use ibc_relayer_types::core::ics02_client::height::Height;

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
        + CanRaiseError<eyre::Report>
        + HasBlockingChainHandle
        + HasErrorType, // TODO: Add chain dependencies for create client payload here
{
    async fn build_create_client_payload(
        chain: &Chain,
        create_client_options: &ClientSettings,
    ) -> Result<SovereignCreateClientPayload, Chain::Error> {
        let client_settings = create_client_options.clone();

        // TODO: This will be replaced by data queried from the Roll-Up
        chain
            .with_blocking_chain_handle(move |chain_handle| {
                let height = chain_handle
                    .query_latest_height()
                    .map_err(Chain::raise_error)?;

                let any_client_state = chain_handle
                    .build_client_state(height, client_settings)
                    .map_err(Chain::raise_error)?;

                let client_state = match &any_client_state {
                    AnyClientState::Tendermint(client_state) => client_state.clone(),
                };

                let any_consensus_state = chain_handle
                    .build_consensus_state(
                        any_client_state.latest_height(),
                        height,
                        any_client_state,
                    )
                    .map_err(Chain::raise_error)?;

                let consensus_state = match any_consensus_state {
                    AnyConsensusState::Tendermint(consensus_state) => consensus_state,
                    _ => {
                        return Err(Chain::raise_error(eyre!(
                            "expect Tendermint consensus state"
                        )));
                    }
                };

                let celestia_payload = CosmosCreateClientPayload {
                    client_state,
                    consensus_state,
                };

                let code_hash = "wasm_code_hash".as_bytes().to_vec();
                let latest_height = Height::new(1, 20).unwrap();

                Ok(SovereignCreateClientPayload {
                    celestia_payload,
                    code_hash,
                    latest_height,
                })
            })
            .await
    }
}
