use alloc::vec::Vec;
use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::{HasChannelIdType, HasPortIdType};
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::chain::traits::types::timestamp::HasTimeoutType;

use crate::chain::traits::types::address::HasAddressType;
use crate::chain::traits::types::memo::HasMemoType;

#[cgp_component {
  provider: IbcTokenTransferMessageBuilder,
  context: ChainDriver,
}]
#[async_trait]
pub trait CanBuildIbcTokenTransferMessages<Counterparty>:
    HasAsyncErrorType
    + HasAmountType
    + HasMemoType
    + HasMessageType
    + HasChannelIdType<Counterparty>
    + HasPortIdType<Counterparty>
where
    Counterparty: HasAddressType + HasHeightType + HasTimeoutType,
{
    async fn build_ibc_token_transfer_messages(
        &self,
        _counterparty: PhantomData<Counterparty>,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
        recipient_address: &Counterparty::Address,
        amount: &Self::Amount,
        memo: &Self::Memo,
        timeout_height: Option<&Counterparty::Height>,
        timeout_time: Option<&Counterparty::Timeout>,
    ) -> Result<Vec<Self::Message>, Self::Error>;
}
