use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::queries::connection_end::ConnectionEndQuerier;
use hermes_relayer_components::chain::traits::types::connection::HasConnectionEndType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc::core::connection::types::ConnectionEnd;
use ibc_query::core::connection::QueryConnectionResponse;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::ClientError;
use serde::Serialize;

use crate::traits::json_rpc_client::HasJsonRpcClient;
use crate::types::height::RollupHeight;
use crate::types::rpc::height::HeightParam;

pub struct QueryConnectionEndOnSovereign;

impl<Rollup, Counterparty> ConnectionEndQuerier<Rollup, Counterparty>
    for QueryConnectionEndOnSovereign
where
    Rollup: HasConnectionEndType<Counterparty, ConnectionEnd = ConnectionEnd>
        + HasIbcChainTypes<Counterparty, Height = RollupHeight>
        + HasJsonRpcClient
        + CanRaiseError<ClientError>,
    Rollup::JsonRpcClient: ClientT,
{
    async fn query_connection_end(
        rollup: &Rollup,
        connection_id: &Rollup::ConnectionId,
        height: &Rollup::Height,
    ) -> Result<Rollup::ConnectionEnd, Rollup::Error> {
        let request = Request {
            connection_id: &connection_id.to_string(),
            query_height: &height.into(),
        };

        let response: QueryConnectionResponse = rollup
            .json_rpc_client()
            .request("ibc_connection", (request,))
            .await
            .map_err(Rollup::raise_error)?;

        Ok(response.conn_end)
    }
}

#[derive(Serialize)]
pub struct Request<'a> {
    pub connection_id: &'a str,
    pub query_height: &'a HeightParam,
}
