use borsh::BorshSerialize;

use crate::sovereign::types::address::SovereignAddressBytes;

pub struct BankMessage(pub BankFields);

#[derive(BorshSerialize)]
pub enum BankFields {
    CreateToken {},
    Transfer {
        to: SovereignAddressBytes,
        coins: CoinFields,
    },
}

#[derive(BorshSerialize)]
pub struct CoinFields {
    pub amount: u64,
    pub token_address: SovereignAddressBytes,
}

impl BorshSerialize for BankMessage {
    fn serialize<W: borsh::maybestd::io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), borsh::maybestd::io::Error> {
        writer.write_all(&1u8.to_le_bytes())?;
        borsh::BorshSerialize::serialize(&self.0, writer)?;
        Ok(())
    }
}
