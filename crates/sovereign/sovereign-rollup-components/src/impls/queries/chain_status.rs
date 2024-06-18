use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::queries::chain_status::ChainStatusQuerier;
use hermes_relayer_components::chain::traits::types::status::HasChainStatusType;
use ibc_relayer_types::timestamp::Timestamp;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::params::ArrayParams;
use jsonrpsee::core::ClientError;
use serde::Deserialize;

use crate::traits::json_rpc_client::HasJsonRpcClient;
use crate::types::height::RollupHeight;
use crate::types::status::SovereignRollupStatus;

pub struct QuerySovereignRollupStatus;

impl<Rollup> ChainStatusQuerier<Rollup> for QuerySovereignRollupStatus
where
    Rollup: HasChainStatusType<ChainStatus = SovereignRollupStatus>
        + HasJsonRpcClient
        + CanRaiseError<ClientError>,
    Rollup::JsonRpcClient: ClientT,
{
    async fn query_chain_status(rollup: &Rollup) -> Result<SovereignRollupStatus, Rollup::Error> {
        let response: SlotResponse = rollup
            .json_rpc_client()
            .request("ledger_getHead", ArrayParams::new())
            .await
            .map_err(Rollup::raise_error)?;

        let height = RollupHeight {
            slot_number: response.number,
        };

        // FIXME: use the relayer's local timestamp for now, as it is currently not possible
        // to query the remote time from the rollup.
        let timestamp = Timestamp::now();

        Ok(SovereignRollupStatus { height, timestamp })
    }
}

#[derive(Deserialize)]
pub struct SlotResponse {
    pub number: u64,
}
