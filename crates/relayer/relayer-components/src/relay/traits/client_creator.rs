use cgp::prelude::*;
use hermes_chain_components::traits::HasClientIdType;

use crate::chain::traits::{
    CreateClientMessageOptionsOf, CreateClientPayloadOptionsOf, HasCreateClientMessageOptionsType,
    HasCreateClientPayloadOptionsType,
};
use crate::chain::types::aliases::ClientIdOf;
use crate::relay::traits::{HasTargetChainTypes, RelayTarget};

#[cgp_component {
  provider: ClientCreator,
  context: Relay,
}]
#[async_trait]
pub trait CanCreateClient<Target: RelayTarget>:
    HasTargetChainTypes<
        Target,
        TargetChain: HasClientIdType<Self::CounterpartyChain>
                         + HasCreateClientMessageOptionsType<Self::CounterpartyChain>,
        CounterpartyChain: HasCreateClientPayloadOptionsType<Self::TargetChain>,
    > + HasAsyncErrorType
{
    /**
       Create a new IBC client on the target chain.

       Notice that this function does not take in `&self` as argument.
       This is because the relay context is required to have fixed client IDs already.
       Since the relay context can't be built yet without the client IDs,
       we pass in the target and counterparty chains as argument directly.

       We define this as a static method for the relay context to reuse the
       existing infrastructure, particularly in handling errors from two chains
       which may be of different types.
    */
    async fn create_client(
        target: Target,
        target_chain: &Self::TargetChain,
        counterparty_chain: &Self::CounterpartyChain,
        create_client_payload_options: &CreateClientPayloadOptionsOf<
            Self::CounterpartyChain,
            Self::TargetChain,
        >,
        create_client_message_options: &CreateClientMessageOptionsOf<
            Self::TargetChain,
            Self::CounterpartyChain,
        >,
    ) -> Result<ClientIdOf<Self::TargetChain, Self::CounterpartyChain>, Self::Error>;
}
