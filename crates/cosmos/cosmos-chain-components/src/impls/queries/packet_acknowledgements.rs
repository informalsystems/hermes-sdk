use std::collections::HashSet;

use cgp::prelude::*;
use hermes_relayer_components::chain::traits::queries::packet_acknowledgements::{
    PacketAcknowledgementsQuerier, PacketAcknowledgementsQuerierComponent,
};
use hermes_relayer_components::chain::traits::types::ibc::{HasIbcChainTypes, HasSequenceType};
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

#[cgp_provider(PacketAcknowledgementsQuerierComponent)]
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
        + CanRaiseAsyncError<InvalidUri>
        + CanRaiseAsyncError<TransportError>
        + CanRaiseAsyncError<DecodingError>
        + CanRaiseAsyncError<Status>
        + CanRaiseAsyncError<eyre::Report>,
    Counterparty: HasSequenceType<Chain, Sequence = Sequence>,
{
    async fn query_packet_acknowlegements(
        chain: &Chain,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
        sequences: &[Counterparty::Sequence],
    ) -> Result<Option<Vec<Counterparty::Sequence>>, Chain::Error> {
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

        Ok(Some(response_acks))
    }
}
