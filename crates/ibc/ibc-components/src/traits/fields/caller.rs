use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;

/**
   A global-level caller associated with the abstract chain context.

   This may the signer of a transaction, or a contract address when
   called from another smart contract.

   In case this is running on an application contract that is
   called from an IBC core contract, the chain context should
   have the caller set to the original caller forwarded from
   the core contract.
*/
#[cgp_component {
  provider: CallerGetter,
  context: Chain,
}]
pub trait HasCaller: HasAddressType {
    fn caller(&self) -> Self::Address;
}
