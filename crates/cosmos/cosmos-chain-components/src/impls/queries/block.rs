use cgp_core::error::CanRaiseError;
use hermes_relayer_components::chain::traits::queries::block::BlockQuerier;
use hermes_relayer_components::chain::traits::types::block::HasBlockType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use tendermint::block::{Block, Height as BlockHeight, Id as BlockId};
use tendermint_rpc::{Client, Error as RpcError};

use crate::traits::rpc_client::HasRpcClient;

pub struct QueryCometBlock;

impl<Chain> BlockQuerier<Chain> for QueryCometBlock
where
    Chain: HasBlockType<Block = (BlockId, Block)>
        + HasHeightType
        + CanRaiseError<RpcError>
        + HasRpcClient,
    Chain::Height: Clone + Into<BlockHeight>,
{
    async fn query_block(
        chain: &Chain,
        height: &Chain::Height,
    ) -> Result<(BlockId, Block), Chain::Error> {
        let response = chain
            .rpc_client()
            .block(height.clone())
            .await
            .map_err(Chain::raise_error)?;

        Ok((response.block_id, response.block))
    }
}
