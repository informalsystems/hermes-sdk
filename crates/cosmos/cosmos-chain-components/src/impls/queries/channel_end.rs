use cgp_core::CanRaiseError;

use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;

use ibc_relayer_types::core::ics24_host::identifier::ConnectionId;
use ibc_relayer_types::Height;

use tendermint_proto::Error as TendermintProtoError;

pub struct QueryCosmosChannelEnd;

impl<Chain, Counterparty> ChannelEndQuerier<Chain, Counterparty> for QueryCosmosChannelEnd
where
    Chain: HasChannelEndType
        + HasIbcChainTypes<Counterparty, Height = Height, ConnectionId = ConnectionId>
        + CanRaiseError<TendermintProtoError>,
{
    async fn query_channel_end() -> Result<ChannelEnd, Chain::Error> {}
}
