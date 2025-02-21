use core::marker::PhantomData;
use core::time::Duration;

use cgp::prelude::*;

#[cgp_component {
    provider: PollIntervalGetter,
    context: Chain,
  }]
pub trait HasPollInterval {
    fn poll_interval(&self) -> Duration;
}

#[cgp_provider(PollIntervalGetterComponent)]
impl<Context, Tag> PollIntervalGetter<Context> for UseField<Tag>
where
    Context: HasField<Tag, Value = Duration>,
{
    fn poll_interval(context: &Context) -> Duration {
        *context.get_field(PhantomData)
    }
}
