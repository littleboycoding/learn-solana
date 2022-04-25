pub mod entrypoint;
mod processor;
mod state;

#[cfg(test)]
mod tests {
    use crate::{
        processor::{process_instruction, Payload},
        state::AccountData,
    };
    use borsh::{BorshDeserialize, BorshSerialize};
    use solana_program_test::*;
    use solana_sdk::{
        account::Account,
        instruction::{AccountMeta, Instruction},
        native_token::sol_to_lamports,
        pubkey::Pubkey,
        signature::Signer,
        system_program,
        transaction::Transaction,
    };
    use std::{borrow::BorrowMut, str::FromStr};

    const PROGRAM_NAME: &str = "pda";
    const SEED: &[u8] = b"pda_counter";
    const PROGRAM_ID: &str = "4jKPaw1ZMW78nC2qVqZ2TL5s3At2Frdhn3hakCMbW8rU";

    #[tokio::test]
    async fn initialize() {
        let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
        let program_test =
            ProgramTest::new(PROGRAM_NAME, program_id, processor!(process_instruction));

        let (mut bank_client, payer, hash) = program_test.start().await;

        let (pda, bumps) = Pubkey::find_program_address(&[SEED], &program_id);

        let mut transaction = Transaction::new_with_payer(
            &[Instruction::new_with_borsh(
                program_id,
                &Payload {
                    variant: 0,
                    seed: Some(SEED.to_vec()),
                    bumps: Some(bumps),
                },
                vec![
                    AccountMeta {
                        is_signer: true,
                        is_writable: true,
                        pubkey: payer.pubkey(),
                    },
                    AccountMeta {
                        is_signer: false,
                        is_writable: true,
                        pubkey: pda,
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

        transaction.sign(&[&payer], hash);

        bank_client
            .process_transactions(vec![transaction])
            .await
            .unwrap();

        let pda = bank_client.get_account(pda).await.unwrap().unwrap();

        assert_eq!(pda.owner, program_id);
    }

    #[tokio::test]
    async fn increment() {
        let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
        let (pda, _) = Pubkey::find_program_address(&[SEED], &program_id);

        let mut program_test =
            ProgramTest::new(PROGRAM_NAME, program_id, processor!(process_instruction));

        let initial_data = AccountData { counter: 0 };

        program_test.add_account(
            pda,
            Account {
                executable: false,
                lamports: sol_to_lamports(0.00094656),
                owner: program_id,
                rent_epoch: 0,
                data: initial_data.try_to_vec().unwrap(),
            },
        );

        let (mut bank_client, payer, hash) = program_test.start().await;

        let mut transaction = Transaction::new_with_payer(
            &[Instruction::new_with_borsh(
                program_id,
                &Payload {
                    variant: 1,
                    bumps: None,
                    seed: None,
                },
                vec![AccountMeta {
                    is_signer: false,
                    is_writable: true,
                    pubkey: pda,
                }],
            )],
            Some(&payer.pubkey()),
        );

        transaction.sign(&[&payer], hash);

        bank_client
            .process_transactions(vec![transaction])
            .await
            .unwrap();

        let user_data = AccountData::try_from_slice(
            bank_client
                .get_account(pda)
                .await
                .unwrap()
                .unwrap()
                .data
                .borrow_mut(),
        )
        .unwrap();

        assert_eq!(user_data.counter, 1);
    }

    #[tokio::test]
    async fn decrement() {
        let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
        let (pda, _) = Pubkey::find_program_address(&[SEED], &program_id);

        let mut program_test =
            ProgramTest::new(PROGRAM_NAME, program_id, processor!(process_instruction));

        let initial_data = AccountData { counter: 5 };

        program_test.add_account(
            pda,
            Account {
                executable: false,
                lamports: sol_to_lamports(0.00094656),
                owner: program_id,
                rent_epoch: 0,
                data: initial_data.try_to_vec().unwrap(),
            },
        );

        let (mut bank_client, payer, hash) = program_test.start().await;

        let mut transaction = Transaction::new_with_payer(
            &[Instruction::new_with_borsh(
                program_id,
                &Payload {
                    variant: 2,
                    bumps: None,
                    seed: None,
                },
                vec![AccountMeta {
                    is_signer: false,
                    is_writable: true,
                    pubkey: pda,
                }],
            )],
            Some(&payer.pubkey()),
        );

        transaction.sign(&[&payer], hash);

        bank_client
            .process_transactions(vec![transaction])
            .await
            .unwrap();

        let user_data = AccountData::try_from_slice(
            bank_client
                .get_account(pda)
                .await
                .unwrap()
                .unwrap()
                .data
                .borrow_mut(),
        )
        .unwrap();

        assert_eq!(user_data.counter, 4);
    }
}
