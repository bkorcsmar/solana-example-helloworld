use std::convert::TryInto;
use solana_program::{program_error::ProgramError,
                     msg};

#[derive(Debug)]
pub enum TransferInstruction {
    Transfer(u64),
}

impl TransferInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        match tag {
            2 => {
                if rest.len() != 8 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let val: Result<[u8; 8], _> = rest.try_into();
                match val {
                    Ok(i) => {
                        let number = u64::from_le_bytes(i);
                        Ok(TransferInstruction::Transfer(number))
                    },
                    _ => Err(ProgramError::InvalidInstructionData)
                }
            }
            _ => Err(ProgramError::InvalidInstructionData)
        }
    }
}
