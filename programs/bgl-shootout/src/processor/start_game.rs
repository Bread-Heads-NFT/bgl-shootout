use mpl_utils::{assert_derivation, create_or_allocate_account_raw};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    system_program,
};

use crate::{
    error::BglShootoutError,
    instruction::StartGameArgs,
    state::{GameAccount, Key, GAME},
};

pub(crate) fn start_game(accounts: &[AccountInfo], args: StartGameArgs) -> ProgramResult {
    // Accounts.
    let account_info_iter = &mut accounts.iter();
    let game_pda_info = next_account_info(account_info_iter)?;
    let payer_info = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let authority_info = match next_account_info(account_info_iter) {
        Ok(authority) => authority,
        Err(_) => payer_info,
    };

    // Guards.
    if *system_program.key != system_program::id() {
        return Err(BglShootoutError::InvalidSystemProgram.into());
    }

    let authority_address = authority_info.key.to_bytes();
    let mut game_seeds = vec![
        GAME.as_bytes(),
        args.match_name.as_bytes(),
        &authority_address,
        args.mint.as_ref(),
    ];

    let bump = &[assert_derivation(
        &crate::ID,
        game_pda_info,
        &game_seeds,
        BglShootoutError::DerivedKeyInvalid,
    )?];

    game_seeds.push(bump);

    create_or_allocate_account_raw(
        crate::ID,
        game_pda_info,
        system_program,
        payer_info,
        GameAccount::LEN,
        &game_seeds,
    )?;

    let game_account = GameAccount {
        key: Key::GameAccount,
        bump: bump[0],
        owner: *payer_info.key,
        mint: args.mint,
        match_name: args.match_name,
        player_wins: 0,
        cpu_wins: 0,
        draws: 0,
        num_rounds: args.num_rounds,
    };

    game_account.save(game_pda_info)
}
