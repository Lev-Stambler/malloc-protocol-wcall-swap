#![allow(missing_docs)]
use crate::error::ProgError;
use crate::{get_split_balance, transfer};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Instruction processor
/// expects two accounts (aside from program account): source, dest
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    mut input: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let prog_account = account_info_iter
        .next()
        .ok_or(ProgramError::NotEnoughAccountKeys)?;

    let prog_id = prog_account.key;

    let _ = go_nuts(prog_id, accounts, input).map_err(|e| {
        msg!("failed to go_nuts: {}", e);
        ProgramError::Custom(2)
    })?;

    Ok(())
}

// do literally anything
fn go_nuts(prog_id: &Pubkey, accounts: &[AccountInfo], mut input: &[u8]) -> ProgramResult {
    let split_balance = get_split_balance(input)?;
    msg!("Split balance of {}", split_balance);
    let account_info_iter = &mut accounts.iter();
    let prog_account = account_info_iter
        .next()
        .ok_or(ProgramError::NotEnoughAccountKeys)?;

    transfer(
        split_balance,
        prog_id,
        &[
            // spl_account
            accounts[2].to_owned(),
            // source (i.e. Malloc Input account)
            accounts[1].to_owned(),
            // destination (passed in as an associated to this WCall)
            accounts[4].to_owned(),
            // ephemeral "split" account
            accounts[3].to_owned(),
        ],
    )
    .map_err(|e| {
        msg!("error transferring from malloc: {}", e);
        e
    })?;

    /* approve_output(&recipient, output_amount, &prog_id).map_err(|e| {
        msg!("error approving output to recipient: {}", e);
        ProgramError::Custom(1)
    })?;
    */
    Ok(())
}
