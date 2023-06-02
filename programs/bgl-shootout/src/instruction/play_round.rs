use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
    sysvar::slot_hashes,
};

use crate::state::Action;

use super::BglShootoutInstruction;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct PlayRoundArgs {
    pub match_name: String,
    pub mint: Pubkey,
    pub action: Action,
}

pub fn play_round(game_pda: &Pubkey, payer: &Pubkey, args: PlayRoundArgs) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*game_pda, false),
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(slot_hashes::ID, false),
        AccountMeta::new_readonly(system_program::ID, false),
    ];
    Instruction {
        program_id: crate::ID,
        accounts,
        data: BglShootoutInstruction::PlayRound(args)
            .try_to_vec()
            .unwrap(),
    }
}
