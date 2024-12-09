use ibc::core::host::types::identifiers::{ChannelId, PortId};

#[derive(Clone, Default)]
pub struct PacketFilterConfig;

impl PacketFilterConfig {
    /// TODO: Use proper packet filtering
    pub fn is_allowed(&self, _port_id: &PortId, _channel_id: &ChannelId) -> bool {
        true
    }
}
