use std::collections::HashSet;

use cgp::core::error::CanRaiseError;
use eyre::eyre;
use hermes_relayer_components::chain::traits::queries::packet_acknowledgements::PacketAcknowledgementsQuerier;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use http::uri::InvalidUri;
use http::Uri;
use ibc::core::client::types::Height;
use ibc::core::host::types::error::DecodingError;
use ibc::core::host::types::identifiers::{ChannelId, PortId, Sequence};
use ibc_proto::cosmos::base::query::v1beta1::PageRequest;
use ibc_proto::ibc::core::channel::v1::query_client::QueryClient as ChannelQueryClient;
use ibc_proto::ibc::core::channel::v1::QueryPacketAcknowledgementsRequest;
use tonic::transport::Error as TransportError;
use tonic::{Request, Status};

use crate::traits::grpc_address::HasGrpcAddress;

pub struct QueryCosmosPacketAcknowledgements;

impl<Chain, Counterparty> PacketAcknowledgementsQuerier<Chain, Counterparty>
    for QueryCosmosPacketAcknowledgements
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
        + CanRaiseError<DecodingError>
        + CanRaiseError<Status>
        + CanRaiseError<eyre::Report>,
    Counterparty: HasIbcChainTypes<Chain, Sequence = Sequence, Height = Height>,
{
    async fn query_packet_acknowlegements(
        chain: &Chain,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
        sequences: &[Counterparty::Sequence],
    ) -> Result<Option<(Vec<Counterparty::Sequence>, Chain::Height)>, Chain::Error> {
        let mut client = ChannelQueryClient::connect(
            Uri::try_from(&chain.grpc_address().to_string()).map_err(Chain::raise_error)?,
        )
        .await
        .map_err(Chain::raise_error)?;

        let raw_request = QueryPacketAcknowledgementsRequest {
            port_id: port_id.to_string(),
            channel_id: channel_id.to_string(),
            pagination: Some(PageRequest {
                limit: u32::MAX as u64,
                ..Default::default()
            }),
            packet_commitment_sequences: sequences
                .iter()
                .map(|sequence| sequence.value())
                .collect(),
        };

        let request = Request::new(raw_request);

        let response = client
            .packet_acknowledgements(request)
            .await
            .map_err(Chain::raise_error)?
            .into_inner();

        let commit_set = sequences.iter().cloned().collect::<HashSet<_>>();

        let mut response_acks: Vec<Sequence> = response
            .acknowledgements
            .into_iter()
            .map(|packet_state| packet_state.sequence.into())
            .collect();

        response_acks.retain(|s| commit_set.contains(s));
        response_acks.sort_unstable();

        let raw_height = response
            .height
            .ok_or_else(|| Chain::raise_error(eyre!("missing height in response")))?;

        let height = Height::try_from(raw_height).map_err(Chain::raise_error)?;

        Ok(Some((response_acks, height)))
    }
}
