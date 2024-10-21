use cgp::prelude::HasErrorType;

use crate::traits::types::payload::data::HasPayloadDataType;
use crate::traits::types::payload::header::HasPayloadHeaderType;
use crate::traits::types::payload::payload::HasPayloadType;

pub trait CanBuildPayload<Counterparty, App>:
    HasPayloadType<Counterparty>
    + HasPayloadHeaderType<Counterparty>
    + HasPayloadDataType<Counterparty, App>
    + HasErrorType
{
    fn build_payload(
        header: Self::PayloadHeader,
        data: Self::PayloadData,
    ) -> Result<Self::Payload, Self::Error>;
}
