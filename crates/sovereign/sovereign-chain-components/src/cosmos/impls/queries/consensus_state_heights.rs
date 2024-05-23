use cgp_core::CanRaiseError;
use hermes_cosmos_chain_components::traits::grpc_address::HasGrpcAddress;
use hermes_relayer_components::chain::traits::queries::consensus_state_height::ConsensusStateHeightsQuerier;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_sovereign_rollup_components::types::height::RollupHeight;
use ibc_proto::ibc::core::client::v1::query_client::QueryClient;
use ibc_relayer::chain::requests::QueryConsensusStateHeightsRequest;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use tonic::transport::Error as TransportError;
use tonic::Status;

pub struct QuerySovereignConsensusStateHeightsFromGrpc;

impl<Chain, Counterparty> ConsensusStateHeightsQuerier<Chain, Counterparty>
    for QuerySovereignConsensusStateHeightsFromGrpc
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId>
        + HasGrpcAddress
        + CanRaiseError<TransportError>
        + CanRaiseError<Status>,
    Counterparty: HasHeightType<Height = RollupHeight>,
{
    async fn query_consensus_state_heights(
        chain: &Chain,
        client_id: &ClientId,
    ) -> Result<Vec<RollupHeight>, Chain::Error> {
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

        let mut heights: Vec<RollupHeight> = response
            .consensus_state_heights
            .into_iter()
            .map(|height| RollupHeight {
                slot_number: height.revision_height,
            })
            .collect();

        heights.sort_unstable();

        Ok(heights)
    }
}
