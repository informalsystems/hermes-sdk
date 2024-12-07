use core::marker::PhantomData;

use cgp::core::component::WithProvider;
use cgp::core::field::FieldGetter;
use cgp::prelude::*;

use crate::traits::types::app_id::HasAppIdType;
use crate::traits::types::payload::header::HasPayloadHeaderType;

#[cgp_component {
  name: PayloadAppIdGetterComponent,
  provider: PayloadAppIdGetter,
  context: Chain,
}]
pub trait HasPayloadAppIds<Counterparty>:
    HasPayloadHeaderType<Counterparty> + HasAppIdType<Counterparty>
where
    Counterparty: HasAppIdType<Self>,
{
    fn payload_src_app_id(payload_header: &Self::PayloadHeader) -> &Self::AppId;

    fn payload_dst_app_id(payload_header: &Self::PayloadHeader) -> &Counterparty::AppId;
}

impl<Chain, Counterparty, Provider> PayloadAppIdGetter<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: HasPayloadHeaderType<Counterparty> + HasAppIdType<Counterparty>,
    Counterparty: HasAppIdType<Chain>,
    Provider: FieldGetter<Chain::PayloadHeader, symbol!("src_app_id"), Value = Chain::AppId>
        + FieldGetter<Chain::PayloadHeader, symbol!("dst_app_id"), Value = Counterparty::AppId>,
{
    fn payload_src_app_id(packet_header: &Chain::PayloadHeader) -> &Chain::AppId {
        Provider::get_field(packet_header, PhantomData::<symbol!("src_app_id")>)
    }

    fn payload_dst_app_id(packet_header: &Chain::PayloadHeader) -> &Counterparty::AppId {
        Provider::get_field(packet_header, PhantomData::<symbol!("dst_app_id")>)
    }
}
