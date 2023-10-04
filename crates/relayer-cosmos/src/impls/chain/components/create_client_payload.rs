use async_trait::async_trait;
use cgp_core::CanRaiseError;
use eyre::eyre;
use ibc_relayer::chain::client::ClientSettings;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::client_state::AnyClientState;
use ibc_relayer::consensus_state::AnyConsensusState;
use ibc_relayer_components::chain::traits::components::create_client_payload_builder::CreateClientPayloadBuilder;
use ibc_relayer_components::chain::traits::types::create_client::{
    HasCreateClientOptions, HasCreateClientPayload,
};

use crate::traits::chain_handle::HasBlockingChainHandle;
use crate::types::payloads::client::CosmosCreateClientPayload;

pub struct BuildCreateClientPayloadWithChainHandle;

#[async_trait]
impl<Chain, Counterparty> CreateClientPayloadBuilder<Chain, Counterparty>
    for BuildCreateClientPayloadWithChainHandle
where
    Chain: HasCreateClientOptions<Counterparty, CreateClientPayloadOptions = ClientSettings>
        + HasCreateClientPayload<Counterparty, CreateClientPayload = CosmosCreateClientPayload>
        + HasBlockingChainHandle
        + CanRaiseError<eyre::Report>,
{
    async fn build_create_client_payload(
        chain: &Chain,
        create_client_options: &ClientSettings,
    ) -> Result<Chain::CreateClientPayload, Chain::Error> {
        let client_settings = create_client_options.clone();

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
                        return Err(
                            Chain::raise_error(eyre!("expect Tendermint consensus state")).into(),
                        );
                    }
                };

                Ok(CosmosCreateClientPayload {
                    client_state,
                    consensus_state,
                })
            })
            .await
    }
}
