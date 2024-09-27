use cgp::prelude::*;

#[derive_component(TimeTypeComponent, ProvideTimeType<Chain>)]
pub trait HasTimeType: Async {
    type Time: Async;
}
