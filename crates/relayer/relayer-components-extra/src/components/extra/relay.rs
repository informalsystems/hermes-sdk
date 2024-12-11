pub use cgp::extra::run::RunnerComponent;
use cgp::prelude::*;
pub use hermes_relayer_components::components::default::relay::*;
use hermes_relayer_components::relay::impls::message_senders::chain_sender::SendIbcMessagesToChain;
use hermes_relayer_components::relay::impls::message_senders::update_client::SendIbcMessagesWithUpdateClient;
use hermes_relayer_components::relay::impls::packet_relayers::general::filter_relayer::FilterRelayer;
use hermes_relayer_components::relay::impls::packet_relayers::general::full_relay::FullCycleRelayer;
use hermes_relayer_components::relay::impls::packet_relayers::general::lock::LockPacketRelayer;
use hermes_relayer_components::relay::impls::packet_relayers::general::log::LoggerRelayer;

use crate::batch::impls::message_sender::SendMessagesToBatchWorker;
pub use crate::batch::types::sink::BatchWorkerSink;
use crate::relay::components::packet_relayers::retry::RetryRelayer;

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
                    LockPacketRelayer<LoggerRelayer<FilterRelayer<RetryRelayer<FullCycleRelayer>>>>,
                Components:
                    DefaultRelayPreset,
            }
        }
    }
}
