#![cfg(feature = "test-bpf")]

use borsh::BorshDeserialize;
use bgl_shootout::{instruction::CreateArgs, state::MyAccount};
use solana_program_test::{tokio, ProgramTest};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};

#[tokio::test]
async fn create() {
    let mut context = ProgramTest::new("bgl_shootout", bgl_shootout::ID, None)
        .start_with_context()
        .await;

    let address = Keypair::new();
    let create_args = CreateArgs { foo: 1, bar: 2 };

    let ix = bgl_shootout::instruction::create(
        &address.pubkey(),
        &context.payer.pubkey(),
        &context.payer.pubkey(),
        create_args,
    );

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&context.payer.pubkey()),
        &[&context.payer, &address],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    let account = context
        .banks_client
        .get_account(address.pubkey())
        .await
        .unwrap();

    assert!(account.is_some());

    let account = account.unwrap();
    assert_eq!(account.data.len(), MyAccount::LEN);

    let mut account_data = account.data.as_ref();
    let my_account = MyAccount::deserialize(&mut account_data).unwrap();
    assert_eq!(my_account.data.foo, 1);
    assert_eq!(my_account.data.bar, 2);
}