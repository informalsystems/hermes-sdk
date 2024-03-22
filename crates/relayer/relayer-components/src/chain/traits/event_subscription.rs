use crate::chain::traits::types::event::HasEventType;
use crate::chain::traits::types::height::HasHeightType;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::subscription::HasSubscription;

pub trait HasEventSubscription: HasHeightType + HasEventType + HasRuntime
where
    Self::Runtime: HasSubscription,
{
    fn event_subscription(
        &self,
    ) -> &<Self::Runtime as HasSubscription>::Subscription<(Self::Height, Self::Event)>;
}
