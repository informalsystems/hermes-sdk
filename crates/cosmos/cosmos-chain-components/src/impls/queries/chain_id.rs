use cgp::core::error::CanRaiseError;

use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainHeight;
use hermes_relayer_components::chain::traits::queries::counterparty_chain_id::CounterpartyChainIdQuerier;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;

use ibc::core::connection::types::ConnectionEnd;
use ibc_relayer::client_state::AnyClientState;
use ibc_relayer_types::core::ics04_channel::channel::ChannelEnd;
use ibc_relayer_types::core::ics04_channel::channel::State;
use ibc_relayer_types::core::ics24_host::{
    identifier::{ChainId, ChannelId, PortId},
    IBC_QUERY_PATH,
};

use ibc_proto::Protobuf;
use tendermint_proto::Error as TendermintProtoError;

use crate::traits::abci_query::CanQueryAbci;

pub struct QueryChainIdFromAbci;

impl<Chain, Counterparty> CounterpartyChainIdQuerier<Chain, Counterparty> for QueryChainIdFromAbci
where
    Chain: HasIbcChainTypes<Counterparty, ChannelId = ChannelId, PortId = PortId>
        + CanQueryChainHeight
        + CanQueryAbci
        + CanRaiseError<TendermintProtoError>
        + CanRaiseError<String>,
    Counterparty: HasChainIdType<ChainId = ChainId>,
{
    async fn query_counterparty_chain_id_from_channel_id(
        chain: &Chain,
        channel_id: &ChannelId,
        port_id: &PortId,
    ) -> Result<ChainId, Chain::Error> {
        let port_id = port_id.clone();
        let channel_id = channel_id.clone();

        let latest_height = chain.query_chain_height().await?;

        // channel end query path
        let channel_end_path = format!("channelEnds/ports/{port_id}/channels/{channel_id}");

        let channel_end_bytes = chain
            .query_abci(IBC_QUERY_PATH, channel_end_path.as_bytes(), &latest_height)
            .await?;

        let channel_end = ChannelEnd::decode_vec(&channel_end_bytes).map_err(Chain::raise_error)?;

        // check if channel end is initialized, otherwize return error.
        if channel_end.state_matches(&State::Uninitialized) {
            return Err(Chain::raise_error(format!(
                "channel with id `{channel_id}` is uninitialized"
            )));
        }

        let connection_id = channel_end.connection_hops.first().ok_or_else(|| {
            Chain::raise_error(format!("channel with id `{channel_id}` has no connections"))
        })?;

        // connection end query path
        let connection_path = format!("connections/{connection_id}");

        let connnection_end_bytes = chain
            .query_abci(IBC_QUERY_PATH, connection_path.as_bytes(), &latest_height)
            .await?;

        let connection_end =
            ConnectionEnd::decode_vec(&connnection_end_bytes).map_err(Chain::raise_error)?;

        let client_id = connection_end.client_id();

        // client state query path
        let client_state_path = format!("clients/{client_id}/clientState");

        let client_state_bytes = chain
            .query_abci(IBC_QUERY_PATH, client_state_path.as_bytes(), &latest_height)
            .await?;

        let client_state =
            AnyClientState::decode_vec(&client_state_bytes).map_err(Chain::raise_error)?;

        Ok(client_state.chain_id())
    }
}
