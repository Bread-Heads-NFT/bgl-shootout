use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

use super::BglShootoutInstruction;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct StartGameArgs {
    pub match_name: String,
    pub mint: Pubkey,
    pub num_rounds: u8,
}

pub fn start_game(game_pda: &Pubkey, payer: &Pubkey, args: StartGameArgs) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*game_pda, false),
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(system_program::ID, false),
    ];
    Instruction {
        program_id: crate::ID,
        accounts,
        data: BglShootoutInstruction::StartGame(args)
            .try_to_vec()
            .unwrap(),
    }
}
