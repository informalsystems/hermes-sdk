use alloc::vec::Vec;
use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::{HasChannelIdType, HasPortIdType};
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::chain::traits::types::timestamp::HasTimeoutType;

use crate::chain::traits::types::address::HasAddressType;
use crate::chain::traits::types::amount::HasAmountType;
use crate::chain::traits::types::memo::HasMemoType;

#[cgp_component {
  provider: IbcTokenTransferMessageBuilder,
  context: ChainDriver,
}]
#[async_trait]
pub trait CanBuildIbcTokenTransferMessage<Counterparty>:
    HasAsyncErrorType
    + HasAmountType
    + HasMemoType
    + HasMessageType
    + HasHeightType
    + HasTimeoutType
    + HasChannelIdType<Counterparty>
    + HasPortIdType<Counterparty>
where
    Counterparty: HasAddressType,
{
    async fn build_ibc_token_transfer_message(
        &self,
        _counterparty: PhantomData<Counterparty>,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
        recipient_address: &Counterparty::Address,
        amount: &Self::Amount,
        memo: &Self::Memo,
        timeout_height: Option<&Self::Height>,
        timeout_time: Option<&Self::Timeout>,
    ) -> Result<Vec<Self::Message>, Self::Error>;
}
