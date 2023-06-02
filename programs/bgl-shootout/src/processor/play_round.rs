use mpl_utils::assert_derivation;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    system_program,
};

use crate::{
    error::BglShootoutError,
    instruction::PlayRoundArgs,
    state::{Action, GameAccount, RoundResult, GAME},
};

pub(crate) fn play_round(accounts: &[AccountInfo], args: PlayRoundArgs) -> ProgramResult {
    // Accounts.
    let account_info_iter = &mut accounts.iter();
    let game_pda_info = next_account_info(account_info_iter)?;
    let payer_info = next_account_info(account_info_iter)?;
    let slot_hashes_info = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    // Guards.
    if *system_program.key != system_program::id() {
        return Err(BglShootoutError::InvalidSystemProgram.into());
    }

    let payer_address = payer_info.key.to_bytes();
    let game_seeds = vec![
        GAME.as_bytes(),
        args.match_name.as_bytes(),
        &payer_address,
        args.mint.as_ref(),
    ];

    let bump = assert_derivation(
        &crate::ID,
        game_pda_info,
        &game_seeds,
        BglShootoutError::DerivedKeyInvalid,
    )?;

    let mut game_account = GameAccount::load(game_pda_info)?;

    assert!(game_account.bump == bump);

    let player_action = args.action;
    let cpu_action = Action::rand_action(slot_hashes_info)?;

    let round_result = Action::get_round_result(player_action, cpu_action);
    match round_result {
        RoundResult::PlayerWin => {
            game_account.player_wins += 1;
        }
        RoundResult::CpuWin => {
            game_account.cpu_wins += 1;
        }
        RoundResult::Draw => {
            game_account.draws += 1;
        }
    }

    game_account.save(game_pda_info)?;

    Ok(())
}
