use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::queries::chain_status::ChainStatusAtHeightQuerier;
use hermes_relayer_components::chain::traits::queries::chain_status::ChainStatusQuerier;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
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
        let SlotResponse {
            number,
            hash,
            state_root,
        } = rollup
            .json_rpc_client()
            .request("ledger_getHead", ArrayParams::new())
            .await
            .map_err(Rollup::raise_error)?;

        let height = RollupHeight {
            // FIXME: the actual latest slot of the rollup is +1, due to bugs on Sovereign's side
            slot_number: number + 1,
        };

        // Use the relayer's local timestamp for now, as it is currently not possible
        // to query the remote time from the rollup.
        let timestamp = Timestamp::now();

        Ok(SovereignRollupStatus {
            height,
            timestamp,
            hash: hex::decode(hash.strip_prefix("0x").unwrap()).unwrap(),
            // First 32 bytes are user hash and the last 32 bytes are kernel hash.
            state_root: hex::decode(state_root.strip_prefix("0x").unwrap()).unwrap()[..32].to_vec(),
        })
    }
}

#[derive(Deserialize)]
pub struct SlotResponse {
    pub number: u64,
    pub hash: String,
    pub state_root: String,
    // pub batch_range: (u64, u64),
    // pub batches: Vec<Batch<B, TxReceipt, E>>,
    // pub finality_status: FinalityStatus,
}

impl<Rollup> ChainStatusAtHeightQuerier<Rollup> for QuerySovereignRollupStatus
where
    Rollup: HasChainStatusType<ChainStatus = SovereignRollupStatus>
        + HasJsonRpcClient
        + HasHeightType<Height = RollupHeight>
        + CanRaiseError<ClientError>,
    Rollup::JsonRpcClient: ClientT,
{
    async fn query_chain_status_at_height(
        rollup: &Rollup,
        height: &Rollup::Height,
    ) -> Result<SovereignRollupStatus, Rollup::Error> {
        let params = {
            let mut params = ArrayParams::new();
            params.insert(height.slot_number).unwrap();
            params
        };

        let SlotResponse {
            number,
            hash,
            state_root,
        } = rollup
            .json_rpc_client()
            .request("ledger_getSlotByNumber", params)
            .await
            .map_err(Rollup::raise_error)?;

        let height = RollupHeight {
            // FIXME: the actual latest slot of the rollup is +1, due to bugs on Sovereign's side
            slot_number: number + 1,
        };

        // Use the relayer's local timestamp for now, as it is currently not possible
        // to query the remote time from the rollup.
        let timestamp = Timestamp::now();

        Ok(SovereignRollupStatus {
            height,
            timestamp,
            hash: hex::decode(hash.strip_prefix("0x").unwrap()).unwrap(),
            // First 32 bytes are user hash and the last 32 bytes are kernel hash.
            state_root: hex::decode(state_root.strip_prefix("0x").unwrap()).unwrap()[..32].to_vec(),
        })
    }
}
