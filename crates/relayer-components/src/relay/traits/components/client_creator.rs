use cgp_core::prelude::*;

use crate::chain::traits::client::create::HasCreateClientOptions;
use crate::chain::types::aliases::ClientId;
use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::target::ChainTarget;
use crate::std_prelude::*;

#[derive_component(ClientCreatorComponent, ClientCreator<Relay>)]
#[async_trait]
pub trait CanCreateClient<Target>: HasRelayChains
where
    Target: ChainTarget<Self>,
    Target::CounterpartyChain: HasCreateClientOptions<Target::TargetChain>,
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
        target_chain: &Target::TargetChain,
        counterparty_chain: &Target::CounterpartyChain,
        create_client_options: &<Target::CounterpartyChain as HasCreateClientOptions<
            Target::TargetChain,
        >>::CreateClientPayloadOptions,
    ) -> Result<ClientId<Target::TargetChain, Target::CounterpartyChain>, Self::Error>;
}
