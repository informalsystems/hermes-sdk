use cgp_core::prelude::HasErrorType;
use hermes_cosmos_client_components::traits::message::{CosmosMessage, ToCosmosMessage};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_test_components::chain::traits::messages::ibc_transfer::IbcTokenTransferMessageBuilder;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::amount::HasAmountType;
use hermes_test_components::chain::traits::types::memo::HasMemoType;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};
use ibc_relayer_types::timestamp::Timestamp;
use ibc_relayer_types::Height;

use crate::chain::types::amount::Amount;
use crate::chain::types::messages::token_transfer::TokenTransferMessage;

pub struct BuildCosmosIbcTransferMessage;

impl<Chain, Counterparty> IbcTokenTransferMessageBuilder<Chain, Counterparty>
    for BuildCosmosIbcTransferMessage
where
    Chain: HasErrorType
        + HasAddressType
        + HasAmountType<Amount = Amount>
        + HasMemoType<Memo = Option<String>>
        + HasIbcChainTypes<
            Counterparty,
            ChannelId = ChannelId,
            PortId = PortId,
            Height = Height,
            Timestamp = Timestamp,
            Message = CosmosMessage,
        >,
    Counterparty: HasAddressType,
{
    async fn build_ibc_token_transfer_message(
        _chain: &Chain,
        channel_id: &ChannelId,
        port_id: &PortId,
        recipient_address: &Counterparty::Address,
        amount: &Amount,
        memo: &Option<String>,
        timeout_height: Option<&Height>,
        timeout_time: Option<&Timestamp>,
    ) -> Result<CosmosMessage, Chain::Error> {
        let message = TokenTransferMessage {
            channel_id: channel_id.clone(),
            port_id: port_id.clone(),
            recipient_address: recipient_address.to_string(),
            amount: amount.clone(),
            memo: memo.clone(),
            timeout_height: timeout_height.cloned(),
            timeout_time: timeout_time.cloned(),
        };

        Ok(message.to_cosmos_message())
    }
}
