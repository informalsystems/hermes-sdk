use borsh::BorshSerialize;
use ibc_proto_new::google::protobuf::Any;

#[derive(BorshSerialize)]
pub enum IbcMessage {
    Core(Any),
}
