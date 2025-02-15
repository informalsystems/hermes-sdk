pub use cgp::extra::run::RunnerComponent;
use cgp::prelude::*;
pub use hermes_relayer_components::components::default::relay::*;
use hermes_relayer_components::relay::impls::message_senders::chain_sender::SendIbcMessagesToChain;
use hermes_relayer_components::relay::impls::message_senders::update_client::SendIbcMessagesWithUpdateClient;

use crate::batch::impls::message_sender::SendMessagesToBatchWorker;
pub use crate::batch::types::sink::BatchWorkerSink;
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
                Components:
                    DefaultRelayPreset::Provider,
            }
        }
    }
}
