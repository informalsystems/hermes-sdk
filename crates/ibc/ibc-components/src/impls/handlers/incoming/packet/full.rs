use core::marker::PhantomData;

// use cgp::core::component::UseContext;
use cgp::prelude::*;

// use crate::impls::handlers::incoming::packet::error_ack::WrapHandlerErrorAsAck;
use crate::impls::handlers::incoming::packet::no_replay::DisallowDoubleReceive;
use crate::impls::handlers::incoming::packet::payloads::HandleIncomingPacketPayloads;
use crate::impls::handlers::incoming::packet::store_recv::StoreRecvPacket;
use crate::impls::handlers::incoming::packet::timeout::DisallowTimedOutIncomingPacket;
use crate::impls::handlers::incoming::packet::verify::VerifySendPacketCommitmentProof;
pub use crate::traits::handlers::incoming::packet::IncomingPacketHandlerComponent;

pub struct FullIncomingPacketHandler<App>(pub PhantomData<App>);

delegate_components! {
    <App>
    FullIncomingPacketHandler<App> {
        IncomingPacketHandlerComponent:
            VerifySendPacketCommitmentProof<
                DisallowDoubleReceive<
                    DisallowTimedOutIncomingPacket<
                        StoreRecvPacket<
                            // WrapHandlerErrorAsAck<
                                // UseContext,
                                HandleIncomingPacketPayloads<App>
                            // >
                        >
                    >
                >
            >,
    }
}
