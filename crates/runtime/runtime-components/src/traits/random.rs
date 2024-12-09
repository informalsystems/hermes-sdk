use cgp::prelude::*;

#[cgp_component {
  provider: RandomGenerator,
  context: Runtime,
}]
#[async_trait]
pub trait CanGenerateRandom<T: Async>: Async {
    async fn generate_random(&self) -> T;

    async fn random_range(&self, min: T, max: T) -> T;
}
