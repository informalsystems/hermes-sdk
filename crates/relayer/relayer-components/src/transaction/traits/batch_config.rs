use hermes_chain_type_components::impls::BatchConfig;
use hermes_prelude::*;

#[cgp_component {
  provider: BatchConfigGetter,
  context: Chain,
}]
pub trait HasBatchConfig {
    fn batch_config(&self) -> BatchConfig;
}
