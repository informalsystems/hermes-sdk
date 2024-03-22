use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use tendermint::block::Height as TendermintHeight;
use tendermint_rpc::endpoint::abci_query::AbciQuery;
use tendermint_rpc::{Client, Error as RpcError};

use crate::traits::abci_query::AbciQuerier;
use crate::traits::rpc_client::HasRpcClient;

pub struct QueryAbci;

#[derive(Debug)]
pub struct AbciQueryError {
    pub response: AbciQuery,
}

impl<Chain> AbciQuerier<Chain> for QueryAbci
where
    Chain: HasRpcClient + HasHeightType + CanRaiseError<RpcError> + CanRaiseError<AbciQueryError>,
    Chain::Height: Clone + Into<TendermintHeight>,
{
    async fn query_abci(
        chain: &Chain,
        path: &str,
        data: &[u8],
        height: &Chain::Height,
    ) -> Result<Vec<u8>, Chain::Error> {
        let response = chain
            .rpc_client()
            .abci_query(
                Some(path.to_owned()),
                data,
                Some(height.clone().into()),
                false,
            )
            .await
            .map_err(Chain::raise_error)?;

        if !response.code.is_ok() {
            return Err(Chain::raise_error(AbciQueryError { response }));
        }

        Ok(response.value)
    }
}
