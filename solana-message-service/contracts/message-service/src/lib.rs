use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::{Pack, Sealed},
    sysvar::{rent::Rent, Sysvar},
    program_pack::IsInitialized,
    hash::Hash,
};

use borsh::{BorshDeserialize, BorshSerialize};

const PROGRAM_ID: Pubkey = Pubkey::new_from_array([0; 32]);

#[derive(Debug, BorshSerialize, BorshDeserialize)]
struct Message {
    from: Pubkey,
    to: Pubkey,
    content: String,
}

#[derive(Debug, PartialEq)]
pub enum MessageServiceError {
    InvalidInstruction,
    MessageTooLong,
    InsufficientFunds,
}

#[entrypoint]
fn send_message(
    ctx: Context,
    to: Pubkey,
    content: String,
) -> ProgramResult {
    if content.len() > 256 {
        return Err(MessageServiceError::MessageTooLong.into());
    }

    let message = Message {
        from: *ctx.accounts.sender.key,
        to,
        content,
    };

    let mut message_data = message.try_to_vec().unwrap();

    let rent = &Rent::from_account_info(next_account_info(next_account_info(&ctx.accounts[0])?)?)?;
    let required_lamports = rent.minimum_balance(message_data.len());

    if ctx.accounts.sender.lamports() < required_lamports {
        return Err(MessageServiceError::InsufficientFunds.into());
    }

    ctx.accounts.sender.transfer(ctx.accounts.program.key, required_lamports)?;

    let mut to_account_data = ctx.accounts.to_account.data.borrow_mut();
    to_account_data.extend_from_slice(&message_data);

    Ok(())
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data[0] {
        0 => send_message(
            Context {
                program_id: *program_id,
                accounts,
            },
            Pubkey::new_from_array(instruction_data[1..33].try_into().unwrap()),
            String::from_utf8(instruction_data[33..].to_vec()).unwrap(),
        ),
        _ => Err(MessageServiceError::InvalidInstruction.into()),
    }
}

pub struct Context<'a> {
    pub program_id: Pubkey,
    pub accounts: &'a [AccountInfo<'a>],
}

impl From<MessageServiceError> for ProgramError {
    fn from(e: MessageServiceError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
