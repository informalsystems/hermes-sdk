use cgp::prelude::*;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::subscription::HasSubscription;

use crate::traits::types::event::HasEventType;
use crate::traits::types::height::HasHeightType;

#[cgp_component {
    provider: EventSubscriptionGetter,
    context: Chain,
}]
pub trait HasEventSubscription: HasHeightType + HasEventType + HasRuntime
where
    Self::Runtime: HasSubscription,
{
    fn event_subscription(
        &self,
    ) -> Option<&<Self::Runtime as HasSubscription>::Subscription<(Self::Height, Self::Event)>>;
}
