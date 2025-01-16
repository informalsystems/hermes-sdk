use cgp::core::error::CanRaiseAsyncError;
use hermes_relayer_components::chain::traits::queries::block::BlockQuerier;
use hermes_relayer_components::chain::traits::types::block::HasBlockType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use ibc::core::client::types::Height;
use tendermint::block::{Block, Height as BlockHeight, Id as BlockId};
use tendermint::Error as TendermintError;
use tendermint_rpc::{Client, Error as RpcError};

use crate::traits::rpc_client::HasRpcClient;

pub struct QueryCometBlock;

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
