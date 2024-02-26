pub trait HasDataChain {
    type DataChain;

    fn data_chain(&self) -> &Self::DataChain;
}
