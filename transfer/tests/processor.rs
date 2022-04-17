#[cfg(test)]
mod tests {
    use {
        solana_program::{
            instruction::{AccountMeta, Instruction},
            native_token::LAMPORTS_PER_SOL,
            pubkey::Pubkey,
        },
        solana_program_test::*,
        solana_sdk::{
            account::Account, signature::Signer, signer::keypair::Keypair, transaction::Transaction,
        },
    };

    use std::str::FromStr;

    use transfer::processor::process_instruction;

    #[tokio::test]
    async fn transfer() {
        let program_id = Pubkey::from_str("AJ4YB3dHrwUAKoLbS6RdfncBNJqfq1338B2MTAmTw4XF").unwrap();
        let from = Keypair::new();
        let to = Keypair::new();

        let mut program_test =
            ProgramTest::new("transfer", program_id, processor!(process_instruction));

        let amount = LAMPORTS_PER_SOL * 5;

        program_test.add_account(
            to.pubkey(),
            Account {
                data: vec![],
                executable: false,
                lamports: amount,
                owner: program_id,
                rent_epoch: 0,
            },
        );
        program_test.add_account(
            from.pubkey(),
            Account {
                data: vec![],
                executable: false,
                lamports: amount,
                owner: program_id,
                rent_epoch: 0,
            },
        );

        let (mut bank_client, payer, recent_blockhash) = program_test.start().await;

        let mut transaction = Transaction::new_with_payer(
            &[Instruction::new_with_borsh(
                program_id,
                &(),
                vec![
                    AccountMeta {
                        pubkey: from.pubkey(),
                        is_signer: false,
                        is_writable: true,
                    },
                    AccountMeta {
                        pubkey: to.pubkey(),
                        is_signer: false,
                        is_writable: true,
                    },
                ],
            )],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);
        bank_client.process_transaction(transaction).await.unwrap();

        assert_eq!(
            bank_client
                .get_account(from.pubkey())
                .await
                .unwrap()
                .unwrap()
                .lamports,
            LAMPORTS_PER_SOL * (5 - 1),
        );
        assert_eq!(
            bank_client
                .get_account(to.pubkey())
                .await
                .unwrap()
                .unwrap()
                .lamports,
            LAMPORTS_PER_SOL * (5 + 1),
        );
    }
}
