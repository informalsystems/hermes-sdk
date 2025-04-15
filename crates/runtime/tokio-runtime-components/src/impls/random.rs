use cgp::prelude::*;
use hermes_runtime_components::traits::{RandomGenerator, RandomGeneratorComponent};
use rand::distributions::uniform::SampleUniform;
use rand::distributions::Standard;
use rand::prelude::*;

pub struct ThreadRandomGenerator;

#[cgp_provider(RandomGeneratorComponent)]
impl<Runtime, T> RandomGenerator<Runtime, T> for ThreadRandomGenerator
where
    Runtime: Async,
    Standard: Distribution<T>,
    T: Async + SampleUniform + PartialOrd,
{
    async fn generate_random(_runtime: &Runtime) -> T {
        let mut rng = thread_rng();
        rng.gen()
    }

    async fn random_range(_runtime: &Runtime, min: T, max: T) -> T {
        let mut rng = thread_rng();
        rng.gen_range(min..max)
    }
}
