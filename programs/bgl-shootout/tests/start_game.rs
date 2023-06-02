#![cfg(feature = "test-bpf")]

use bgl_shootout::{
    instruction::StartGameArgs,
    state::{GameAccount, GAME},
};
use borsh::BorshDeserialize;
use solana_program::pubkey::Pubkey;
use solana_program_test::{tokio, ProgramTest};
use solana_sdk::{signature::Signer, transaction::Transaction};

#[tokio::test]
async fn start_game() {
    let mut context = ProgramTest::new("bgl_shootout", bgl_shootout::ID, None)
        .start_with_context()
        .await;

    let mint = Pubkey::new_unique();

    let name = "test".to_string();
    let game_seeds = [
        GAME.as_bytes(),
        name.as_bytes(),
        &context.payer.pubkey().to_bytes(),
        mint.as_ref(),
    ];

    let game_pda_address = Pubkey::find_program_address(&game_seeds, &bgl_shootout::ID);

    let create_args = StartGameArgs {
        match_name: name,
        mint,
        num_rounds: 2,
    };

    let ix = bgl_shootout::instruction::start_game(
        &game_pda_address.0,
        &context.payer.pubkey(),
        create_args,
    );

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    let account = context
        .banks_client
        .get_account(game_pda_address.0)
        .await
        .unwrap();

    assert!(account.is_some());

    let account = account.unwrap();
    assert_eq!(account.data.len(), GameAccount::LEN);

    let mut account_data = account.data.as_ref();
    let game_account = GameAccount::deserialize(&mut account_data).unwrap();
    assert_eq!(game_account.match_name, "test".to_string());
    assert_eq!(game_account.bump, game_pda_address.1);
    assert_eq!(game_account.player_wins, 0);
    assert_eq!(game_account.cpu_wins, 0);
    assert_eq!(game_account.num_rounds, 2);
}
