use cgp::core::error::CanRaiseError;
use hermes_relayer_components::chain::traits::queries::consensus_state_height::ConsensusStateHeightsQuerier;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_proto::ibc::core::client::v1::query_client::QueryClient;
use ibc_relayer::chain::requests::QueryConsensusStateHeightsRequest;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use ibc_relayer_types::Height;
use tonic::transport::Error as TransportError;
use tonic::Status;

use crate::traits::grpc_address::HasGrpcAddress;

pub struct QueryConsensusStateHeightsFromGrpc;

impl<Chain, Counterparty> ConsensusStateHeightsQuerier<Chain, Counterparty>
    for QueryConsensusStateHeightsFromGrpc
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId>
        + HasGrpcAddress
        + CanRaiseError<TransportError>
        + CanRaiseError<Status>,
    Counterparty: HasHeightType<Height = Height>,
{
    async fn query_consensus_state_heights(
        chain: &Chain,
        client_id: &ClientId,
    ) -> Result<Vec<Height>, Chain::Error> {
        let mut client = QueryClient::connect(chain.grpc_address().clone())
            .await
            .map_err(Chain::raise_error)?
            .max_decoding_message_size(33554432);

        let request = QueryConsensusStateHeightsRequest {
            client_id: client_id.clone(),
            pagination: None,
        };

        let response = client
            .consensus_state_heights(tonic::Request::new(request.into()))
            .await
            .map_err(Chain::raise_error)?
            .into_inner();

        let mut heights: Vec<Height> = response
            .consensus_state_heights
            .into_iter()
            .filter_map(|height| height.try_into().ok())
            .collect();

        heights.sort_unstable();

        Ok(heights)
    }
}
