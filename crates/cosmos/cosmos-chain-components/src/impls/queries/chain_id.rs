use cgp_core::error::CanRaiseError;
use hermes_relayer_components::chain::traits::queries::counterparty_chain_id::CounterpartyChainIdQuerier;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer::chain::counterparty::counterparty_chain_from_channel;
use ibc_relayer::supervisor::Error as SupervisorError;
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ChannelId, PortId};

use crate::traits::chain_handle::HasBlockingChainHandle;

pub struct QueryChainIdWithChainHandle;

impl<Chain, Counterparty> CounterpartyChainIdQuerier<Chain, Counterparty>
    for QueryChainIdWithChainHandle
where
    Chain: HasIbcChainTypes<Counterparty, ChannelId = ChannelId, PortId = PortId>
        + HasBlockingChainHandle
        + CanRaiseError<SupervisorError>,
    Counterparty: HasChainIdType<ChainId = ChainId>,
{
    async fn query_counterparty_chain_id_from_channel_id(
        chain: &Chain,
        channel_id: &ChannelId,
        port_id: &PortId,
    ) -> Result<ChainId, Chain::Error> {
        let port_id = port_id.clone();
        let channel_id = channel_id.clone();

        chain
            .with_blocking_chain_handle(move |chain_handle| {
                let channel_id =
                    counterparty_chain_from_channel(&chain_handle, &channel_id, &port_id)
                        .map_err(Chain::raise_error)?;

                Ok(channel_id)
            })
            .await
    }
}
