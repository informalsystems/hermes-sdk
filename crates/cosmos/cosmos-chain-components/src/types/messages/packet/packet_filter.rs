use std::collections::HashMap;

use ibc::core::host::types::identifiers::{ChannelId, PortId};

#[derive(Clone, Default)]
pub struct PacketFilterConfig {
    pub filter_map: HashMap<(ChannelId, PortId), bool>,
}

impl PacketFilterConfig {
    pub fn new(filter_map: HashMap<(ChannelId, PortId), bool>) -> Self {
        Self { filter_map }
    }
}

impl PacketFilterConfig {
    pub fn is_allowed(&self, port_id: &PortId, channel_id: &ChannelId) -> bool {
        *self
            .filter_map
            .get(&(channel_id.clone(), port_id.clone()))
            .unwrap_or(&true)
    }
}
