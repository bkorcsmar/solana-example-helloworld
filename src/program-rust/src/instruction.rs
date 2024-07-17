use std::convert::TryInto;
use solana_program::{program_error::ProgramError};

#[derive(Debug)]
pub enum HelloInstruction {
    Increment,
    Decrement,
    Set(u32)
}

impl HelloInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        match tag {
            0 => Ok(HelloInstruction::Increment),
            1 => Ok(HelloInstruction::Decrement),
            2 => {
                if rest.len() != 4 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let val: Result<[u8; 4], _> = rest[..4].try_into();
                match val {
                    Ok(i) => Ok(HelloInstruction::Set(u32::from_le_bytes(i))),
                    _ => Err(ProgramError::InvalidInstructionData)
                }
            }
            _ => Err(ProgramError::InvalidInstructionData)
        }
    }
}
