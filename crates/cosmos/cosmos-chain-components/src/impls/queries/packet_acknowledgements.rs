use std::collections::HashSet;

use cgp::core::error::CanRaiseError;
use eyre::eyre;
use hermes_relayer_components::chain::traits::queries::packet_acknowledgements::PacketAcknowledgementsQuerier;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use http::uri::InvalidUri;
use http::Uri;
use ibc_proto::ibc::core::channel::v1::query_client::QueryClient as ChannelQueryClient;
use ibc_relayer::chain::requests::{Paginate, QueryPacketAcknowledgementsRequest};
use ibc_relayer_types::core::ics02_client::error::Error as Ics02Error;
use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};
use ibc_relayer_types::Height;
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
        + CanRaiseError<Status>
        + CanRaiseError<Ics02Error>
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
            port_id: port_id.clone(),
            channel_id: channel_id.clone(),
            pagination: Paginate::All,
            packet_commitment_sequences: sequences.to_vec(),
        };

        let request = Request::new(raw_request.into());

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
