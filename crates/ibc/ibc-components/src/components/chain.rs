#[cgp::re_export_imports]
mod preset {
    use cgp::core::component::WithContext;
    use hermes_prelude::*;

    use crate::impls::handlers::incoming::packet::full::FullIncomingPacketHandler;
    use crate::impls::handlers::outgoing::packet::build::AllocateNonceAndBuildPacket;
    use crate::impls::handlers::outgoing::packet::commit::CommitSendPacket;
    use crate::traits::builders::packet::PacketBuilderComponent;
    use crate::traits::fields::message::app_id::IbcMessageAppIdGetterComponent;
    use crate::traits::fields::packet::header::channel_id::PacketChannelIdGetterComponent;
    use crate::traits::fields::packet::header::timeout::PacketTimeoutGetterComponent;
    use crate::traits::fields::packet::packet::header::PacketHeaderGetterComponent;
    use crate::traits::fields::packet::packet::nonce::PacketNonceGetterComponent;
    use crate::traits::fields::packet::packet::payloads::PacketPayloadsGetterComponent;
    use crate::traits::fields::payload::app_id::PayloadAppIdGetterComponent;
    use crate::traits::fields::payload::data::PayloadDataGetterComponent;
    use crate::traits::fields::payload::header::PayloadHeaderGetterComponent;
    use crate::traits::handlers::incoming::packet::IncomingPacketHandlerComponent;
    use crate::traits::handlers::outgoing::packet::PacketSenderComponent;
    use crate::traits::types::message_header::IbcMessageHeaderTypeComponent;
    use crate::traits::types::packet::header::PacketHeaderTypeComponent;
    use crate::traits::types::packet::packet::PacketTypeComponent;
    use crate::traits::types::payload::header::PayloadHeaderTypeComponent;
    use crate::traits::types::payload::payload::PayloadTypeComponent;
    use crate::types::message_header::UseIbcMessageHeader;
    use crate::types::packet::UseIbcPacket;
    use crate::types::packet_header::UseIbcPacketHeader;
    use crate::types::payload::UseIbcPayload;
    use crate::types::payload_header::UseIbcPayloadHeader;
    use crate::types::tags::apps::any::AnyApp;

    cgp_preset! {
        IbcChainComponents {
            [
                PacketTypeComponent,
                PacketBuilderComponent,
            ]:
                UseIbcPacket,
            PacketHeaderTypeComponent:
                UseIbcPacketHeader,
            PayloadTypeComponent:
                UseIbcPayload<AnyApp>,
            PayloadHeaderTypeComponent:
                UseIbcPayloadHeader,
            IbcMessageHeaderTypeComponent:
                UseIbcMessageHeader,
            [
                PacketHeaderGetterComponent,
                PacketChannelIdGetterComponent,
                PacketNonceGetterComponent,
                PacketTimeoutGetterComponent,
                PacketPayloadsGetterComponent,
                PayloadHeaderGetterComponent,
                PayloadAppIdGetterComponent,
                PayloadDataGetterComponent,
                IbcMessageAppIdGetterComponent,
            ]:
                WithContext,
            IncomingPacketHandlerComponent:
                FullIncomingPacketHandler<AnyApp>,
            PacketSenderComponent:
                CommitSendPacket<AllocateNonceAndBuildPacket>,
        }
    }
}
