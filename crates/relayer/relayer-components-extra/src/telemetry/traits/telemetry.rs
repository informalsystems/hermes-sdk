use cgp::core::Async;

pub trait HasTelemetry {
    type Telemetry: Async;

    fn telemetry(&self) -> &Self::Telemetry;
}
