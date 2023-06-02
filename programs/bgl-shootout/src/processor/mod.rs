use crate::instruction::BglShootoutInstruction;
use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

mod end_game;
mod play_round;
mod start_game;
pub(crate) use end_game::*;
pub(crate) use play_round::*;
pub(crate) use start_game::*;

pub struct Processor;
impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction: BglShootoutInstruction =
            BglShootoutInstruction::try_from_slice(instruction_data)?;
        match instruction {
            BglShootoutInstruction::StartGame(args) => {
                msg!("Instruction: StartGame");
                start_game(accounts, args)
            }
            BglShootoutInstruction::EndGame(args) => {
                msg!("Instruction: EndGame");
                end_game(accounts, args)
            }
            BglShootoutInstruction::PlayRound(args) => {
                msg!("Instruction: PlayRound");
                play_round(accounts, args)
            }
        }
    }
}
