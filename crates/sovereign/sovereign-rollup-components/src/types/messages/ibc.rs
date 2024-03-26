use borsh::BorshSerialize;
use ibc_proto_sov::google::protobuf::Any;

#[derive(BorshSerialize)]
pub enum IbcMessage {
    Core(Any),
}
