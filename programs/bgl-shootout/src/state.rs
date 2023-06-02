use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::error::BglShootoutError;
use crate::util::rand_choice;

pub const GAME: &str = "game";

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub enum Key {
    Uninitialized,
    GameAccount,
    Response,
}

/// Fancy rock paper scissors:
/// - Shoot beats Lasso
/// - Lasso beats Duck
/// - Duck beats Shoot
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, Eq, PartialEq)]
pub enum Action {
    Shoot,
    Lasso,
    Duck,
}

pub enum RoundResult {
    PlayerWin,
    CpuWin,
    Draw,
}

impl Action {
    pub fn rand_action(slot_hashes: &AccountInfo) -> Result<Self, ProgramError> {
        rand_choice(
            &vec![Action::Shoot, Action::Lasso, Action::Duck],
            slot_hashes,
        )
    }

    pub fn get_round_result(player_action: Action, cpu_action: Action) -> RoundResult {
        match (player_action, cpu_action) {
            (Action::Shoot, Action::Shoot)
            | (Action::Duck, Action::Duck)
            | (Action::Lasso, Action::Lasso) => RoundResult::Draw,
            (Action::Shoot, Action::Lasso)
            | (Action::Lasso, Action::Duck)
            | (Action::Duck, Action::Shoot) => RoundResult::PlayerWin,
            (Action::Shoot, Action::Duck)
            | (Action::Lasso, Action::Shoot)
            | (Action::Duck, Action::Lasso) => RoundResult::CpuWin,
        }
    }
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, ShankAccount)]
pub struct GameAccount {
    pub key: Key,
    pub bump: u8,
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub match_name: String,
    pub player_wins: u8,
    pub cpu_wins: u8,
    pub draws: u8,
    pub num_rounds: u8,
}

impl GameAccount {
    pub const LEN: usize = 1 + 1 + 32 + 32 + (4 + 32) + 4 + 4 + 128;

    pub fn load(account: &AccountInfo) -> Result<Self, ProgramError> {
        let mut bytes: &[u8] = &(*account.data).borrow();
        GameAccount::deserialize(&mut bytes).map_err(|error| {
            msg!("Error: {}", error);
            BglShootoutError::DeserializationError.into()
        })
    }

    pub fn save(&self, account: &AccountInfo) -> ProgramResult {
        let mut bytes = Vec::with_capacity(account.data_len());
        self.serialize(&mut bytes).map_err(|error| {
            msg!("Error: {}", error);
            BglShootoutError::SerializationError
        })?;
        account.try_borrow_mut_data().unwrap()[..bytes.len()].copy_from_slice(&bytes);
        Ok(())
    }
}
