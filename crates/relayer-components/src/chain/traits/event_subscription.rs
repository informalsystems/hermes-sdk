use crate::chain::traits::types::event::HasEventType;
use crate::chain::traits::types::height::HasHeightType;
use crate::runtime::traits::runtime::HasRuntime;
use crate::runtime::traits::subscription::HasSubscriptionType;

pub trait HasEventSubscription: HasHeightType + HasEventType + HasRuntime
where
    Self::Runtime: HasSubscriptionType,
{
    fn event_subscription(
        &self,
    ) -> &<Self::Runtime as HasSubscriptionType>::Subscription<(Self::Height, Self::Event)>;
}
