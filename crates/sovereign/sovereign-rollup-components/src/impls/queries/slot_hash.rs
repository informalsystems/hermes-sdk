use cgp_core::{async_trait, CanRaiseError, HasErrorType};
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hex::FromHexError;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::ClientError;
use serde::Deserialize;

use crate::traits::json_rpc_client::HasJsonRpcClient;
use crate::types::height::RollupHeight;

pub struct SlotHash {
    pub root_hash: [u8; 32],
    pub user_hash: [u8; 32],
    pub kernel_hash: [u8; 32],
}

#[async_trait]
pub trait CanQuerySlotHash: HasHeightType + HasErrorType {
    async fn query_slot_hash(&self, height: &Self::Height) -> Result<SlotHash, Self::Error>;
}

impl<Rollup> CanQuerySlotHash for Rollup
where
    Rollup: HasHeightType<Height = RollupHeight>
        + HasJsonRpcClient
        + CanRaiseError<ClientError>
        + CanRaiseError<FromHexError>
        + CanRaiseError<&'static str>,
    Rollup::JsonRpcClient: ClientT,
{
    async fn query_slot_hash(&self, height: &RollupHeight) -> Result<SlotHash, Self::Error> {
        // FIXME: due to quirks on Sovereign SDK, the "actual" hash of a rollup at
        // slot H is queried at slot H-1.
        let slot_number = height.slot_number - 1;

        let response: SlotResponse = self
            .json_rpc_client()
            .request("ledger_getSlotByNumber", (slot_number,))
            .await
            .map_err(Rollup::raise_error)?;

        let state_root_str = response
            .state_root
            .strip_prefix("0x")
            .ok_or_else(|| Rollup::raise_error("expect response.state_root to contain hex"))?;

        let state_root = hex::decode(state_root_str).map_err(Rollup::raise_error)?;

        let user_hash = state_root[..32]
            .try_into()
            .map_err(|_| Rollup::raise_error("expect user hash to be made of 32 bytes"))?;

        let kernel_hash = state_root[32..]
            .try_into()
            .map_err(|_| Rollup::raise_error("expect kernel hash to be made of 32 bytes"))?;

        let root_hash_str = response
            .hash
            .strip_prefix("0x")
            .ok_or_else(|| Rollup::raise_error("expect response.root_hash to contain hex"))?;

        let root_hash = hex::decode(root_hash_str)
            .map_err(Rollup::raise_error)?
            .try_into()
            .map_err(|_| Rollup::raise_error("expect root hash to be made of 32 bytes"))?;

        Ok(SlotHash {
            root_hash,
            // First 32 bytes are user hash and the last 32 bytes are kernel hash.
            user_hash,
            kernel_hash,
        })
    }
}

#[derive(Deserialize)]
pub struct SlotResponse {
    pub number: u64,
    pub hash: String,
    pub state_root: String,
}
