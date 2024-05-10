use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::queries::connection_end::{
    ConnectionEndQuerier, ConnectionNotFoundError,
};
use hermes_relayer_components::chain::traits::types::connection::HasConnectionEndType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_proto::ibc::core::connection::v1::query_client::QueryClient;
use ibc_relayer_types::core::ics03_connection::connection::ConnectionEnd;
use ibc_relayer_types::core::ics03_connection::error::Error as Ics03Error;
use ibc_relayer_types::core::ics24_host::identifier::ConnectionId;
use ibc_relayer_types::Height;
use tonic::metadata::errors::InvalidMetadataValue;
use tonic::metadata::AsciiMetadataValue;
use tonic::transport::Error as TransportError;
use tonic::Status;

use crate::traits::grpc_address::HasGrpcAddress;

pub struct QueryCosmosConnectionEndFromChainHandle;

impl<Chain, Counterparty> ConnectionEndQuerier<Chain, Counterparty>
    for QueryCosmosConnectionEndFromChainHandle
where
    Chain: HasConnectionEndType<Counterparty, ConnectionEnd = ConnectionEnd>
        + HasIbcChainTypes<Counterparty, Height = Height, ConnectionId = ConnectionId>
        + HasGrpcAddress
        + CanRaiseError<TransportError>
        + CanRaiseError<InvalidMetadataValue>
        + CanRaiseError<Status>
        + CanRaiseError<Ics03Error>
        + for<'a> CanRaiseError<ConnectionNotFoundError<'a, Chain, Counterparty>>,
{
    async fn query_connection_end(
        chain: &Chain,
        connection_id: &ConnectionId,
        height: &Height,
    ) -> Result<ConnectionEnd, Chain::Error> {
        let mut client = QueryClient::connect(chain.grpc_address().clone())
            .await
            .map_err(Chain::raise_error)?;

        let mut request = tonic::Request::new(
            ibc_proto::ibc::core::connection::v1::QueryConnectionRequest {
                connection_id: connection_id.to_string(),
            },
        );

        let height_metadata = AsciiMetadataValue::try_from(height.revision_height().to_string())
            .map_err(Chain::raise_error)?;

        request
            .metadata_mut()
            .insert("x-cosmos-block-height", height_metadata);

        let response = client.connection(request).await.map_err(|e| {
            if e.code() == tonic::Code::NotFound {
                Chain::raise_error(ConnectionNotFoundError {
                    chain,
                    connection_id,
                    height,
                })
            } else {
                Chain::raise_error(e)
            }
        })?;

        let raw_connection = response.into_inner().connection.ok_or_else(|| {
            Chain::raise_error(ConnectionNotFoundError {
                chain,
                connection_id,
                height,
            })
        })?;

        let connection_end = ConnectionEnd::try_from(raw_connection).map_err(Chain::raise_error)?;

        Ok(connection_end)
    }
}
