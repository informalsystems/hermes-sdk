use core::marker::PhantomData;

use cgp_core::prelude::{DelegateComponent, HasErrorType};

use crate::encode::traits::convert::Converter;
use crate::encode::traits::decoder::Decoder;
use crate::encode::traits::encoded::HasEncodedType;
use crate::encode::traits::encoder::Encoder;
use crate::encode::traits::schema::{HasSchemaType, SchemaGetter};

pub struct DelegateEncoding<Delegate>(pub PhantomData<Delegate>);

impl<Encoding, Value, Components, Delegate> Encoder<Encoding, Value>
    for DelegateEncoding<Components>
where
    Encoding: HasEncodedType + HasErrorType,
    Components: DelegateComponent<Value, Delegate = Delegate>,
    Delegate: Encoder<Encoding, Value>,
{
    fn encode(encoding: &Encoding, value: &Value) -> Result<Encoding::Encoded, Encoding::Error> {
        Delegate::encode(encoding, value)
    }
}

impl<Encoding, Value, Components, Delegate> Decoder<Encoding, Value>
    for DelegateEncoding<Components>
where
    Encoding: HasEncodedType + HasErrorType,
    Components: DelegateComponent<Value, Delegate = Delegate>,
    Delegate: Decoder<Encoding, Value>,
{
    fn decode(encoding: &Encoding, encoded: &Encoding::Encoded) -> Result<Value, Encoding::Error> {
        Delegate::decode(encoding, encoded)
    }
}

impl<Encoding, To, From, Components, Delegate> Converter<Encoding, From, To>
    for DelegateEncoding<Components>
where
    Encoding: HasErrorType,
    Components: DelegateComponent<(To, From), Delegate = Delegate>,
    Delegate: Converter<Encoding, From, To>,
{
    fn convert(encoding: &Encoding, from: &From) -> Result<To, Encoding::Error> {
        Delegate::convert(encoding, from)
    }
}

impl<Encoding, Value, Components, Delegate> SchemaGetter<Encoding, Value>
    for DelegateEncoding<Components>
where
    Encoding: HasSchemaType,
    Components: DelegateComponent<Value, Delegate = Delegate>,
    Delegate: SchemaGetter<Encoding, Value>,
{
    fn schema(encoding: &Encoding, phantom: PhantomData<Value>) -> &Encoding::Schema {
        Delegate::schema(encoding, phantom)
    }
}
