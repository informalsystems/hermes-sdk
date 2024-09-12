use prost::encoding::WireType;

pub struct ProtoChunk<'a> {
    pub tag: u32,
    pub wire_type: WireType,
    pub chunk: &'a [u8],
}
