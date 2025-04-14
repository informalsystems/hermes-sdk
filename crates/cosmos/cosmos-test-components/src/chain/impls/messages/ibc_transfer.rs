use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::traits::{HasAddressType, HasAmountType};
use hermes_cosmos_chain_components::traits::message::{CosmosMessage, ToCosmosMessage};
use hermes_relayer_components::chain::traits::{
    HasChannelIdType, HasHeightFields, HasMessageType, HasPortIdType, HasTimeoutType,
};
use hermes_test_components::chain::traits::messages::ibc_transfer::{
    IbcTokenTransferMessageBuilder, IbcTokenTransferMessageBuilderComponent,
};
use hermes_test_components::chain::traits::types::memo::HasMemoType;
use ibc::core::client::types::error::ClientError;
use ibc::core::client::types::Height;
use ibc::core::host::types::identifiers::{ChannelId, PortId};
use ibc::primitives::Timestamp;

use crate::chain::types::amount::Amount;
use crate::chain::types::messages::token_transfer::TokenTransferMessage;

#[cgp_new_provider(IbcTokenTransferMessageBuilderComponent)]
impl<Chain, Counterparty> IbcTokenTransferMessageBuilder<Chain, Counterparty>
    for BuildCosmosIbcTransferMessage
where
    Chain: HasAddressType
        + HasMessageType
        + HasAmountType<Amount = Amount>
        + HasMemoType<Memo = Option<String>>
        + HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasPortIdType<Counterparty, PortId = PortId>
        + CanRaiseAsyncError<ClientError>,
    Counterparty: HasAddressType + HasHeightFields + HasTimeoutType<Timeout = Timestamp>,
    Chain::Message: From<CosmosMessage>,
{
    async fn build_ibc_token_transfer_messages(
        _chain: &Chain,
        _counterparty: PhantomData<Counterparty>,
        channel_id: &ChannelId,
        port_id: &PortId,
        recipient_address: &Counterparty::Address,
        amount: &Amount,
        memo: &Option<String>,
        timeout_height: Option<&Counterparty::Height>,
        timeout_time: Option<&Timestamp>,
    ) -> Result<Vec<Chain::Message>, Chain::Error> {
        let timeout_height = match timeout_height {
            Some(height) => Some(
                Height::new(
                    Counterparty::revision_number(height),
                    Counterparty::revision_height(height),
                )
                .map_err(Chain::raise_error)?,
            ),
            None => None,
        };

        let message = TokenTransferMessage {
            channel_id: channel_id.clone(),
            port_id: port_id.clone(),
            recipient_address: recipient_address.to_string(),
            amount: amount.clone(),
            memo: memo.clone(),
            timeout_height,
            timeout_time: timeout_time.cloned(),
        };

        let messages = vec![message.to_cosmos_message().into()];

        Ok(messages)
    }
}
