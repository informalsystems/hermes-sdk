use cgp::prelude::*;

use crate::impls::handlers::incoming::packet::entries::HandleIncomingPacketEntries;
use crate::impls::handlers::incoming::packet::no_replay::DisallowDoubleReceive;
use crate::impls::handlers::incoming::packet::store_ack::StorePacketAck;
use crate::impls::handlers::incoming::packet::timeout::DisallowTimedOutIncomingPacket;
use crate::impls::handlers::incoming::packet::verify::VerifySendPacketCommitmentProof;
pub use crate::traits::handlers::incoming::packet::IncomingPacketHandlerComponent;
use crate::types::any_app::AnyApp;

pub struct FullIncomingPacketHandler;

delegate_components! {
    FullIncomingPacketHandler {
        IncomingPacketHandlerComponent:
            VerifySendPacketCommitmentProof<
                DisallowDoubleReceive<
                    DisallowTimedOutIncomingPacket<
                        StorePacketAck<
                            HandleIncomingPacketEntries<AnyApp>
                        >
                    >
                >
            >,
    }
}
