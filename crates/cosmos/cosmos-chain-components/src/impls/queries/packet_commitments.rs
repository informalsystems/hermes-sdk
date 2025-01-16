use cgp::core::error::CanRaiseAsyncError;
use eyre::eyre;
use hermes_relayer_components::chain::traits::queries::packet_commitments::PacketCommitmentsQuerier;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use http::uri::InvalidUri;
use http::Uri;
use ibc::core::client::types::Height;
use ibc::core::host::types::error::DecodingError;
use ibc::core::host::types::identifiers::{ChannelId, PortId, Sequence};
use ibc_proto::cosmos::base::query::v1beta1::PageRequest;
use ibc_proto::ibc::core::channel::v1::query_client::QueryClient as ChannelQueryClient;
use ibc_proto::ibc::core::channel::v1::QueryPacketCommitmentsRequest;
use tonic::transport::Error as TransportError;
use tonic::{Request, Status};

use crate::traits::grpc_address::HasGrpcAddress;

pub struct QueryCosmosPacketCommitments;

impl<Chain, Counterparty> PacketCommitmentsQuerier<Chain, Counterparty>
    for QueryCosmosPacketCommitments
where
    Chain: HasIbcChainTypes<
            Counterparty,
            Height = Height,
            PortId = PortId,
            ChannelId = ChannelId,
            Sequence = Sequence,
        > + HasGrpcAddress
        + CanRaiseAsyncError<InvalidUri>
        + CanRaiseAsyncError<TransportError>
        + CanRaiseAsyncError<Status>
        + CanRaiseAsyncError<DecodingError>
        + CanRaiseAsyncError<eyre::Report>,
{
    async fn query_packet_commitments(
        chain: &Chain,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
    ) -> Result<(Vec<Chain::Sequence>, Chain::Height), Chain::Error> {
        let mut client = ChannelQueryClient::connect(
            Uri::try_from(&chain.grpc_address().to_string()).map_err(Chain::raise_error)?,
        )
        .await
        .map_err(Chain::raise_error)?;

        let raw_request = QueryPacketCommitmentsRequest {
            port_id: port_id.to_string(),
            channel_id: channel_id.to_string(),
            pagination: Some(PageRequest {
                limit: u32::MAX as u64,
                ..Default::default()
            }),
        };

        let request = Request::new(raw_request);

        let response = client
            .packet_commitments(request)
            .await
            .map_err(Chain::raise_error)?
            .into_inner();

        let commitment_sequences: Vec<Sequence> = response
            .commitments
            .into_iter()
            .map(|packet_state| packet_state.sequence.into())
            .collect();

        let raw_height = response
            .height
            .ok_or_else(|| Chain::raise_error(eyre!("missing height in response")))?;

        let height = Height::try_from(raw_height).map_err(Chain::raise_error)?;

        Ok((commitment_sequences, height))
    }
}
