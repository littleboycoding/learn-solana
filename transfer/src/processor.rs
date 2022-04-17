use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    native_token::sol_to_lamports,
    pubkey::Pubkey,
};

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_info_iter = &mut accounts.iter();

    let from = next_account_info(accounts_info_iter)?;
    let to = next_account_info(accounts_info_iter)?;

    let amount = sol_to_lamports(1.0);

    **from.lamports.borrow_mut() = from.lamports().checked_sub(amount).unwrap();
    **to.lamports.borrow_mut() = to.lamports().checked_add(amount).unwrap();

    Ok(())
}
