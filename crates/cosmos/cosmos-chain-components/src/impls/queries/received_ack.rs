use core::fmt::Display;

use cgp::prelude::*;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainHeight;
use hermes_relayer_components::chain::traits::queries::packet_is_cleared::{
    PacketIsClearedQuerier, PacketIsClearedQuerierComponent,
};
use hermes_relayer_components::chain::traits::types::ibc::{
    HasChannelIdType, HasPortIdType, HasSequenceType,
};
use ibc::cosmos_host::IBC_QUERY_PATH;

use crate::traits::abci_query::CanQueryAbci;

pub struct QueryCosmosPacketIsCleared;

#[cgp_provider(PacketIsClearedQuerierComponent)]
impl<Chain, Counterparty> PacketIsClearedQuerier<Chain, Counterparty> for QueryCosmosPacketIsCleared
where
    Chain: HasChannelIdType<Counterparty>
        + HasPortIdType<Counterparty>
        + HasSequenceType<Counterparty>
        + CanQueryAbci
        + CanQueryChainHeight
        + HasAsyncErrorType,
    Chain::ChannelId: Display,
{
    async fn query_packet_is_cleared(
        chain: &Chain,
        port_id: &Chain::PortId,
        channel_id: &Chain::ChannelId,
        sequence: &Chain::Sequence,
    ) -> Result<bool, Chain::Error> {
        let height = chain.query_chain_height().await?;

        let commitment_path =
            format!("commitments/ports/{port_id}/channels/{channel_id}/sequences/{sequence}");

        let commitment = chain
            .query_abci(IBC_QUERY_PATH, commitment_path.as_bytes(), &height)
            .await?;

        // Once a packet has been cleared, the chain would have removed its packet commitment

        Ok(commitment.is_empty())
    }
}
