use core::fmt::Display;

use cgp::prelude::HasAsyncErrorType;
use hermes_relayer_components::chain::traits::queries::ack_is_received::AckIsReceivedQuerier;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainHeight;
use hermes_relayer_components::chain::traits::types::ibc::{
    HasChannelIdType, HasPortIdType, HasSequenceType,
};
use ibc::cosmos_host::IBC_QUERY_PATH;

use crate::traits::abci_query::CanQueryAbci;

pub struct QueryCosmosAckedPacket;

impl<Chain, Counterparty> AckIsReceivedQuerier<Chain, Counterparty> for QueryCosmosAckedPacket
where
    Chain: HasChannelIdType<Counterparty>
        + HasPortIdType<Counterparty>
        + HasSequenceType<Counterparty>
        + CanQueryAbci
        + CanQueryChainHeight
        + HasAsyncErrorType,
    Chain::ChannelId: Display,
{
    async fn query_ack_is_received(
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

        // Checks if a packet commitment has been cleared on source.
        // The packet commitment is cleared when either an acknowledgment or a timeout is received on source.

        Ok(commitment.is_empty())
    }
}
