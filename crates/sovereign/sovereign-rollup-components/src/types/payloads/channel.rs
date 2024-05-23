use ibc_relayer_types::core::ics04_channel::channel::Ordering;
use ibc_relayer_types::core::ics04_channel::version::Version;
use ibc_relayer_types::core::ics24_host::identifier::ConnectionId;

#[derive(Clone, Default, Debug)]
pub struct SovereignInitChannelOptions {
    pub ordering: Ordering,
    pub connection_hops: Vec<ConnectionId>,
    pub channel_version: Version,
}
