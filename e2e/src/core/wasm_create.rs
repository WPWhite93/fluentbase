use crate::core::utils::TestingContext;
use crate::helpers::wasm2rwasm;
use fluentbase_codec::Encoder;
use fluentbase_core::{
    helpers::{calc_create2_address, calc_create_address},
    Account,
};
use fluentbase_runtime::{DefaultEmptyRuntimeDatabase, RuntimeContext};
use fluentbase_sdk::LowLevelSDK;
use fluentbase_sdk::{
    CoreInput, WasmCallMethodInput, WasmCreateMethodInput, WASM_CALL_METHOD_ID,
    WASM_CREATE_METHOD_ID,
};
use fluentbase_types::{address, Address, Bytes, ExitCode, IJournaledTrie, B256, STATE_MAIN, U256};
use hex_literal::hex;

#[test]
fn test_wasm_create() {
    let caller_address = address!("000000000000000000000000000000000000000c");
    let caller_account = Account {
        address: caller_address,
        balance: U256::from_be_slice(1000000000u128.to_be_bytes().as_slice()),
        ..Default::default()
    };

    // let wcl_address = address!("0000000000000000000000000000000000000777");
    // let wcl_wasm_bytecode = include_bytes!("../../../crates/contracts/assets/wcl_contract.wasm");
    // let wcl_account = Account {
    //     address: wcl_address,
    //     source_code_size: wcl_wasm_bytecode.len() as u64,
    //     source_code_hash: keccak_hash::keccak(wcl_wasm_bytecode).0.into(),
    //     aot_code_size: 0,
    //     aot_code_hash: Default::default(),
    //     ..Default::default()
    // };

    let expected_contract_address = calc_create_address(&caller_address, caller_account.nonce);
    let block_coinbase: Address = address!("0000000000000000000000000000000000000012");

    let wasm_bytecode = include_bytes!("../../../examples/bin/greeting.wasm");

    let create_value = B256::left_padding_from(&hex!("1000"));
    let gas_limit: u64 = 10_000_000;
    let method_input = WasmCreateMethodInput {
        bytecode: wasm_bytecode.into(),
        value: create_value.into(),
        gas_limit,
        salt: None,
    };
    let core_input = CoreInput::new(WASM_CREATE_METHOD_ID, method_input);
    let core_input_vec = core_input.encode_to_vec(0);

    const IS_RUNTIME: bool = true;
    let contract_wasm_binary = include_bytes!("../../../crates/contracts/assets/wcl_contract.wasm");
    let contract_rwasm_binary = wasm2rwasm(contract_wasm_binary).unwrap();

    let mut runtime_ctx = RuntimeContext::<DefaultEmptyRuntimeDatabase>::new(contract_rwasm_binary)
        .with_jzkt(LowLevelSDK::with_default_jzkt())
        .with_state(STATE_MAIN);
    let mut test_ctx = TestingContext::<IS_RUNTIME>::new();
    let jzkt = runtime_ctx.jzkt();
    test_ctx.try_add_account(&caller_account);
    test_ctx
        .contract_input_wrapper
        .set_journal_checkpoint(jzkt.checkpoint().to_u64())
        .set_contract_input(Bytes::copy_from_slice(&core_input_vec))
        .set_contract_caller(caller_address)
        .set_block_coinbase(block_coinbase)
        .set_tx_caller(caller_address);
    test_ctx.apply_ctx(&mut runtime_ctx);

    let output = test_ctx.run_rwasm_with_input(runtime_ctx, false, gas_limit);
    assert_eq!(ExitCode::Ok.into_i32(), output.exit_code);
    assert!(output.output.len() > 0);
    let contract_address = Address::from_slice(&output.output);
    assert_eq!(expected_contract_address, contract_address);

    {
        let mut test_ctx = TestingContext::<{ !IS_RUNTIME }>::new();
        test_ctx.apply_ctx();
        let _account = Account::new_from_jzkt(contract_address);
        // assert_eq!(236, account.load_source_bytecode().len());
        // assert_eq!(479, account.load_rwasm_bytecode().len());
    }
}

#[test]
fn test_wasm_create2() {
    let caller_address = address!("000000000000000000000000000000000000000c");
    let caller_account = Account {
        address: caller_address,
        balance: U256::from_be_slice(1000000000u128.to_be_bytes().as_slice()),
        ..Default::default()
    };

    let wasm_bytecode = include_bytes!("../../../examples/bin/greeting.wasm");
    let mut wasm_bytecode_hash = B256::default();
    keccak_hash::keccak_256(wasm_bytecode.as_ref(), wasm_bytecode_hash.as_mut_slice());

    let create_value = B256::left_padding_from(&hex!("1000"));
    let salt = B256::left_padding_from(&hex!("3749269486238462"));
    let gas_limit: u64 = 10_000_000;
    let method_input = WasmCreateMethodInput {
        bytecode: wasm_bytecode.into(),
        value: create_value.into(),
        gas_limit,
        salt: Some(salt.into()),
    };
    let core_input = CoreInput::new(WASM_CREATE_METHOD_ID, method_input);
    let core_input_vec = core_input.encode_to_vec(0);

    let expected_contract_address =
        calc_create2_address(&caller_address, &salt.into(), &wasm_bytecode_hash);

    const IS_RUNTIME: bool = true;
    let contract_wasm_binary = include_bytes!("../../../crates/contracts/assets/wcl_contract.wasm");
    let contract_rwasm_binary = wasm2rwasm(contract_wasm_binary.as_slice()).unwrap();
    let mut runtime_ctx = RuntimeContext::<DefaultEmptyRuntimeDatabase>::new(contract_rwasm_binary)
        .with_jzkt(LowLevelSDK::with_default_jzkt())
        .with_state(STATE_MAIN);
    let mut test_ctx = TestingContext::<IS_RUNTIME>::new();
    test_ctx
        .try_add_account(&caller_account)
        .contract_input_wrapper
        .set_journal_checkpoint(runtime_ctx.jzkt().checkpoint().to_u64())
        .set_contract_input(Bytes::copy_from_slice(&core_input_vec))
        .set_contract_caller(caller_address)
        .set_tx_caller(caller_address);
    test_ctx.apply_ctx(&mut runtime_ctx);

    let output = test_ctx.run_rwasm_with_input(runtime_ctx, false, gas_limit);
    assert_eq!(ExitCode::Ok.into_i32(), output.exit_code);
    assert!(output.output.len() > 0);
    let contract_address = Address::from_slice(&output.output);
    assert_eq!(expected_contract_address, contract_address);

    {
        let mut test_ctx = TestingContext::<{ !IS_RUNTIME }>::new();
        test_ctx.apply_ctx();
        let _account = Account::new_from_jzkt(contract_address);
        // assert_eq!(236, account.load_source_bytecode().len());
        // assert_eq!(479, account.load_rwasm_bytecode().len());
    }
}

#[test]
fn test_wasm_call_after_create() {
    let caller_address = address!("000000000000000000000000000000000000000c");
    let caller_account = Account {
        address: caller_address,
        balance: U256::from_be_slice(1000000000u128.to_be_bytes().as_slice()),
        ..Default::default()
    };

    const IS_RUNTIME: bool = true;
    let wcl_contract_wasm = include_bytes!("../../../crates/contracts/assets/wcl_contract.wasm");
    let wcl_contract_rwasm = wasm2rwasm(wcl_contract_wasm.as_slice()).unwrap();
    let block_coinbase: Address = address!("0000000000000000000000000000000000000012");
    let gas_limit: u64 = 10_000_000;
    let create_value = B256::left_padding_from(&hex!("1000"));
    let call_value = B256::left_padding_from(&hex!("00"));
    let deploy_wasm = include_bytes!("../../../examples/bin/greeting.wasm");

    let (jzkt, deployed_contract_address) = {
        let expected_contract_address = calc_create_address(&caller_address, caller_account.nonce);
        let method_input = WasmCreateMethodInput {
            bytecode: deploy_wasm.into(),
            value: create_value.into(),
            gas_limit,
            salt: None,
        };
        let core_input = CoreInput::new(WASM_CREATE_METHOD_ID, method_input);
        let core_input_vec = core_input.encode_to_vec(0);

        let mut runtime_ctx =
            RuntimeContext::<DefaultEmptyRuntimeDatabase>::new(wcl_contract_rwasm.clone())
                .with_jzkt(LowLevelSDK::with_default_jzkt())
                .with_state(STATE_MAIN);
        let mut test_ctx = TestingContext::<IS_RUNTIME>::new();
        let jzkt = runtime_ctx.jzkt().clone();
        test_ctx
            .try_add_account(&caller_account)
            .contract_input_wrapper
            .set_journal_checkpoint(runtime_ctx.jzkt().checkpoint().to_u64())
            .set_contract_caller(caller_address)
            .set_contract_input(Bytes::copy_from_slice(&core_input_vec))
            .set_block_coinbase(block_coinbase);
        test_ctx.apply_ctx(&mut runtime_ctx);

        let output = test_ctx.run_rwasm_with_input(runtime_ctx, false, gas_limit);
        assert_eq!(ExitCode::Ok.into_i32(), output.exit_code);
        assert!(output.output.len() > 0);
        let contract_address = Address::from_slice(&output.output);
        assert_eq!(expected_contract_address, contract_address);

        {
            let mut test_ctx = TestingContext::<{ !IS_RUNTIME }>::new();
            test_ctx.apply_ctx();
            let _account = Account::new_from_jzkt(contract_address);
            // assert_eq!(236, account.load_source_bytecode().len());
            // assert_eq!(479, account.load_rwasm_bytecode().len());
        }

        (jzkt, contract_address)
    };

    let _jzkt = {
        let ecl_method_input = WasmCallMethodInput {
            callee: deployed_contract_address,
            value: call_value.into(),
            input: Default::default(),
            gas_limit,
        };
        let ecl_core_input = CoreInput::new(WASM_CALL_METHOD_ID, ecl_method_input);
        let ecl_core_input_vec = ecl_core_input.encode_to_vec(0);

        let mut runtime_ctx =
            RuntimeContext::<DefaultEmptyRuntimeDatabase>::new(wcl_contract_rwasm.clone())
                .with_jzkt(jzkt.clone());
        let mut test_ctx = TestingContext::<IS_RUNTIME>::new();
        test_ctx
            .try_add_account(&caller_account)
            .contract_input_wrapper
            .set_journal_checkpoint(runtime_ctx.jzkt().checkpoint().to_u64())
            .set_contract_address(deployed_contract_address)
            .set_contract_input(Bytes::copy_from_slice(&ecl_core_input_vec))
            .set_contract_caller(caller_address);
        test_ctx.apply_ctx(&mut runtime_ctx);

        let output = test_ctx.run_rwasm_with_input(runtime_ctx, false, gas_limit);

        // println!("total opcodes spent: {}", output_res.tracer().logs.len());
        assert_eq!(ExitCode::Ok.into_i32(), output.exit_code);
        assert!(output.output.len() > 0);
        assert_eq!("Hello, World".as_bytes(), output.output.as_slice());

        jzkt
    };
}
