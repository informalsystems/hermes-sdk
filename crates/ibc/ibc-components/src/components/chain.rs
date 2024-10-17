use cgp::prelude::*;

use crate::impls::handlers::incoming::packet::full::FullIncomingPacketHandler;
pub use crate::traits::handlers::incoming::packet::IncomingPacketHandlerComponent;
use crate::types::tags::apps::any::AnyApp;

define_components! {
    IbcChainComponents {
        IncomingPacketHandlerComponent: FullIncomingPacketHandler<AnyApp>,
    }
}
