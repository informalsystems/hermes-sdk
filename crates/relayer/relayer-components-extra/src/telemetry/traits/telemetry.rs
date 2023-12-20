use cgp_core::Async;

pub trait HasTelemetry {
    type Telemetry: Async;

    fn telemetry(&self) -> &Self::Telemetry;
}
