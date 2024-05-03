use std::str::FromStr;

use cgp_core::HasErrorType;
use hermes_cosmos_chain_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_relayer_components::chain::traits::payload_builders::create_client::CreateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientOptionsType, HasCreateClientPayloadType,
};
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::ChainId;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::chain::requests::{QueryHeight, QueryHostConsensusStateRequest};
use ibc_relayer::consensus_state::AnyConsensusState;
use sov_celestia_client::types::client_state::test_util::dummy_sov_client_state;
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
        + HasErrorType, // TODO: Add chain dependencies for create client payload here
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
        let height = data_chain
            .with_blocking_chain_handle(move |chain_handle| {
                let height = chain_handle.query_latest_height().unwrap();
                Ok(height)
            })
            .await
            .unwrap();

        let latest_height = Height::new(height.revision_number(), height.revision_height())
            .unwrap()
            .sub(3) // dummy_sov_client_state's genesis height is 3; so rollup height is 3 less than data chain height.
            .unwrap();

        let chain_id = ChainId::from_str(&create_client_options.chain_id).unwrap();

        // returns with genesis_da_height: Height(0, 3)
        let client_state = dummy_sov_client_state(chain_id, latest_height);

        let host_consensus_state_query = QueryHostConsensusStateRequest {
            height: QueryHeight::Specific(height),
        };

        let any_consensus_state = data_chain
            .with_blocking_chain_handle(move |chain_handle| {
                Ok(chain_handle
                    .query_host_consensus_state(host_consensus_state_query)
                    .unwrap())
            })
            .await
            .unwrap();

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
            client_state: client_state.clone(),
            consensus_state,
            code_hash,
            latest_height,
        })
    }
}
