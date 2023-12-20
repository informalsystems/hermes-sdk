use async_trait::async_trait;
use cgp_core::{DelegateComponent, HasErrorType};
use cosmos_client_components::traits::rpc_client::HasRpcClient;
use hermes_relayer_components::chain::traits::components::consensus_state_querier::ConsensusStateQuerier;
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer::chain::cosmos::query::abci_query;
use ibc_relayer_cosmos::impls::chain::components::query_consensus_state::DelegateCosmosConsensusStateQuerier;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use ibc_relayer_types::core::ics24_host::path::ClientConsensusStatePath;
use ibc_relayer_types::core::ics24_host::IBC_QUERY_PATH;
use ibc_relayer_types::Height;

use crate::traits::solomachine::Solomachine;
use crate::types::chain::SolomachineChain;
use crate::types::consensus_state::{decode_client_consensus_state, SolomachineConsensusState};

pub struct QuerySolomachineConsensusStateFromCosmos;

impl<Counterparty> DelegateComponent<SolomachineChain<Counterparty>>
    for DelegateCosmosConsensusStateQuerier
where
    Counterparty: Solomachine,
{
    type Delegate = QuerySolomachineConsensusStateFromCosmos;
}

#[async_trait]
impl<Chain, Counterparty> ConsensusStateQuerier<Chain, Counterparty>
    for QuerySolomachineConsensusStateFromCosmos
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId> + HasErrorType + HasRpcClient,
    Counterparty: HasConsensusStateType<Chain, ConsensusState = SolomachineConsensusState>
        + HasHeightType<Height = Height>,
{
    async fn query_consensus_state(
        chain: &Chain,
        client_id: &ClientId,
        height: &Height,
    ) -> Result<SolomachineConsensusState, Chain::Error> {
        let data = ClientConsensusStatePath {
            client_id: client_id.clone(),
            epoch: height.revision_number(),
            height: height.revision_height(),
        };

        let response = abci_query(
            chain.rpc_client(),
            chain.rpc_address(),
            IBC_QUERY_PATH.to_string(),
            data.to_string(),
            (*height).into(),
            false,
        )
        .await
        .unwrap();

        let client_consensus_state = decode_client_consensus_state(response.value.as_slice());

        Ok(client_consensus_state)
    }
}
