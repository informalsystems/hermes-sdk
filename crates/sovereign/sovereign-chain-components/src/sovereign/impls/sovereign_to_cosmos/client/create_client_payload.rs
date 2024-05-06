use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::payload_builders::create_client::CreateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainHeight;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientOptionsType, HasCreateClientPayloadType,
};
use hermes_sovereign_rollup_components::types::height::RollupHeight;
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::ChainId as IbcChainId;
use ibc::primitives::Timestamp;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use sov_celestia_client::types::client_state::test_util::{
    dummy_sov_client_state, dummy_sov_consensus_state,
};

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
        + HasCreateClientOptionsType<Counterparty, CreateClientOptions = SovereignCreateClientOptions>
        + HasCreateClientPayloadType<Counterparty, CreateClientPayload = SovereignCreateClientPayload>
        + CanRaiseError<Rollup::Error>,
    Rollup: CanQueryChainHeight<Height = RollupHeight>,
    DataChain: HasChainId<ChainId = ChainId>,
{
    async fn build_create_client_payload(
        chain: &Chain,
        create_client_options: &SovereignCreateClientOptions,
    ) -> Result<SovereignCreateClientPayload, Chain::Error> {
        // TODO: This will be replaced by data queried from the Roll-Up

        let chain_id = IbcChainId::new(chain.data_chain().chain_id().as_str()).unwrap();

        let rollup_height = chain
            .rollup()
            .query_chain_height()
            .await
            .map_err(Chain::raise_error)?;

        let latest_height = Height::new(0, rollup_height.slot_number).unwrap();

        let client_state = dummy_sov_client_state(chain_id.clone(), latest_height);
        let tm = Timestamp::now();
        let consensus_state = dummy_sov_consensus_state(tm);

        let code_hash = create_client_options.code_hash.clone();

        Ok(SovereignCreateClientPayload {
            client_state: client_state.clone(),
            consensus_state,
            code_hash,
            latest_height,
        })
    }
}
