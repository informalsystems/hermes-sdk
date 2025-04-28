use hermes_core::encoding_components::traits::{Converter, ConverterComponent};
use hermes_prelude::*;
use ibc::primitives::proto::Any as IbcAny;
use prost_types::Any as ProstAny;

pub struct ConvertIbcAny;

#[cgp_provider(ConverterComponent)]
impl<Encoding> Converter<Encoding, ProstAny, IbcAny> for ConvertIbcAny
where
    Encoding: HasAsyncErrorType,
{
    fn convert(_encoding: &Encoding, from: &ProstAny) -> Result<IbcAny, Encoding::Error> {
        Ok(IbcAny {
            type_url: from.type_url.clone(),
            value: from.value.clone(),
        })
    }
}

#[cgp_provider(ConverterComponent)]
impl<Encoding> Converter<Encoding, IbcAny, ProstAny> for ConvertIbcAny
where
    Encoding: HasAsyncErrorType,
{
    fn convert(_encoding: &Encoding, from: &IbcAny) -> Result<ProstAny, Encoding::Error> {
        Ok(ProstAny {
            type_url: from.type_url.clone(),
            value: from.value.clone(),
        })
    }
}
