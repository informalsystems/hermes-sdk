use core::marker::PhantomData;

use cgp::core::component::WithProvider;
use cgp::core::field::FieldGetter;
use cgp::prelude::*;

use crate::traits::types::app_id::HasAppIdType;
use crate::traits::types::message_header::HasIbcMessageHeaderType;

#[cgp_component {
  provider: IbcMessageAppIdGetter,
  context: Chain,
}]
pub trait HasIbcMessageAppIds<Counterparty>:
    HasIbcMessageHeaderType<Counterparty> + HasAppIdType<Counterparty>
where
    Counterparty: HasAppIdType<Self>,
{
    fn ibc_message_src_app_id(message_header: &Self::IbcMessageHeader) -> &Self::AppId;

    fn ibc_message_dst_app_id(message_header: &Self::IbcMessageHeader) -> &Counterparty::AppId;
}

#[cgp_provider(IbcMessageAppIdGetterComponent)]
impl<Chain, Counterparty, Provider> IbcMessageAppIdGetter<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: HasIbcMessageHeaderType<Counterparty> + HasAppIdType<Counterparty>,
    Counterparty: HasAppIdType<Chain>,
    Provider: FieldGetter<Chain::IbcMessageHeader, symbol!("src_app_id"), Value = Chain::AppId>
        + FieldGetter<Chain::IbcMessageHeader, symbol!("dst_app_id"), Value = Counterparty::AppId>,
{
    fn ibc_message_src_app_id(packet_header: &Chain::IbcMessageHeader) -> &Chain::AppId {
        Provider::get_field(packet_header, PhantomData::<symbol!("src_app_id")>)
    }

    fn ibc_message_dst_app_id(packet_header: &Chain::IbcMessageHeader) -> &Counterparty::AppId {
        Provider::get_field(packet_header, PhantomData::<symbol!("dst_app_id")>)
    }
}
