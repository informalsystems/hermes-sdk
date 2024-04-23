use cgp_core::prelude::HasErrorType;
use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::queries::client_state::ClientStateBytesQuerier;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc::core::client::types::error::ClientError as Ics02Error;
use ibc::core::client::types::Height;
use ibc::core::host::types::error::IdentifierError;
use ibc::core::host::types::identifiers::ClientId as IbcClientId;
use ibc_query::core::client::{QueryClientStateRequest, QueryClientStateResponse};
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::ClientError;
use prost::Message;

use crate::traits::json_rpc_client::HasJsonRpcClient;
use crate::types::height::RollupHeight;

pub struct QueryClientStateOnSovereign;

impl<Rollup, Counterparty> ClientStateBytesQuerier<Rollup, Counterparty>
    for QueryClientStateOnSovereign
where
    Rollup: HasIbcChainTypes<Counterparty, ClientId = ClientId, Height = RollupHeight>
        + HasJsonRpcClient
        + HasErrorType
        + CanRaiseError<ClientError>
        + CanRaiseError<Ics02Error>
        + CanRaiseError<IdentifierError>,
    Rollup::JsonRpcClient: ClientT,
{
    async fn query_client_state_bytes(
        rollup: &Rollup,
        client_id: &ClientId,
        height: &RollupHeight,
    ) -> Result<Vec<u8>, Rollup::Error> {
        let normalized_height = Height::new(0, height.slot_number).map_err(Rollup::raise_error)?;

        let client_id_param: IbcClientId =
            client_id.as_str().parse().map_err(Rollup::raise_error)?;

        let request = QueryClientStateRequest {
            client_id: client_id_param,
            query_height: Some(normalized_height),
        };

        let response: QueryClientStateResponse = rollup
            .json_rpc_client()
            .request("ibc_clientState", (request,))
            .await
            .map_err(Rollup::raise_error)?;

        let client_state_bytes = Message::encode_to_vec(&response.client_state);

        Ok(client_state_bytes)
    }
}
