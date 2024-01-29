use cgp_core::{CanRaiseError, HasInner};

use crate::chain::traits::types::create_client::{CreateClientOptions, HasCreateClientOptionsType};
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::types::aliases::ClientIdOf;
use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::components::client_creator::{CanCreateClient, ClientCreator};
use crate::relay::traits::target::ChainTarget;

pub struct ForwardCreateClient;

impl<Relay, Target, Inner, TargetChain, CounterpartyChain> ClientCreator<Relay, Target>
    for ForwardCreateClient
where
    Relay: HasInner<Inner = Inner> + HasRelayChains + CanRaiseError<Inner::Error>,
    Target: ChainTarget<Relay, TargetChain = TargetChain, CounterpartyChain = CounterpartyChain>
        + ChainTarget<Inner, TargetChain = TargetChain, CounterpartyChain = CounterpartyChain>,
    CounterpartyChain: HasCreateClientOptionsType<TargetChain>,
    TargetChain: HasIbcChainTypes<CounterpartyChain>,
    Inner: CanCreateClient<Target>,
{
    async fn create_client(
        target: Target,
        target_chain: &TargetChain,
        counterparty_chain: &CounterpartyChain,
        create_client_options: &CreateClientOptions<CounterpartyChain, TargetChain>,
    ) -> Result<ClientIdOf<TargetChain, CounterpartyChain>, Relay::Error> {
        Inner::create_client(
            target,
            target_chain,
            counterparty_chain,
            create_client_options,
        )
        .await
        .map_err(Relay::raise_error)
    }
}
