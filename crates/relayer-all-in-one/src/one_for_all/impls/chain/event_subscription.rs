use ibc_relayer_components::chain::traits::event_subscription::HasEventSubscription;
use ibc_relayer_components::runtime::traits::subscription::HasSubscriptionType;

use crate::one_for_all::traits::chain::OfaChain;
use crate::one_for_all::types::chain::OfaChainWrapper;

impl<Chain> HasEventSubscription for OfaChainWrapper<Chain>
where
    Chain: OfaChain,
{
    fn event_subscription(
        &self,
    ) -> &<Self::Runtime as HasSubscriptionType>::Subscription<(Self::Height, Self::Event)> {
        self.chain.event_subscription()
    }
}
