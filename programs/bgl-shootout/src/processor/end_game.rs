use mpl_utils::{assert_derivation, close_account_raw};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    system_program,
};

use crate::{error::BglShootoutError, instruction::EndGameArgs, state::GAME};

pub(crate) fn end_game(accounts: &[AccountInfo], args: EndGameArgs) -> ProgramResult {
    // Accounts.
    let account_info_iter = &mut accounts.iter();
    let game_pda_info = next_account_info(account_info_iter)?;
    let payer_info = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    // Guards.
    if *system_program.key != system_program::id() {
        return Err(BglShootoutError::InvalidSystemProgram.into());
    }

    let payer_address = payer_info.key.to_bytes();
    let mut game_seeds = vec![
        GAME.as_bytes(),
        args.match_name.as_bytes(),
        &payer_address,
        args.mint.as_ref(),
    ];

    let bump = &[assert_derivation(
        &crate::ID,
        game_pda_info,
        &game_seeds,
        BglShootoutError::DerivedKeyInvalid,
    )?];

    game_seeds.push(bump);

    close_account_raw(payer_info, game_pda_info)
}
