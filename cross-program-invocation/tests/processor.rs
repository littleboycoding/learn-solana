#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cross_program_invocation::processor::instruction_processor;
    use solana_program::{
        instruction::AccountMeta, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, system_program,
    };
    use solana_program_test::*;
    use solana_sdk::{
        account::Account, instruction::Instruction, signature::Keypair, signer::Signer,
        transaction::Transaction,
    };

    #[tokio::test]
    async fn cross_program_invocation() {
        let program_id = Pubkey::from_str("4Quv15soKrrfgnm4mp3sMsW16FBQH7hjSdTJi1ibqzqF").unwrap();
        let mut program_test = ProgramTest::new(
            "cross-program-invocation",
            program_id,
            processor!(instruction_processor),
        );

        let from = Keypair::new();
        let to = Keypair::new();

        program_test.add_account(
            from.pubkey(),
            Account {
                owner: system_program::ID,
                data: vec![],
                executable: false,
                lamports: LAMPORTS_PER_SOL * 5,
                rent_epoch: 0,
            },
        );
        program_test.add_account(
            to.pubkey(),
            Account {
                owner: system_program::ID,
                data: vec![],
                executable: false,
                lamports: 0,
                rent_epoch: 0,
            },
        );

        let (mut bank, payer, blockhash) = program_test.start().await;

        let mut transaction = Transaction::new_with_payer(
            &[Instruction::new_with_borsh(
                program_id,
                &(),
                vec![
                    AccountMeta {
                        is_signer: true,
                        is_writable: true,
                        pubkey: from.pubkey(),
                    },
                    AccountMeta {
                        is_signer: false,
                        is_writable: true,
                        pubkey: to.pubkey(),
                    },
                    AccountMeta {
                        is_signer: false,
                        is_writable: false,
                        pubkey: system_program::ID,
                    },
                ],
            )],
            Some(&payer.pubkey()),
        );

        transaction.sign(&[&payer, &from], blockhash);

        bank.process_transaction(transaction).await.unwrap();

        let to_lamports = bank
            .get_account(to.pubkey())
            .await
            .unwrap()
            .unwrap()
            .lamports;

        assert_eq!(to_lamports, LAMPORTS_PER_SOL * 5);
    }
}
