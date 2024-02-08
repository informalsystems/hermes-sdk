use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use eyre::eyre;
use hermes_relayer_components::chain::traits::queries::packet_commitments::PacketCommitmentsQuerier;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_proto::ibc::core::channel::v1::query_client::QueryClient as ChannelQueryClient;
use ibc_relayer::chain::requests::QueryPacketCommitmentsRequest;
use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};
use ibc_relayer_types::Height;
use tonic::Request;

use crate::traits::grpc_address::HasGrpcAddress;

pub struct QueryCosmosPacketCommitments;

#[async_trait]
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
        + CanRaiseError<eyre::Report>,
{
    async fn query_packet_commitments(
        chain: &Chain,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
    ) -> Result<(Vec<Chain::Sequence>, Chain::Height), Chain::Error> {
        let mut client = ChannelQueryClient::connect(chain.grpc_address().clone())
            .await
            .map_err(|e| Chain::raise_error(e.into()))?;

        let raw_request = QueryPacketCommitmentsRequest {
            port_id: port_id.clone(),
            channel_id: channel_id.clone(),
            pagination: None,
        };

        let request = Request::new(raw_request.into());

        let response = client
            .packet_commitments(request)
            .await
            .map_err(|e| Chain::raise_error(e.into()))?
            .into_inner();

        let commitment_sequences: Vec<Sequence> = response
            .commitments
            .into_iter()
            .map(|packet_state| packet_state.sequence.into())
            .collect();

        let raw_height = response
            .height
            .ok_or_else(|| Chain::raise_error(eyre!("missing height in response")))?;

        let height = Height::try_from(raw_height).map_err(|e| Chain::raise_error(e.into()))?;

        Ok((commitment_sequences, height))
    }
}
