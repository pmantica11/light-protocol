#![cfg(feature = "test-sbf")]
use account_compression::sdk::create_insert_leaves_instruction;
use account_compression::utils::constants::{CPI_AUTHORITY_PDA_SEED, STATE_NULLIFIER_QUEUE_VALUES};
use account_compression::{QueueAccount, StateMerkleTreeAccount};
use anchor_lang::{system_program, InstructionData, ToAccountMetas};
use light_compressed_token::mint_sdk::create_mint_to_instruction;
use light_registry::get_forester_epoch_pda_address;
use light_registry::sdk::{
    create_nullify_instruction, get_cpi_authority_pda, CreateNullifyInstructionInputs,
};
use light_system_program::utils::get_registered_program_pda;
use light_test_utils::rpc::errors::{assert_rpc_error, RpcError};
use light_test_utils::rpc::rpc_connection::RpcConnection;
use light_test_utils::rpc::test_rpc::ProgramTestRpcConnection;
use light_test_utils::test_env::{
    create_address_merkle_tree_and_queue_account, create_state_merkle_tree_and_queue_account,
    initialize_new_group, register_program_with_registry_program, NOOP_PROGRAM_ID,
};
use light_test_utils::transaction_params::{FeeConfig, TransactionParams};
use light_test_utils::{airdrop_lamports, create_account_instruction};
use light_test_utils::{
    assert_custom_error_or_program_error,
    indexer::{create_mint_helper, TestIndexer},
    test_env::setup_test_programs_with_accounts,
    AccountZeroCopy,
};
use solana_sdk::instruction::Instruction;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction};

#[tokio::test]
async fn test_program_owned_merkle_tree() {
    let (mut rpc, env) = setup_test_programs_with_accounts(Some(vec![(
        String::from("system_cpi_test"),
        system_cpi_test::ID,
    )]))
    .await;
    let payer = rpc.get_payer().insecure_clone();
    let payer_pubkey = payer.pubkey();

    let program_owned_merkle_tree_keypair = Keypair::new();
    let program_owned_merkle_tree_pubkey = program_owned_merkle_tree_keypair.pubkey();
    let program_owned_nullifier_queue_keypair = Keypair::new();
    let cpi_context_keypair = Keypair::new();

    let mut test_indexer = TestIndexer::<200, ProgramTestRpcConnection>::init_from_env(
        &payer,
        &env,
        true,
        true,
        "../../circuit-lib/circuitlib-rs/scripts/prover.sh",
    )
    .await;
    test_indexer
        .add_state_merkle_tree(
            &mut rpc,
            &program_owned_merkle_tree_keypair,
            &program_owned_nullifier_queue_keypair,
            &cpi_context_keypair,
            Some(light_compressed_token::ID),
        )
        .await;

    let recipient_keypair = Keypair::new();
    let mint = create_mint_helper(&mut rpc, &payer).await;
    let amount = 10000u64;
    let instruction = create_mint_to_instruction(
        &payer_pubkey,
        &payer_pubkey,
        &mint,
        &program_owned_merkle_tree_pubkey,
        vec![amount; 1],
        vec![recipient_keypair.pubkey(); 1],
    );
    let pre_merkle_tree_account =
        AccountZeroCopy::<StateMerkleTreeAccount>::new(&mut rpc, program_owned_merkle_tree_pubkey)
            .await;
    let pre_merkle_tree = pre_merkle_tree_account
        .deserialized()
        .copy_merkle_tree()
        .unwrap();
    let event = rpc
        .create_and_send_transaction_with_event(
            &[instruction],
            &payer_pubkey,
            &[&payer],
            Some(TransactionParams {
                num_new_addresses: 0,
                num_input_compressed_accounts: 0,
                num_output_compressed_accounts: 1,
                compress: 0,
                fee_config: FeeConfig::default(),
            }),
        )
        .await
        .unwrap()
        .unwrap();
    let post_merkle_tree_account =
        AccountZeroCopy::<StateMerkleTreeAccount>::new(&mut rpc, program_owned_merkle_tree_pubkey)
            .await;
    let post_merkle_tree = post_merkle_tree_account
        .deserialized()
        .copy_merkle_tree()
        .unwrap();
    test_indexer.add_compressed_accounts_with_token_data(&event);
    assert_ne!(post_merkle_tree.root(), pre_merkle_tree.root());
    assert_eq!(
        post_merkle_tree.root(),
        test_indexer.state_merkle_trees[1].merkle_tree.root()
    );

    let invalid_program_owned_merkle_tree_keypair = Keypair::new();
    let invalid_program_owned_merkle_tree_pubkey =
        invalid_program_owned_merkle_tree_keypair.pubkey();
    let invalid_program_owned_nullifier_queue_keypair = Keypair::new();
    let cpi_context_keypair = Keypair::new();
    test_indexer
        .add_state_merkle_tree(
            &mut rpc,
            &invalid_program_owned_merkle_tree_keypair,
            &invalid_program_owned_nullifier_queue_keypair,
            &cpi_context_keypair,
            Some(Keypair::new().pubkey()),
        )
        .await;
    let recipient_keypair = Keypair::new();
    let instruction = create_mint_to_instruction(
        &payer_pubkey,
        &payer_pubkey,
        &mint,
        &invalid_program_owned_merkle_tree_pubkey,
        vec![amount + 1; 1],
        vec![recipient_keypair.pubkey(); 1],
    );

    let latest_blockhash = rpc.get_latest_blockhash().await.unwrap();
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer_pubkey),
        &[&payer],
        latest_blockhash,
    );
    let res = rpc.process_transaction(transaction).await;
    assert_custom_error_or_program_error(
        res,
        light_system_program::errors::SystemProgramError::InvalidMerkleTreeOwner.into(),
    )
    .unwrap();
}

const CPI_SYSTEM_TEST_PROGRAM_ID_KEYPAIR: [u8; 64] = [
    57, 80, 188, 3, 162, 80, 232, 181, 222, 192, 247, 98, 140, 227, 70, 15, 169, 202, 73, 184, 23,
    90, 69, 95, 211, 74, 128, 232, 155, 216, 5, 230, 213, 158, 155, 203, 26, 211, 193, 195, 11,
    219, 9, 155, 58, 172, 58, 200, 254, 75, 231, 106, 31, 168, 183, 76, 179, 113, 234, 101, 191,
    99, 156, 98,
];

/// Test:
/// - Register the test program
/// - failing test registered program signer check
/// 1. FAIL: try to append leaves to the merkle tree from test program with invalid registered program account
/// 2. try to append leaves to the merkle tree from account compression program
/// - register the test program to the correct group
/// 3. SUCCEED: append leaves to the merkle tree from test program
/// - register the token program to the correct group
/// 4. FAIL: try to append leaves to the merkle tree from test program with invalid registered program account
/// 5. FAIL: rollover state Merkle tree  with invalid group
/// 6. FAIL: rollover address Merkle tree with invalid group
/// 7. FAIL: update address Merkle tree with invalid group
/// 8. FAIL: nullify leaves with invalid group
/// 9. FAIL: insert into address queue with invalid group
/// 10. FAIL: insert into nullifier queue with invalid group
#[tokio::test]
async fn test_invalid_registered_program() {
    let (mut rpc, env) = setup_test_programs_with_accounts(Some(vec![(
        String::from("system_cpi_test"),
        system_cpi_test::ID,
    )]))
    .await;
    let payer = env.forester.insecure_clone();
    airdrop_lamports(&mut rpc, &payer.pubkey(), 100_000_000_000)
        .await
        .unwrap();
    let group_seed_keypair = Keypair::new();
    let program_id_keypair = Keypair::from_bytes(&CPI_SYSTEM_TEST_PROGRAM_ID_KEYPAIR).unwrap();
    let invalid_group_pda =
        initialize_new_group(&group_seed_keypair, &payer, &mut rpc, payer.pubkey()).await;
    let invalid_group_registered_program_pda =
        register_program(&mut rpc, &payer, &program_id_keypair, &invalid_group_pda)
            .await
            .unwrap();
    let invalid_group_state_merkle_tree = Keypair::new();
    let invalid_group_nullifier_queue = Keypair::new();
    create_state_merkle_tree_and_queue_account(
        &payer,
        &invalid_group_pda,
        &mut rpc,
        &invalid_group_state_merkle_tree,
        &invalid_group_nullifier_queue,
        None,
        3,
    )
    .await;
    let invalid_group_address_merkle_tree = Keypair::new();
    let invalid_group_address_queue = Keypair::new();
    create_address_merkle_tree_and_queue_account(
        &payer,
        &invalid_group_pda,
        &mut rpc,
        &invalid_group_address_merkle_tree,
        &invalid_group_address_queue,
        None,
        3,
    )
    .await;

    let merkle_tree_pubkey = env.merkle_tree_pubkey;

    // invoke account compression program through system cpi test
    // 1. the program is registered with a different group than the Merkle tree
    {
        let derived_address =
            Pubkey::find_program_address(&[CPI_AUTHORITY_PDA_SEED], &system_cpi_test::ID).0;
        let accounts = system_cpi_test::accounts::AppendLeavesAccountCompressionProgram {
            signer: payer.pubkey(),
            registered_program_pda: invalid_group_registered_program_pda,
            noop_program: Pubkey::new_from_array(
                account_compression::utils::constants::NOOP_PUBKEY,
            ),
            account_compression_program: account_compression::ID,
            cpi_signer: derived_address,
            system_program: solana_sdk::system_program::ID,
            merkle_tree: merkle_tree_pubkey,
            queue: merkle_tree_pubkey, // not used in this ix
        };

        let instruction_data =
            system_cpi_test::instruction::AppendLeavesAccountCompressionProgram {};
        let instruction = Instruction {
            program_id: system_cpi_test::ID,
            accounts: [accounts.to_account_metas(Some(true))].concat(),
            data: instruction_data.data(),
        };
        let result = rpc
            .create_and_send_transaction(&[instruction], &payer.pubkey(), &[&payer])
            .await;
        let expected_error_code =
            account_compression::errors::AccountCompressionErrorCode::InvalidAuthority.into();
        assert_rpc_error(result, 0, expected_error_code).unwrap();
    }

    // 2. directly invoke account compression program
    {
        let instruction = create_insert_leaves_instruction(
            vec![(0, [1u8; 32])],
            payer.pubkey(),
            payer.pubkey(),
            vec![merkle_tree_pubkey],
        );
        let expected_error_code =
            account_compression::errors::AccountCompressionErrorCode::InvalidAuthority.into();

        let result = rpc
            .create_and_send_transaction(&[instruction], &payer.pubkey(), &[&payer])
            .await;
        assert_rpc_error(result, 0, expected_error_code).unwrap();
    }
    let other_program_id_keypair = Keypair::new();
    let token_program_registered_program_pda =
        register_program_with_registry_program(&mut rpc, &env, &other_program_id_keypair)
            .await
            .unwrap();
    // 4. use registered_program_pda of other program
    {
        let derived_address =
            Pubkey::find_program_address(&[CPI_AUTHORITY_PDA_SEED], &system_cpi_test::ID).0;
        let accounts = system_cpi_test::accounts::AppendLeavesAccountCompressionProgram {
            signer: payer.pubkey(),
            registered_program_pda: token_program_registered_program_pda,
            noop_program: Pubkey::new_from_array(
                account_compression::utils::constants::NOOP_PUBKEY,
            ),
            account_compression_program: account_compression::ID,
            cpi_signer: derived_address,
            system_program: solana_sdk::system_program::ID,
            merkle_tree: merkle_tree_pubkey,
            queue: merkle_tree_pubkey, // not used in this ix
        };

        let instruction_data =
            system_cpi_test::instruction::AppendLeavesAccountCompressionProgram {};
        let instruction = Instruction {
            program_id: system_cpi_test::ID,
            accounts: [accounts.to_account_metas(Some(true))].concat(),
            data: instruction_data.data(),
        };
        let result = rpc
            .create_and_send_transaction(&[instruction], &payer.pubkey(), &[&payer])
            .await;
        let expected_error_code =
            account_compression::errors::AccountCompressionErrorCode::InvalidAuthority.into();

        assert_rpc_error(result, 0, expected_error_code).unwrap();
    }

    {
        let new_merkle_tree_keypair = Keypair::new();
        let new_queue_keypair = Keypair::new();
        let (cpi_authority, bump) = light_registry::sdk::get_cpi_authority_pda();
        let registered_program_pda = get_registered_program_pda(&light_registry::ID);
        let registered_forester_pda = get_forester_epoch_pda_address(&env.forester.pubkey()).0;
        let instruction_data =
            light_registry::instruction::RolloverStateMerkleTreeAndQueue { bump };
        let accounts = light_registry::accounts::RolloverMerkleTreeAndQueue {
            account_compression_program: account_compression::ID,
            registered_forester_pda,
            cpi_authority,
            authority: payer.pubkey(),
            registered_program_pda,
            new_merkle_tree: new_merkle_tree_keypair.pubkey(),
            new_queue: new_queue_keypair.pubkey(),
            old_merkle_tree: invalid_group_state_merkle_tree.pubkey(),
            old_queue: invalid_group_nullifier_queue.pubkey(),
        };
        let size = QueueAccount::size(STATE_NULLIFIER_QUEUE_VALUES as usize).unwrap();
        let create_nullifier_queue_instruction = create_account_instruction(
            &payer.pubkey(),
            size,
            rpc.get_minimum_balance_for_rent_exemption(size)
                .await
                .unwrap(),
            &account_compression::ID,
            Some(&new_queue_keypair),
        );
        let create_state_merkle_tree_instruction = create_account_instruction(
            &payer.pubkey(),
            account_compression::StateMerkleTreeAccount::LEN,
            rpc.get_minimum_balance_for_rent_exemption(
                account_compression::StateMerkleTreeAccount::LEN,
            )
            .await
            .unwrap(),
            &account_compression::ID,
            Some(&new_merkle_tree_keypair),
        );
        let instruction = Instruction {
            program_id: light_registry::ID,
            accounts: accounts.to_account_metas(Some(true)),
            data: instruction_data.data(),
        };
        let result = rpc
            .create_and_send_transaction(
                &[
                    create_nullifier_queue_instruction,
                    create_state_merkle_tree_instruction,
                    instruction,
                ],
                &payer.pubkey(),
                &[&payer, &new_merkle_tree_keypair, &new_queue_keypair],
            )
            .await;
        let expected_error_code =
            account_compression::errors::AccountCompressionErrorCode::InvalidAuthority.into();

        assert_rpc_error(result, 2, expected_error_code).unwrap();
    }
    // 6. rollover address Merkle tree with invalid group
    {
        let new_merkle_tree_keypair = Keypair::new();
        let new_queue_keypair = Keypair::new();
        let (cpi_authority, bump) = light_registry::sdk::get_cpi_authority_pda();
        let registered_program_pda = get_registered_program_pda(&light_registry::ID);
        let instruction_data =
            light_registry::instruction::RolloverAddressMerkleTreeAndQueue { bump };
        let registered_forester_pda = get_forester_epoch_pda_address(&env.forester.pubkey()).0;

        let accounts = light_registry::accounts::RolloverMerkleTreeAndQueue {
            account_compression_program: account_compression::ID,
            registered_forester_pda,
            cpi_authority,
            authority: payer.pubkey(),
            registered_program_pda,
            new_merkle_tree: new_merkle_tree_keypair.pubkey(),
            new_queue: new_queue_keypair.pubkey(),
            old_merkle_tree: invalid_group_address_merkle_tree.pubkey(),
            old_queue: invalid_group_address_queue.pubkey(),
        };
        let size = QueueAccount::size(
            account_compression::utils::constants::ADDRESS_QUEUE_VALUES as usize,
        )
        .unwrap();
        let create_nullifier_queue_instruction = create_account_instruction(
            &payer.pubkey(),
            size,
            rpc.get_minimum_balance_for_rent_exemption(size)
                .await
                .unwrap(),
            &account_compression::ID,
            Some(&new_queue_keypair),
        );
        let create_state_merkle_tree_instruction = create_account_instruction(
            &payer.pubkey(),
            account_compression::AddressMerkleTreeAccount::LEN,
            rpc.get_minimum_balance_for_rent_exemption(
                account_compression::AddressMerkleTreeAccount::LEN,
            )
            .await
            .unwrap(),
            &account_compression::ID,
            Some(&new_merkle_tree_keypair),
        );
        let instruction = Instruction {
            program_id: light_registry::ID,
            accounts: accounts.to_account_metas(Some(true)),
            data: instruction_data.data(),
        };
        let result = rpc
            .create_and_send_transaction(
                &[
                    create_nullifier_queue_instruction,
                    create_state_merkle_tree_instruction,
                    instruction,
                ],
                &payer.pubkey(),
                &[&payer, &new_merkle_tree_keypair, &new_queue_keypair],
            )
            .await;
        let expected_error_code =
            account_compression::errors::AccountCompressionErrorCode::InvalidAuthority.into();

        assert_rpc_error(result, 2, expected_error_code).unwrap();
    }
    // 7. nullify with invalid group
    {
        let inputs = CreateNullifyInstructionInputs {
            authority: payer.pubkey(),
            nullifier_queue: invalid_group_nullifier_queue.pubkey(),
            merkle_tree: invalid_group_state_merkle_tree.pubkey(),
            change_log_indices: vec![1],
            leaves_queue_indices: vec![1u16],
            indices: vec![0u64],
            proofs: vec![vec![[0u8; 32]; 26]],
        };
        let ix = create_nullify_instruction(inputs);

        let result = rpc
            .create_and_send_transaction(&[ix], &payer.pubkey(), &[&payer])
            .await;
        let expected_error_code =
            account_compression::errors::AccountCompressionErrorCode::InvalidAuthority.into();

        assert_rpc_error(result, 0, expected_error_code).unwrap();
    }
    // 8. update address with invalid group
    {
        let register_program_pda = get_registered_program_pda(&light_registry::ID);
        let registered_forester_pda = get_forester_epoch_pda_address(&env.forester.pubkey()).0;
        let (cpi_authority, bump) = get_cpi_authority_pda();
        let instruction_data = light_registry::instruction::UpdateAddressMerkleTree {
            bump,
            changelog_index: 1,
            value: 1u16,
            low_address_index: 1,
            low_address_proof: [[0u8; 32]; 16],
            low_address_next_index: 1,
            low_address_next_value: [0u8; 32],
            low_address_value: [0u8; 32],
        };
        let accounts = light_registry::accounts::UpdateAddressMerkleTree {
            authority: payer.pubkey(),
            registered_forester_pda,
            registered_program_pda: register_program_pda,
            queue: invalid_group_address_queue.pubkey(),
            merkle_tree: invalid_group_address_merkle_tree.pubkey(),
            log_wrapper: NOOP_PROGRAM_ID,
            cpi_authority,
            account_compression_program: account_compression::ID,
        };
        let ix = Instruction {
            program_id: light_registry::ID,
            accounts: accounts.to_account_metas(Some(true)),
            data: instruction_data.data(),
        };
        let result = rpc
            .create_and_send_transaction(&[ix], &payer.pubkey(), &[&payer])
            .await;
        let expected_error_code =
            account_compression::errors::AccountCompressionErrorCode::InvalidAuthority.into();

        assert_rpc_error(result, 0, expected_error_code).unwrap();
    }
    // 9. insert into address queue with invalid group
    {
        let derived_address =
            Pubkey::find_program_address(&[CPI_AUTHORITY_PDA_SEED], &system_cpi_test::ID).0;
        let accounts = system_cpi_test::accounts::AppendLeavesAccountCompressionProgram {
            signer: payer.pubkey(),
            registered_program_pda: token_program_registered_program_pda,
            noop_program: Pubkey::new_from_array(
                account_compression::utils::constants::NOOP_PUBKEY,
            ),
            account_compression_program: account_compression::ID,
            cpi_signer: derived_address,
            system_program: solana_sdk::system_program::ID,
            merkle_tree: env.address_merkle_tree_pubkey,
            queue: env.address_merkle_tree_queue_pubkey,
        };

        let instruction_data = system_cpi_test::instruction::InsertIntoAddressQueue {};
        let instruction = Instruction {
            program_id: system_cpi_test::ID,
            accounts: [accounts.to_account_metas(Some(true))].concat(),
            data: instruction_data.data(),
        };
        let result = rpc
            .create_and_send_transaction(&[instruction], &payer.pubkey(), &[&payer])
            .await;
        let expected_error_code =
            account_compression::errors::AccountCompressionErrorCode::InvalidAuthority.into();

        assert_rpc_error(result, 0, expected_error_code).unwrap();
    }
    // 10. insert into nullifier queue with invalid group
    {
        let derived_address =
            Pubkey::find_program_address(&[CPI_AUTHORITY_PDA_SEED], &system_cpi_test::ID).0;
        let accounts = system_cpi_test::accounts::AppendLeavesAccountCompressionProgram {
            signer: payer.pubkey(),
            registered_program_pda: token_program_registered_program_pda,
            noop_program: Pubkey::new_from_array(
                account_compression::utils::constants::NOOP_PUBKEY,
            ),
            account_compression_program: account_compression::ID,
            cpi_signer: derived_address,
            system_program: solana_sdk::system_program::ID,
            merkle_tree: env.merkle_tree_pubkey,
            queue: env.nullifier_queue_pubkey,
        };

        let instruction_data = system_cpi_test::instruction::InsertIntoNullifierQueue {};
        let instruction = Instruction {
            program_id: system_cpi_test::ID,
            accounts: [accounts.to_account_metas(Some(true))].concat(),
            data: instruction_data.data(),
        };
        let result = rpc
            .create_and_send_transaction(&[instruction], &payer.pubkey(), &[&payer])
            .await;
        let expected_error_code =
            account_compression::errors::AccountCompressionErrorCode::InvalidAuthority.into();

        assert_rpc_error(result, 0, expected_error_code).unwrap();
    }
}

pub async fn register_program(
    rpc: &mut ProgramTestRpcConnection,
    authority: &Keypair,
    program_id_keypair: &Keypair,
    group_account: &Pubkey,
) -> Result<Pubkey, RpcError> {
    let registered_program_pda = Pubkey::find_program_address(
        &[program_id_keypair.pubkey().to_bytes().as_slice()],
        &account_compression::ID,
    )
    .0;

    let accounts = account_compression::accounts::RegisterProgramToGroup {
        authority: authority.pubkey(),
        program_to_be_registered: program_id_keypair.pubkey(),
        system_program: system_program::ID,
        registered_program_pda,
        group_authority_pda: *group_account,
    };
    let instruction = Instruction {
        program_id: account_compression::ID,
        accounts: accounts.to_account_metas(Some(true)),
        data: account_compression::instruction::RegisterProgramToGroup {}.data(),
    };

    rpc.create_and_send_transaction(
        &[instruction],
        &authority.pubkey(),
        &[authority, program_id_keypair],
    )
    .await?;

    Ok(registered_program_pda)
}
