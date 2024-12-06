use ibc::core::channel::types::channel::Order;
use ibc::core::channel::types::Version;
use ibc::core::host::types::identifiers::ConnectionId;

#[derive(Clone, Debug)]
pub struct CosmosInitChannelOptions {
    pub ordering: Order,
    pub connection_hops: Vec<ConnectionId>,
    pub channel_version: Version,
}

impl Default for CosmosInitChannelOptions {
    fn default() -> Self {
        Self {
            ordering: Order::Unordered,
            connection_hops: Default::default(),
            channel_version: Version::new("ics20-1".to_string()),
        }
    }
}

impl CosmosInitChannelOptions {
    pub fn new(connection_id: ConnectionId) -> Self {
        Self {
            ordering: Order::Unordered,
            connection_hops: vec![connection_id],
            channel_version: Version::new("ics20-1".to_string()),
        }
    }
}
