use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;

mod end_game;
mod play_round;
mod start_game;
pub use end_game::*;
pub use play_round::*;
pub use start_game::*;

#[derive(Debug, Clone, ShankInstruction, BorshSerialize, BorshDeserialize)]
#[rustfmt::skip]
pub enum BglShootoutInstruction {
    /// Create the game account.
    #[account(0, writable, name="game_pda", desc = "The PDA of the game")]
    #[account(1, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(2, name="system_program", desc = "The system program")]
    #[account(3, optional, name="authority", desc = "The authority who will control gameplay")]
    StartGame(StartGameArgs),

    /// Close the game account.
    #[account(0, writable, name="game_pda", desc = "The PDA of the game")]
    #[account(1, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(2, name="system_program", desc = "The system program")]
    EndGame(EndGameArgs),

    /// Close the game account.
    #[account(0, writable, name="game_pda", desc = "The PDA of the game")]
    #[account(1, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(2, name="slot_hashes_sysvar", desc = "The Slot_Hashes System Variable")]
    #[account(3, name="system_program", desc = "The system program")]
    PlayRound(PlayRoundArgs),
}
