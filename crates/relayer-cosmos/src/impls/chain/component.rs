use cgp_core::{delegate_component, Async, HasComponents};
use ibc_relayer_components::chain::traits::components::chain_status_querier::ChainStatusQuerierComponent;
use ibc_relayer_components::chain::traits::components::message_sender::MessageSenderComponent;
use ibc_relayer_components_extra::components::extra::chain::ExtraChainComponents;

use crate::contexts::chain::CosmosChain;
use crate::impls::chain::components::query_chain_status::QueryChainStatusWithChainHandle;
use crate::impls::chain::components::send_messages_as_tx::SendMessagesToTxContext;

pub struct CosmosChainComponents;

impl<Chain> HasComponents for CosmosChain<Chain>
where
    Chain: Async,
{
    type Components = ExtraChainComponents<CosmosChainComponents>;
}

delegate_component!(
    MessageSenderComponent,
    CosmosChainComponents,
    SendMessagesToTxContext,
);

delegate_component!(
    ChainStatusQuerierComponent,
    CosmosChainComponents,
    QueryChainStatusWithChainHandle,
);
