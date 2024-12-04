use cgp::core::error::CanRaiseError;
use eyre::eyre;
use hermes_relayer_components::chain::traits::queries::packet_commitments::PacketCommitmentsQuerier;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use http::uri::InvalidUri;
use http::Uri;
use ibc_proto::ibc::core::channel::v1::query_client::QueryClient as ChannelQueryClient;
use ibc_relayer::chain::requests::{Paginate, QueryPacketCommitmentsRequest};
use ibc_relayer_types::core::ics02_client::error::Error as Ics02Error;
use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};
use ibc_relayer_types::Height;
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
        + CanRaiseError<InvalidUri>
        + CanRaiseError<TransportError>
        + CanRaiseError<Status>
        + CanRaiseError<Ics02Error>
        + CanRaiseError<eyre::Report>,
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
            port_id: port_id.clone(),
            channel_id: channel_id.clone(),
            pagination: Paginate::All,
            query_height: ibc_relayer::chain::requests::QueryHeight::Latest,
        };

        let request = Request::new(raw_request.into());

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
