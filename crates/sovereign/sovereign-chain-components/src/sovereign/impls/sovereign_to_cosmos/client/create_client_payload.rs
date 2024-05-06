use cgp_core::HasErrorType;
use eyre::eyre;
use eyre::Error as ReportError;
use hermes_cosmos_chain_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_relayer_components::chain::traits::payload_builders::create_client::CreateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientOptionsType, HasCreateClientPayloadType,
};
use ibc::core::client::types::Height;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::chain::requests::{QueryHeight, QueryHostConsensusStateRequest};
use ibc_relayer::consensus_state::AnyConsensusState;
use sov_celestia_client::types::client_state::ClientState;
use sov_celestia_client::types::consensus_state::{SovTmConsensusState, TmConsensusParams};
use sov_celestia_client::types::sovereign::SovereignConsensusParams;

use crate::sovereign::traits::chain::data_chain::{HasDataChain, HasDataChainType};
use crate::sovereign::types::payloads::client::{
    SovereignCreateClientOptions, SovereignCreateClientPayload,
};

/**
   Build a create client payload from a Sovereign rollup, to be
   used as a create message to a Cosmos counterparty chain
*/
pub struct BuildSovereignCreateClientPayload;

impl<Chain, Counterparty, DataChain> CreateClientPayloadBuilder<Chain, Counterparty>
    for BuildSovereignCreateClientPayload
where
    Chain: HasCreateClientOptionsType<Counterparty, CreateClientOptions = SovereignCreateClientOptions>
        + HasCreateClientPayloadType<Counterparty, CreateClientPayload = SovereignCreateClientPayload>
        + HasDataChain
        + HasDataChainType<DataChain = DataChain>
        + HasErrorType<Error = ReportError>, // TODO: Add chain dependencies for create client payload here
    Chain::DataChain: HasErrorType + HasBlockingChainHandle,
{
    async fn build_create_client_payload(
        chain: &Chain,
        create_client_options: &SovereignCreateClientOptions,
    ) -> Result<SovereignCreateClientPayload, Chain::Error> {
        // TODO: This will be replaced by data queried from the Roll-Up

        //let chain_id = chain.chain_id();
        //let latest_height = chain.query_chain_height().await?;
        let data_chain = chain.data_chain();
        let da_latest_height = data_chain
            .with_blocking_chain_handle(move |chain_handle| {
                let height = chain_handle.query_latest_height().unwrap();
                Ok(height)
            })
            .await
            .map_err(|e| eyre!("Error querying latest height from DA chain: {e:?}"))?;

        let rollup_latest_height = Height::new(
            da_latest_height.revision_number(),
            da_latest_height.revision_height(),
        )
        .map_err(|e| eyre!("Error creating new Height from queried height: {e}"))?
        .sub(
            create_client_options
                .sovereign_client_params
                .genesis_da_height
                .revision_height(),
        )?;

        let mut sovereign_client_params = create_client_options.sovereign_client_params.clone();
        sovereign_client_params.latest_height = rollup_latest_height;

        let host_consensus_state_query = QueryHostConsensusStateRequest {
            height: QueryHeight::Specific(da_latest_height),
        };

        let client_state = ClientState::new(
            sovereign_client_params,
            create_client_options.tendermint_params_config.clone(),
        );

        let any_consensus_state = chain
            .data_chain()
            .with_blocking_chain_handle(move |chain_handle| {
                Ok(chain_handle
                    .query_host_consensus_state(host_consensus_state_query)
                    .unwrap())
            })
            .await
            .map_err(|e| eyre!("Error host consensus state from DA chain: {e:?}"))?;

        let AnyConsensusState::Tendermint(tm_consensus_state) = any_consensus_state else {
            panic!("Expected Tendermint consensus state");
        };

        let tendermint_params = TmConsensusParams::new(
            tm_consensus_state.timestamp,
            tm_consensus_state.next_validators_hash,
        );

        let sovereign_params = SovereignConsensusParams::new(vec![0].into());

        let consensus_state = SovTmConsensusState::new(sovereign_params, tendermint_params);

        let code_hash = create_client_options.code_hash.clone();

        Ok(SovereignCreateClientPayload {
            client_state,
            consensus_state,
            code_hash,
            latest_height: rollup_latest_height,
        })
    }
}
