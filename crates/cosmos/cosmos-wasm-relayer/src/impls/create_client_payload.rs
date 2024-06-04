use cgp_core::HasErrorType;
use hermes_cosmos_chain_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_relayer_components::chain::traits::payload_builders::create_client::CreateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientOptionsType, HasCreateClientPayloadType,
};
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::client_state::AnyClientState;
use ibc_relayer::consensus_state::AnyConsensusState;

use crate::types::create_client::{
    CreateWasmTendermintClientOptions, CreateWasmTendermintClientPayload,
};

pub struct BuildCreateWasmTendermintClientPayload;

impl<Chain, Counterparty> CreateClientPayloadBuilder<Chain, Counterparty>
    for BuildCreateWasmTendermintClientPayload
where
    Chain: HasCreateClientOptionsType<
            Counterparty,
            CreateClientOptions = CreateWasmTendermintClientOptions,
        > + HasCreateClientPayloadType<
            Counterparty,
            CreateClientPayload = CreateWasmTendermintClientPayload,
        > + HasBlockingChainHandle
        + HasErrorType,
{
    async fn build_create_client_payload(
        chain: &Chain,
        create_client_options: &CreateWasmTendermintClientOptions,
    ) -> Result<CreateWasmTendermintClientPayload, Chain::Error> {
        let create_client_options = create_client_options.clone();

        chain
            .with_blocking_chain_handle(move |chain_handle| {
                let height = chain_handle
                    .query_latest_height()
                    .map_err(Chain::raise_error)?;

                let any_client_state = chain_handle
                    .build_client_state(height, create_client_options.client_settings)
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

                let AnyConsensusState::Tendermint(consensus_state) = any_consensus_state;

                Ok(CreateWasmTendermintClientPayload {
                    client_state,
                    consensus_state,
                    code_hash: create_client_options.code_hash,
                })
            })
            .await
    }
}
