use cgp_core::traits::Async;

pub trait HasTelemetry {
    type Telemetry: Async;

    fn telemetry(&self) -> &Self::Telemetry;
}
