use std::collections::HashSet;

use cgp_core::error::CanRaiseError;
use eyre::eyre;
use hermes_relayer_components::chain::traits::queries::packet_acknowledgements::PacketAcknowledgementsQuerier;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_proto::ibc::core::channel::v1::query_client::QueryClient as ChannelQueryClient;
use ibc_relayer::chain::requests::{PageRequest, QueryPacketAcknowledgementsRequest};
use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};
use ibc_relayer_types::Height;
use tonic::Request;

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
        + CanRaiseError<eyre::Report>,
    Counterparty: HasIbcChainTypes<Chain, Sequence = Sequence, Height = Height>,
{
    async fn query_packet_acknowlegements(
        chain: &Chain,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
        sequences: &[Counterparty::Sequence],
    ) -> Result<Option<(Vec<Counterparty::Sequence>, Chain::Height)>, Chain::Error> {
        let mut client = ChannelQueryClient::connect(chain.grpc_address().clone())
            .await
            .map_err(|e| Chain::raise_error(e.into()))?;

        let raw_request = QueryPacketAcknowledgementsRequest {
            port_id: port_id.clone(),
            channel_id: channel_id.clone(),
            pagination: Some(PageRequest::all()),
            packet_commitment_sequences: sequences.to_vec(),
        };

        let request = Request::new(raw_request.into());

        let response = client
            .packet_acknowledgements(request)
            .await
            .map_err(|e| Chain::raise_error(e.into()))?
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

        let height = Height::try_from(raw_height).map_err(|e| Chain::raise_error(e.into()))?;

        Ok(Some((response_acks, height)))
    }
}
