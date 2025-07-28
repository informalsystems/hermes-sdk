use core::marker::PhantomData;
use core::time::Duration;

use cgp::core::field::UseField;
use hermes_prelude::*;

#[cgp_component {
  name: RefreshRateAtoBGetterComponent,
  provider: RefreshRateAtoBGetter,
}]
pub trait HasRefreshRateAToB {
    fn refresh_rate_a_to_b(&self) -> &Option<Duration>;
}

#[cgp_provider(RefreshRateAtoBGetterComponent)]
impl<Context, FieldTag> RefreshRateAtoBGetter<Context> for UseField<FieldTag>
where
    Context: HasField<symbol!("refresh_rate_a_to_b"), Value = Option<Duration>>,
{
    fn refresh_rate_a_to_b(context: &Context) -> &Option<Duration> {
        context.get_field(PhantomData::<symbol!("refresh_rate_a_to_b")>)
    }
}

#[cgp_component {
  name: RefreshRateBtoAGetterComponent,
  provider: RefreshRateBtoAGetter,
}]
pub trait HasRefreshRateBtoA {
    fn refresh_rate_b_to_a(&self) -> &Option<Duration>;
}

#[cgp_provider(RefreshRateBtoAGetterComponent)]
impl<Context, FieldTag> RefreshRateBtoAGetter<Context> for UseField<FieldTag>
where
    Context: HasField<symbol!("refresh_rate_b_to_a"), Value = Option<Duration>>,
{
    fn refresh_rate_b_to_a(context: &Context) -> &Option<Duration> {
        context.get_field(PhantomData::<symbol!("refresh_rate_b_to_a")>)
    }
}
