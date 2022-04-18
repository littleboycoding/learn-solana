use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
    system_instruction,
};

pub fn instruction_processor(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_info_iter = &mut _accounts.iter();

    let from = next_account_info(accounts_info_iter)?;
    let to = next_account_info(accounts_info_iter)?;

    let amount = from.lamports();

    let instruction = system_instruction::transfer(from.key, to.key, amount);

    invoke(&instruction, _accounts)?;

    Ok(())
}
