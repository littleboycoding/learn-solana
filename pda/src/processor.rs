use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    native_token::sol_to_lamports,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};

use crate::state::AccountData;

enum ProgramInstruction {
    Initialize(Vec<u8>, u8),
    Increment,
    Decrement,
}

impl ProgramInstruction {
    fn unpack(instruction_data: &[u8]) -> Result<Self, ProgramError> {
        let payload = Payload::try_from_slice(instruction_data)?;

        match payload.variant {
            0 => {
                let (seed, bumps) = (payload.seed.unwrap(), payload.bumps.unwrap());
                Ok(Self::Initialize(seed, bumps))
            }
            1 => Ok(Self::Increment),
            2 => Ok(Self::Decrement),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Payload {
    pub variant: u8,
    pub seed: Option<Vec<u8>>,
    pub bumps: Option<u8>,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = ProgramInstruction::unpack(instruction_data)?;

    match instruction {
        ProgramInstruction::Initialize(seed, bumps) => {
            let accounts_info_iter = &mut accounts.iter();

            let payer_account = next_account_info(accounts_info_iter)?;
            let pda_account = next_account_info(accounts_info_iter)?;

            let instruction = system_instruction::create_account(
                &payer_account.key,
                &pda_account.key,
                sol_to_lamports(0.00094656),
                8,
                program_id,
            );

            invoke_signed(
                &instruction,
                &[payer_account.clone(), pda_account.clone()],
                &[&[seed.as_slice(), &[bumps]]],
            )?;

            Ok(())
        }
        ProgramInstruction::Increment => {
            let accounts_info_iter = &mut accounts.iter();
            let pda_account = next_account_info(accounts_info_iter)?;

            let mut data = AccountData::try_from_slice(*pda_account.data.borrow())?;
            data.inc();

            data.serialize(&mut *pda_account.data.borrow_mut())?;
            Ok(())
        }
        ProgramInstruction::Decrement => {
            let accounts_info_iter = &mut accounts.iter();
            let pda_account = next_account_info(accounts_info_iter)?;

            let mut data = AccountData::try_from_slice(*pda_account.data.borrow())?;
            data.dec();

            data.serialize(&mut *pda_account.data.borrow_mut())?;
            Ok(())
        }
    }
}
