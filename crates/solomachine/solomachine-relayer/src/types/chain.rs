use crate::traits::solomachine::Solomachine;

#[derive(Clone)]
pub struct SolomachineChain<Chain> {
    pub chain: Chain,
}

impl<Chain: Solomachine> SolomachineChain<Chain> {
    pub fn new(chain: Chain) -> Self {
        SolomachineChain { chain }
    }
}
