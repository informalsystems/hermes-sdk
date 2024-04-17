// use cgp_core::prelude::*;
// use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::queries::channel::ChannelBytesQuerier;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_types::core::ics24_host::identifier::{PortId, ChannelId};
use ibc_relayer_types::Height;

use crate::traits::abci_query::CanQueryAbci;

pub struct QueryCosmosChannelFromAbci;

pub const IBC_QUERY_PATH: &str = "store/ibc/key";

impl<Chain, Counterparty> ChannelBytesQuerier<Chain, Counterparty>
    for QueryCosmosChannelFromAbci
where
    Chain: HasIbcChainTypes<Counterparty, PortId = PortId, ChannelId = ChannelId, Height = Height> + CanQueryAbci,
{
    async fn query_channel_bytes(
        chain: &Chain,
        port_id: &PortId,
        channel_id: &ChannelId,
        height: &Height,
    ) -> Result<Vec<u8>, Chain::Error> {
        let channel_ends_path = format!("channelEnds/ports/{port_id}/channels/{channel_id}");

        let channel_bytes = chain
            .query_abci(IBC_QUERY_PATH, channel_ends_path.as_bytes(), height)
            .await?;

        Ok(channel_bytes)
    }
}