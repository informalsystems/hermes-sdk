use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::queries::consensus_state_height::ConsensusStateHeightsQuerier;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use ibc_relayer_types::Height;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::ClientError;
use serde::{Deserialize, Serialize};

use crate::traits::json_rpc_client::HasJsonRpcClient;

pub struct QueryConsensusStateHeightsOnSovereign;

impl<Rollup, Counterparty> ConsensusStateHeightsQuerier<Rollup, Counterparty>
    for QueryConsensusStateHeightsOnSovereign
where
    Rollup: HasIbcChainTypes<Counterparty, ClientId = ClientId>
        + HasJsonRpcClient
        + CanRaiseError<ClientError>,
    // Note: The counterparty is a Cosmos chain, hence the Cosmos height type
    Counterparty: HasHeightType<Height = Height>,
    Rollup::JsonRpcClient: ClientT,
{
    async fn query_consensus_state_heights(
        rollup: &Rollup,
        client_id: &ClientId,
    ) -> Result<Vec<Height>, Rollup::Error> {
        let request = Request { client_id };

        let response: Response = rollup
            .json_rpc_client()
            .request("ibc_consensusStateHeights", (request,))
            .await
            .map_err(Rollup::raise_error)?;

        Ok(response.consensus_state_heights)
    }
}

#[derive(Serialize)]
pub struct Request<'a> {
    pub client_id: &'a ClientId,
}

#[derive(Deserialize)]
pub struct Response {
    pub consensus_state_heights: Vec<Height>,
}
