use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, ConnectionId};

use crate::contexts::chain_driver::CosmosChainDriver;
use crate::contexts::relay_driver::CosmosRelayDriver;

pub struct CosmosBinaryChannelTestDriver {
    pub relay_driver: CosmosRelayDriver,
    pub chain_driver_a: CosmosChainDriver,
    pub chain_driver_b: CosmosChainDriver,
    pub connection_id_a: ConnectionId,
    pub connection_id_b: ConnectionId,
    pub channel_id_a: ChannelId,
    pub channel_id_b: ChannelId,
}
