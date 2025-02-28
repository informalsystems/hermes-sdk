#[cgp::re_export_imports]
mod preset {
    use cgp::extra::run::RunnerComponent;
    use cgp::prelude::*;
    use hermes_relayer_components::components::default::relay::DefaultRelayPreset;
    use hermes_relayer_components::error::impls::retry::PerformRetryWithRetryableError;
    use hermes_relayer_components::error::traits::RetryPerformerComponent;
    use hermes_relayer_components::relay::impls::message_senders::chain_sender::SendIbcMessagesToChain;
    use hermes_relayer_components::relay::impls::message_senders::update_client::SendIbcMessagesWithUpdateClient;
    use DefaultRelayPreset::re_exports::*;

    use crate::batch::impls::message_sender::SendMessagesToBatchWorker;
    use crate::batch::types::sink::BatchWorkerSink;
    use crate::relay::impls::packet_relayers::extra::ExtraPacketRelayer;

    DefaultRelayPreset::with_components! {
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
                    RetryPerformerComponent:
                        PerformRetryWithRetryableError,
                    Components:
                        DefaultRelayPreset::Provider,
                }
            }
        }
    }
}
