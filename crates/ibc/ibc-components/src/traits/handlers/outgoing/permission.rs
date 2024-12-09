use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;

use crate::traits::types::payload::header::HasPayloadHeaderType;

/**
   Used to check whether the sender has permission to send a given payload.

   This can be used when an application smart contract calls the core contract
   to send a packet, to ensure that the source application ID matches the calling
   contract.
*/
#[cgp_component {
  provider: SendPayloadPermissionChecker,
  context: Chain,
}]
#[async_trait]
pub trait CanCheckSendPayloadPermission<Counterparty>:
    HasErrorType + HasAddressType + HasPayloadHeaderType<Counterparty>
{
    async fn check_send_payload_permission(
        &self,
        sender: &Self::Address,
        payload_header: &Self::PayloadHeader,
    ) -> Result<(), Self::Error>;
}
