use cgp_core::prelude::*;
use hermes_cosmos_client_components::components::ibc_client::CosmosIbcClientComponents;
use hermes_relayer_components::chain::traits::queries::consensus_state::ConsensusStateQuerier;
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use ibc_relayer_types::core::ics02_client::height::Height;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::contexts::chain::CosmosChain;
use crate::types::error::Error;

pub struct DelegateCosmosConsensusStateQuerier;

#[async_trait]
impl<Counterparty, Delegate> ConsensusStateQuerier<CosmosChain, Counterparty>
    for DelegateCosmosConsensusStateQuerier
where
    Counterparty: HasConsensusStateType<CosmosChain> + HasHeightType,
    Delegate: ConsensusStateQuerier<CosmosChain, Counterparty>,
    Self: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn query_consensus_state(
        chain: &CosmosChain,
        client_id: &ClientId,
        consensus_height: &Counterparty::Height,
        query_height: &Height,
    ) -> Result<Counterparty::ConsensusState, Error> {
        Delegate::query_consensus_state(chain, client_id, consensus_height, query_height).await
    }
}

impl DelegateComponent<CosmosChain> for DelegateCosmosConsensusStateQuerier {
    type Delegate = CosmosIbcClientComponents;
}
