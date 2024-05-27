use cgp_core::HasErrorType;
use hermes_cosmos_chain_components::methods::encode::encode_to_any;
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::ChannelOpenInitMessageBuilder;
use hermes_relayer_components::chain::traits::types::channel::HasInitChannelOptionsType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc::core::channel::types::channel::State;
use ibc_proto::ibc::core::channel::v1::{
    Channel, Counterparty as ChannelCounterparty, MsgChannelOpenInit,
};
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};
use ibc_relayer_types::signer::Signer;

use crate::types::message::SovereignMessage;
use crate::types::messages::ibc::IbcMessageWithHeight;
use crate::types::payloads::channel::SovereignInitChannelOptions;

pub struct BuildChannelOpenInitMessageOnSovereign;

impl<Rollup, Counterparty> ChannelOpenInitMessageBuilder<Rollup, Counterparty>
    for BuildChannelOpenInitMessageOnSovereign
where
    Rollup: HasInitChannelOptionsType<Counterparty, InitChannelOptions = SovereignInitChannelOptions>
        + HasIbcChainTypes<
            Counterparty,
            Message = SovereignMessage,
            ChannelId = ChannelId,
            PortId = PortId,
        > + HasErrorType,
    Counterparty: HasIbcChainTypes<Rollup, ChannelId = ChannelId, PortId = PortId>,
{
    async fn build_channel_open_init_message(
        _rollup: &Rollup,
        port_id: &Rollup::PortId,
        counterparty_port_id: &Counterparty::PortId,
        init_channel_options: &SovereignInitChannelOptions,
    ) -> Result<SovereignMessage, Rollup::Error> {
        let port_id = port_id.clone();
        let ordering = init_channel_options.ordering;

        let connection_hops = init_channel_options
            .connection_hops
            .iter()
            .map(ToString::to_string)
            .collect();

        let channel_version = init_channel_options.channel_version.to_string();

        let channel = Channel {
            state: State::Init as i32,
            ordering: ordering as i32,
            counterparty: Some(ChannelCounterparty {
                port_id: counterparty_port_id.to_string(),
                channel_id: "".to_string(),
            }),
            connection_hops,
            version: channel_version,
            upgrade_sequence: 0,
        };

        let proto_message = MsgChannelOpenInit {
            port_id: port_id.to_string(),
            channel: channel.into(),
            signer: Signer::dummy().to_string(),
        };

        let any_message = encode_to_any("/ibc.core.channel.v1.MsgChannelOpenInit", &proto_message);

        Ok(IbcMessageWithHeight::new(any_message).into())
    }
}
