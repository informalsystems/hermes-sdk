use cgp_core::CanRaiseError;
use hermes_cosmos_chain_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_relayer_components::chain::traits::payload_builders::create_client::CreateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainHeight;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientPayloadOptionsType, HasCreateClientPayloadType,
};
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_sovereign_rollup_components::impls::queries::slot_hash::CanQuerySlotHash;
use hermes_sovereign_rollup_components::types::height::RollupHeight;
use ibc::core::client::types::Height;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::chain::requests::{QueryHeight, QueryHostConsensusStateRequest};
use ibc_relayer::consensus_state::AnyConsensusState;
use ibc_relayer_types::Height as RelayerHeight;
use sov_celestia_client::types::client_state::ClientState;
use sov_celestia_client::types::consensus_state::{SovTmConsensusState, TmConsensusParams};
use sov_celestia_client::types::sovereign::SovereignConsensusParams;

use crate::sovereign::traits::chain::data_chain::HasDataChain;
use crate::sovereign::traits::chain::rollup::HasRollup;
use crate::sovereign::types::payloads::client::{
    SovereignCreateClientOptions, SovereignCreateClientPayload,
};

/**
   Build a create client payload from a Sovereign rollup, to be
   used as a create message to a Cosmos counterparty chain
*/
pub struct BuildSovereignCreateClientPayload;

impl<Chain, Counterparty, Rollup, DataChain> CreateClientPayloadBuilder<Chain, Counterparty>
    for BuildSovereignCreateClientPayload
where
    Chain: HasRollup<Rollup = Rollup>
        + HasDataChain<DataChain = DataChain>
        + HasCreateClientPayloadOptionsType<
            Counterparty,
            CreateClientPayloadOptions = SovereignCreateClientOptions,
        > + HasCreateClientPayloadType<Counterparty, CreateClientPayload = SovereignCreateClientPayload>
        + CanRaiseError<Rollup::Error>,
    Rollup: CanQueryChainHeight + CanQuerySlotHash + HasHeightType<Height = RollupHeight>,
    DataChain: HasBlockingChainHandle,
{
    async fn build_create_client_payload(
        chain: &Chain,
        create_client_options: &SovereignCreateClientOptions,
    ) -> Result<SovereignCreateClientPayload, Chain::Error> {
        let rollup = chain.rollup();

        // Build client state
        let rollup_height = rollup
            .query_chain_height()
            .await
            .map_err(Chain::raise_error)?;

        let slot_hash = rollup
            .query_slot_hash(&rollup_height)
            .await
            .map_err(Chain::raise_error)?;

        let latest_height = Height::new(0, rollup_height.slot_number).unwrap();

        let mut sovereign_client_params = create_client_options.sovereign_client_params.clone();
        sovereign_client_params.latest_height = latest_height;

        let genesis_da_height = sovereign_client_params.genesis_da_height;

        let client_state = ClientState::new(
            sovereign_client_params,
            create_client_options.tendermint_params_config.clone(),
        );

        let da_latest_height = RelayerHeight::new(
            create_client_options
                .sovereign_client_params
                .genesis_da_height
                .revision_number(),
            rollup_height.slot_number + genesis_da_height.revision_height(),
        )
        .unwrap();

        let host_consensus_state_query = QueryHostConsensusStateRequest {
            height: QueryHeight::Specific(da_latest_height),
        };

        let any_consensus_state = chain
            .data_chain()
            .with_blocking_chain_handle(move |chain_handle| {
                Ok(chain_handle
                    .query_host_consensus_state(host_consensus_state_query)
                    .unwrap())
            })
            .await
            .unwrap();

        let AnyConsensusState::Tendermint(tm_consensus_state) = any_consensus_state;

        let tendermint_params = TmConsensusParams::new(
            tm_consensus_state.timestamp,
            tm_consensus_state.next_validators_hash,
        );

        let sovereign_params = SovereignConsensusParams::new(slot_hash.user_hash.to_vec().into());

        let consensus_state = SovTmConsensusState::new(sovereign_params, tendermint_params);

        // Retrieve code hash
        let code_hash = create_client_options.code_hash.clone();

        // Build Create client payload
        Ok(SovereignCreateClientPayload {
            client_state,
            consensus_state,
            code_hash,
            latest_height,
        })
    }
}
