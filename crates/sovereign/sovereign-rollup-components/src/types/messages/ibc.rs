use borsh::BorshSerialize;
use ibc_proto::google::protobuf::Any;

#[derive(BorshSerialize)]
pub enum IbcMessage {
    Core(Any),
}
