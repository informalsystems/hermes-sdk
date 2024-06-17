use ibc_relayer_types::core::ics04_channel::channel::Ordering;
use ibc_relayer_types::core::ics04_channel::version::Version;
use ibc_relayer_types::core::ics24_host::identifier::ConnectionId;

#[derive(Default, Clone, Debug)]
pub struct CosmosInitChannelOptions {
    pub ordering: Ordering,
    pub connection_hops: Vec<ConnectionId>,
    pub channel_version: Version,
}

impl CosmosInitChannelOptions {
    pub fn new(connection_id: ConnectionId) -> Self {
        Self {
            ordering: Ordering::Unordered,
            connection_hops: vec![connection_id],
            channel_version: Version::default(),
        }
    }
}
