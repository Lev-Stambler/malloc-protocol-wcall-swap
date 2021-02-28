#![allow(missing_docs)]

//! Template for Malloc Protocol's wrapped program calls

mod entrypoint;
mod error;
pub mod processor;

// Export current sdk types for downstream users building with a different sdk version
use core::str::FromStr;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use error::ProgError;
use serde::{Deserialize, Serialize};
pub use solana_program;
use solana_program::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction},
    msg,
    program::{invoke_signed, invoke},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::{
    instruction::{approve as spl_approve, transfer as spl_transfer},
    state::Account,
};

pub const WCALL_SEED: &[u8] = b"wcall";
pub const TOKEN_PROG_ID: &'static str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
pub const MALLOC_PROG_ID: &'static str = "ma11ocdevdevdevdevdevdevdevdevdevdevdevdevd";

solana_program::declare_id!("ma11ocwca1111111111111111111111111111111111");

/// call "Transfer" for SPL token
/// four account infos: SPL token, source, destination, signer (delegate / owner)
pub fn transfer(
    amount: u64,
    prog_id: &Pubkey,
    account_infos: &[AccountInfo],
) -> Result<(), ProgError> {
    //let wcall_pubkey = Pubkey::create_program_address(&[WCALL_SEED], prog_id).map_err(|e| {
       // msg!("error finding program-derived address!");
        //ProgError::ProgDerivedAddrError
    //})?;

//pub fn transfer(
//    token_program_id: &Pubkey, 
//    source_pubkey: &Pubkey, 
//    destination_pubkey: &Pubkey, 
//    authority_pubkey: &Pubkey, 
//    signer_pubkeys: &[&Pubkey], 
//    amount: u64
//) -> Result<Instruction, ProgramError>
    let insn = spl_transfer(
        &Pubkey::from_str(TOKEN_PROG_ID).unwrap(),
        &account_infos[1].key,
        &account_infos[2].key,
        &account_infos[3].key,
        &[&account_infos[3].key],
        amount,
    )
    .map_err(|e| {
        msg!("error constructing SPL transfer: {}", e);
        ProgError::TransferError
    })?;
    invoke(&insn, account_infos).map_err(|e| {
        msg!("error in SPL transfer: {}", e);
        ProgError::TransferError
    })?;
  //  invoke_signed(&insn, account_infos, &[&[WCALL_SEED]]).map_err(|e| {
  //      msg!("error in SPL transfer: {}", e);
  //      ProgError::TransferError
  //  })?;

    Ok(())
}

/// call "Approve" for SPL token
/// four account infos: SPL Program, source, delegate, owner of source
pub fn approve(
    amount: u64,
    prog_id: &Pubkey,
    account_infos: &[AccountInfo],
) -> Result<(), ProgError> {
    let wcall_pubkey = Pubkey::create_program_address(&[WCALL_SEED], prog_id).map_err(|e| {
        msg!("error finding program-derived address!");
        ProgError::ProgDerivedAddrError
    })?;
    let insn = spl_approve(
        &Pubkey::from_str(TOKEN_PROG_ID).unwrap(),
        &account_infos[1].key,
        &account_infos[2].key,
        &wcall_pubkey,
        &[&wcall_pubkey],
        amount,
    )
    .map_err(|e| {
        msg!("error constructing SPL approve: {}", e);
        ProgError::ApproveError
    })?;

    invoke(&insn, account_infos).map_err(|e| {
        msg!("error in SPL approve: {}", e);
        ProgError::ApproveError
    })?;

    Ok(())
}

pub fn get_split_balance(input: &[u8]) -> Result<u64, ProgError> {
    let inp_trimmed = &input[8..];
    let mut rdr = Cursor::new(inp_trimmed);

    msg!("input data of {:?}", input);
    let val = rdr.read_u64::<BigEndian>().unwrap();
    // TODO: Ghetto and unsafe!!
    Ok(val)
    //let state = Account::unpack_from_slice(split_account_data).map_err(|e| {
    //    msg!("falid to unpack split account data: {}", e);
    //    ProgError::InvalidState
    //})?;
    //msg!("Amount of {}", state.amount);
    //Ok(state.delegated_amount)
}
