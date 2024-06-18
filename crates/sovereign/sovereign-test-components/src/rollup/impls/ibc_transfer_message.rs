use cgp_core::prelude::HasErrorType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_sovereign_rollup_components::types::height::RollupHeight;
use hermes_sovereign_rollup_components::types::message::SovereignMessage;
use hermes_test_components::chain::traits::messages::ibc_transfer::IbcTokenTransferMessageBuilder;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::amount::HasAmountType;
use hermes_test_components::chain::traits::types::memo::HasMemoType;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};
use ibc_relayer_types::timestamp::Timestamp;

use crate::types::amount::SovereignAmount;

pub struct BuildSovereignIbcTransferMessage;

impl<Chain, Counterparty> IbcTokenTransferMessageBuilder<Chain, Counterparty>
    for BuildSovereignIbcTransferMessage
where
    Chain: HasErrorType
        + HasAddressType
        + HasAmountType<Amount = SovereignAmount>
        + HasMemoType<Memo = Option<String>>
        + HasIbcChainTypes<
            Counterparty,
            ChannelId = ChannelId,
            PortId = PortId,
            Height = RollupHeight,
            Timestamp = Timestamp,
            Message = SovereignMessage,
        >,
    Counterparty: HasAddressType,
{
    async fn build_ibc_token_transfer_message(
        _chain: &Chain,
        _channel_id: &ChannelId,
        _port_id: &PortId,
        _recipient_address: &Counterparty::Address,
        _amount: &SovereignAmount,
        _memo: &Option<String>,
        _timeout_height: Option<&RollupHeight>,
        _timeout_time: Option<&Timestamp>,
    ) -> Result<SovereignMessage, Chain::Error> {
        todo!()
    }
}
