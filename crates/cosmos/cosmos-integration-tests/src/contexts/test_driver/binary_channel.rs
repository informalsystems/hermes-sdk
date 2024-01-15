use hermes_cosmos_relayer::contexts::birelay::CosmosBiRelay;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, ConnectionId};

use crate::contexts::chain_driver::CosmosChainDriver;

pub struct CosmosBinaryChannelTestDriver {
    pub birelay: CosmosBiRelay,
    pub chain_driver_a: CosmosChainDriver,
    pub chain_driver_b: CosmosChainDriver,
    pub connection_id_a: ConnectionId,
    pub connection_id_b: ConnectionId,
    pub channel_id_a: ChannelId,
    pub channel_id_b: ChannelId,
}
