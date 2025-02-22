#[cgp::re_export_imports]
mod preset {
    use cgp::extra::run::RunnerComponent;
    use cgp::prelude::*;
    use hermes_relayer_components::components::default::relay::re_exports::*;
    use hermes_relayer_components::components::default::relay::{
        with_default_relay_preset, DefaultRelayPreset,
    };
    use hermes_relayer_components::relay::impls::message_senders::chain_sender::SendIbcMessagesToChain;
    use hermes_relayer_components::relay::impls::message_senders::update_client::SendIbcMessagesWithUpdateClient;

    use crate::batch::impls::message_sender::SendMessagesToBatchWorker;
    use crate::batch::types::sink::BatchWorkerSink;
    use crate::relay::impls::packet_relayers::extra::ExtraPacketRelayer;

    with_default_relay_preset! {
        [
            IbcMessageSenderComponent<MainSink>,
            PacketRelayerComponent,
        ],
        | Components | {
            cgp_preset! {
                ExtraRelayPreset {
                    IbcMessageSenderComponent<MainSink>: SendMessagesToBatchWorker,
                    IbcMessageSenderComponent<BatchWorkerSink>:
                        SendIbcMessagesWithUpdateClient<SendIbcMessagesToChain>,
                    PacketRelayerComponent:
                        ExtraPacketRelayer,
                    Components:
                        DefaultRelayPreset,
                }
            }
        }
    }
}
