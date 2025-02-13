use core::fmt;

use ibc::core::host::types::identifiers::Sequence;
use serde::Serialize;

pub mod collate;
use collate::{Collated, CollatedIterExt};

/// A structure to display pending packet commitment IDs
/// at one end of a channel.
#[derive(Debug, Serialize)]
pub struct PendingPackets {
    /// Not yet received on the counterparty chain.
    pub unreceived_packets: Vec<Sequence>,
    /// Received on the counterparty chain,
    /// but the acknowledgement is not yet received on the local chain.
    pub unreceived_acks: Vec<Sequence>,
}

#[derive(Serialize)]
pub struct CollatedPendingPackets {
    pub unreceived_packets: Vec<Collated<u64>>,
    pub unreceived_acks: Vec<Collated<u64>>,
}

impl fmt::Debug for CollatedPendingPackets {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PendingPackets")
            .field("unreceived_packets", &self.unreceived_packets)
            .field("unreceived_acks", &self.unreceived_acks)
            .finish()
    }
}

impl CollatedPendingPackets {
    pub fn new(pending: PendingPackets) -> Self {
        Self {
            unreceived_packets: pending
                .unreceived_packets
                .into_iter()
                .map(u64::from)
                .collated()
                .collect(),
            unreceived_acks: pending
                .unreceived_acks
                .into_iter()
                .map(u64::from)
                .collated()
                .collect(),
        }
    }
}
