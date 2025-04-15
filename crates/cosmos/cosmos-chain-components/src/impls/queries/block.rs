use cgp::prelude::*;
use hermes_core::relayer_components::chain::traits::{
    BlockQuerier, BlockQuerierComponent, HasBlockType, HasHeightType,
};
use ibc::core::client::types::Height;
use tendermint::block::{Block, Height as BlockHeight, Id as BlockId};
use tendermint::Error as TendermintError;
use tendermint_rpc::{Client, Error as RpcError};

use crate::traits::HasRpcClient;

pub struct QueryCometBlock;

#[cgp_provider(BlockQuerierComponent)]
impl<Chain> BlockQuerier<Chain> for QueryCometBlock
where
    Chain: HasBlockType<Block = (BlockId, Block)>
        + HasHeightType<Height = Height>
        + CanRaiseAsyncError<RpcError>
        + CanRaiseAsyncError<TendermintError>
        + HasRpcClient,
    Chain::Height: Clone,
{
    async fn query_block(
        chain: &Chain,
        height: &Chain::Height,
    ) -> Result<(BlockId, Block), Chain::Error> {
        let tm_height =
            BlockHeight::try_from(height.revision_height()).map_err(Chain::raise_error)?;
        let response = chain
            .rpc_client()
            .block(tm_height)
            .await
            .map_err(Chain::raise_error)?;

        Ok((response.block_id, response.block))
    }
}
