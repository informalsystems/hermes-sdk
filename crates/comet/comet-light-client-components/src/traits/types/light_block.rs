use hermes_prelude::*;

#[cgp_component {
  name: LightBlockTypeComponent,
  provider: ProvideLightBlockType,
  context: Client,
}]
pub trait HasLightBlockType: Async {
    type LightBlock: Async;
}
