use cgp::prelude::*;

#[cgp_component {
  provider: Converter,
  context: Encoding,
}]
pub trait CanConvert<From, To>: HasAsyncErrorType {
    fn convert(&self, from: &From) -> Result<To, Self::Error>;
}

pub trait CanConvertBothWays<A, B>: CanConvert<A, B> + CanConvert<B, A> {}

impl<Encoding, A, B> CanConvertBothWays<A, B> for Encoding where
    Encoding: CanConvert<A, B> + CanConvert<B, A>
{
}
