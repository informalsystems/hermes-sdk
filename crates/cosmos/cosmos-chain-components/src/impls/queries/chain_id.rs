use cgp_core::error::CanRaiseError;
use eyre::Ok;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::{
    queries::counterparty_chain_id::CounterpartyChainIdQuerier, types::channel::HasChannelEndType,
};
// use ibc::core::channel::types::channel::ChannelEnd;
use ibc::core::connection::types::ConnectionEnd;
use ibc_proto::Protobuf;
// use ibc_relayer::chain::counterparty::counterparty_chain_from_channel;
use ibc_relayer::supervisor::Error as SupervisorError;
use ibc_relayer_types::core::ics04_channel::channel::ChannelEnd;
use ibc_relayer_types::core::ics04_channel::channel::State;
use ibc_relayer_types::core::ics24_host::{
    identifier::{ChainId, ChannelId, PortId},
    IBC_QUERY_PATH,
};

use crate::traits::abci_query::CanQueryAbci;

// use crate::traits::chain_handle::HasBlockingChainHandle;

pub struct QueryChainIdFromAbci;

impl<Chain, Counterparty> CounterpartyChainIdQuerier<Chain, Counterparty> for QueryChainIdFromAbci
where
    Chain: HasIbcChainTypes<Counterparty, ChannelId = ChannelId, PortId = PortId>
        + HasChannelEndType<Counterparty, ChannelEnd = ChannelEnd>
        // + HasBlockingChainHandle
        + CanQueryAbci
        + CanRaiseError<SupervisorError>,
    Counterparty: HasChainIdType<ChainId = ChainId>,
{
    // async fn query_counterparty_chain_id_from_channel_id(
    //     chain: &Chain,
    //     channel_id: &ChannelId,
    //     port_id: &PortId,
    // ) -> Result<ChainId, Chain::Error> {
    //     let port_id = port_id.clone();
    //     let channel_id = channel_id.clone();

    //     chain
    //         .with_blocking_chain_handle(move |chain_handle| {
    //             let channel_id =
    //                 counterparty_chain_from_channel(&chain_handle, &channel_id, &port_id)
    //                     .map_err(Chain::raise_error)?;

    //             Ok(channel_id)
    //         })
    //         .await
    // }

    async fn query_counterparty_chain_id_from_channel_id(
        chain: &Chain,
        channel_id: &ChannelId,
        port_id: &PortId,
    ) -> Result<ChainId, Chain::Error> {
        let port_id = port_id.clone();
        let channel_id = channel_id.clone();

        // query channel end with proof
        let channel_end_path = format!("channelEnds/ports/{port_id}/channels/{channel_id}");

        let (channel_end_bytes, proof) = chain
            .query_abci_with_proofs(IBC_QUERY_PATH, channel_end_path.as_bytes(), height)
            .await?;

        let channel_end = ChannelEnd::decode_vec(&channel_end_bytes).map_err(Chain::raise_error)?;

        // check if channel end is initialized, otherwize return error.
        if channel_end.state_matches(&State::Uninitialized) {
            return Err(Error::channel_uninitialized(
                port_id.clone(),
                channel_id.clone(),
                chain.id(),
            ));
        }

        // query connection end
        let connection_path = format!("connections/{connection_id}");

        let connnection_end_bytes = chain
            .query_abci(IBC_QUERY_PATH, connection_path.as_bytes(), height)
            .await?;

        let connection_end =
            ConnectionEnd::decode_vec(&connnection_end_bytes).map_err(Chain::raise_error)?;

        // wanted output of this function is chain id of counterparty chain.
        // so we need to query client state of connection end to get chain id of counterparty chain.

        Ok(())
    }
}
