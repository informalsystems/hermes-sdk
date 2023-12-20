use crate::chain::traits::types::event::HasEventType;
use crate::chain::traits::types::height::HasHeightType;
use crate::runtime::traits::runtime::HasRuntime;
use crate::runtime::traits::subscription::HasSubscription;

pub trait HasEventSubscription: HasHeightType + HasEventType + HasRuntime
where
    Self::Runtime: HasSubscription,
{
    fn event_subscription(
        &self,
    ) -> &<Self::Runtime as HasSubscription>::Subscription<(Self::Height, Self::Event)>;
}
