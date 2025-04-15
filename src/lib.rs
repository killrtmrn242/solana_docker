use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct HelloAccount {
    pub counter: u32,
}

// Declare the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");

    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    let mut hello_account = HelloAccount::try_from_slice(&account.data.borrow())?;
    hello_account.counter += 1;
    hello_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Counter updated to: {}", hello_account.counter);

    Ok(())
}
