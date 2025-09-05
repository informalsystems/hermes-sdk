use core::marker::PhantomData;

use hermes_core::relayer_components::chain::traits::{
    CanQueryChainHeight, CanQueryClientState, CounterpartyChainIdQuerier,
    CounterpartyChainIdQuerierComponent, HasChainIdType, HasClientStateFields, HasIbcChainTypes,
};
use hermes_prelude::*;
use ibc::core::channel::types::channel::{ChannelEnd, State};
use ibc::core::channel::types::error::ChannelError;
use ibc::core::connection::types::ConnectionEnd;
use ibc::core::host::types::error::IdentifierError;
use ibc::core::host::types::identifiers::{ChainId, ChannelId, ClientId, PortId};
use ibc::cosmos_host::IBC_QUERY_PATH;
use ibc_proto::Protobuf;
use tendermint_proto::Error as TendermintProtoError;

use crate::traits::CanQueryAbci;

pub struct QueryChainIdFromAbci;

#[cgp_provider(CounterpartyChainIdQuerierComponent)]
impl<Chain, Counterparty> CounterpartyChainIdQuerier<Chain, Counterparty> for QueryChainIdFromAbci
where
    Chain: HasIbcChainTypes<Counterparty, ChannelId = ChannelId, PortId = PortId, ClientId = ClientId>
        + CanQueryChainHeight
        + CanQueryAbci
        + CanQueryClientState<Counterparty>
        + CanRaiseAsyncError<ChannelError>
        + CanRaiseAsyncError<IdentifierError>
        + CanRaiseAsyncError<TendermintProtoError>
        + CanRaiseAsyncError<String>,
    Counterparty: HasChainIdType<ChainId = ChainId> + HasClientStateFields<Chain>,
{
    async fn query_counterparty_chain_id_from_channel_id(
        chain: &Chain,
        channel_id: &ChannelId,
        port_id: &PortId,
    ) -> Result<Counterparty::ChainId, Chain::Error> {
        let port_id = port_id.clone();
        let channel_id = channel_id.clone();

        let latest_height = chain.query_chain_height().await?;

        // channel end query path
        let channel_end_path = format!("channelEnds/ports/{port_id}/channels/{channel_id}");

        let channel_end_bytes = chain
            .query_abci(
                IBC_QUERY_PATH,
                channel_end_path.as_bytes(),
                Some(&latest_height),
            )
            .await?
            .ok_or_else(|| {
                Chain::raise_error(format!("channel not found: {channel_id}/{port_id}"))
            })?;

        let channel_end = ChannelEnd::decode_vec(&channel_end_bytes).map_err(Chain::raise_error)?;

        // check if channel end is initialized, otherwize return error.
        if channel_end
            .verify_state_matches(&State::Uninitialized)
            .is_ok()
        {
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
            .query_abci(
                IBC_QUERY_PATH,
                connection_path.as_bytes(),
                Some(&latest_height),
            )
            .await?
            .ok_or_else(|| Chain::raise_error(format!("connection not found: {connection_id}")))?;

        let connection_end =
            ConnectionEnd::decode_vec(&connnection_end_bytes).map_err(Chain::raise_error)?;

        let client_id = connection_end.client_id();

        let client_state = chain
            .query_client_state(PhantomData, client_id, &latest_height)
            .await?;

        Ok(Counterparty::client_state_chain_id(&client_state))
    }
}
