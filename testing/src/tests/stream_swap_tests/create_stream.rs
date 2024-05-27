#[cfg(test)]
use crate::helpers::{
    mock_messages::{get_create_stream_msg, get_factory_inst_msg},
    setup::{setup, SetupResponse},
};
use cosmwasm_std::coin;
use cw_multi_test::Executor;
use cw_streamswap::{threshold::ThresholdError, ContractError as StreamSwapError};
use cw_streamswap_factory::{
    error::ContractError as FactoryError, msg::QueryMsg, payment_checker::CustomPaymentError,
};
#[test]
fn create_stream_failed_name_url_checks() {
    let SetupResponse {
        mut app,
        test_accounts,
        stream_swap_code_id,
        stream_swap_factory_code_id,
    } = setup();

    let msg = get_factory_inst_msg(stream_swap_code_id, &test_accounts);
    let factory_address = app
        .instantiate_contract(
            stream_swap_factory_code_id,
            test_accounts.admin.clone(),
            &msg,
            &[],
            "Factory".to_string(),
            None,
        )
        .unwrap();
    // Failed name checks
    // Name too short
    let create_stream_msg = get_create_stream_msg(
        "s",
        None,
        &test_accounts.creator.to_string(),
        coin(100, "out_denom"),
        "in_denom",
        app.block_info().height + 100,
        app.block_info().height + 200,
        None,
    );

    let res = app
        .execute_contract(
            test_accounts.creator.clone(),
            factory_address.clone(),
            &create_stream_msg,
            &[coin(100, "fee_token"), coin(100, "out_denom")],
        )
        .unwrap_err();

    let err = res.source().unwrap().source().unwrap();
    let error = err.downcast_ref::<StreamSwapError>().unwrap();
    assert_eq!(*error, StreamSwapError::StreamNameTooShort {});
    // Name too long
    let long_name = "a".repeat(65);
    let create_stream_msg = get_create_stream_msg(
        &long_name,
        None,
        &test_accounts.creator.to_string(),
        coin(100, "out_denom"),
        "in_denom",
        app.block_info().height + 100,
        app.block_info().height + 200,
        None,
    );

    let res = app
        .execute_contract(
            test_accounts.creator.clone(),
            factory_address.clone(),
            &create_stream_msg,
            &[coin(100, "fee_token"), coin(100, "out_denom")],
        )
        .unwrap_err();

    let err = res.source().unwrap().source().unwrap();
    let error = err.downcast_ref::<StreamSwapError>().unwrap();
    assert_eq!(*error, StreamSwapError::StreamNameTooLong {});

    // Invalid name
    let create_stream_msg = get_create_stream_msg(
        "abc~ß",
        None,
        &test_accounts.creator.to_string(),
        coin(100, "out_denom"),
        "in_denom",
        app.block_info().height + 100,
        app.block_info().height + 200,
        None,
    );

    let res = app
        .execute_contract(
            test_accounts.creator.clone(),
            factory_address.clone(),
            &create_stream_msg,
            &[coin(100, "fee_token"), coin(100, "out_denom")],
        )
        .unwrap_err();

    let err = res.source().unwrap().source().unwrap();
    let error = err.downcast_ref::<StreamSwapError>().unwrap();
    assert_eq!(*error, StreamSwapError::InvalidStreamName {});

    // Failed url checks
    // URL too short
    let create_stream_msg = get_create_stream_msg(
        "stream",
        Some("a".to_string()),
        &test_accounts.creator.to_string(),
        coin(100, "out_denom"),
        "in_denom",
        app.block_info().height + 100,
        app.block_info().height + 200,
        None,
    );

    let res = app
        .execute_contract(
            test_accounts.creator.clone(),
            factory_address.clone(),
            &create_stream_msg,
            &[coin(100, "fee_token"), coin(100, "out_denom")],
        )
        .unwrap_err();

    let err = res.source().unwrap().source().unwrap();
    let error = err.downcast_ref::<StreamSwapError>().unwrap();

    assert_eq!(*error, StreamSwapError::StreamUrlTooShort {});

    // URL too long
    let long_url = "a".repeat(256);
    let create_stream_msg = get_create_stream_msg(
        "stream",
        Some(long_url),
        &test_accounts.creator.to_string(),
        coin(100, "out_denom"),
        "in_denom",
        app.block_info().height + 100,
        app.block_info().height + 200,
        Some(100),
    );

    let res = app
        .execute_contract(
            test_accounts.creator.clone(),
            factory_address.clone(),
            &create_stream_msg,
            &[coin(100, "fee_token"), coin(100, "out_denom")],
        )
        .unwrap_err();

    let err = res.source().unwrap().source().unwrap();
    let error = err.downcast_ref::<StreamSwapError>().unwrap();
    assert_eq!(*error, StreamSwapError::StreamUrlTooLong {});
}

#[test]
fn create_stream_failed_fund_checks() {
    let SetupResponse {
        mut app,
        test_accounts,
        stream_swap_code_id,
        stream_swap_factory_code_id,
    } = setup();

    let msg = get_factory_inst_msg(stream_swap_code_id, &test_accounts);
    let factory_address = app
        .instantiate_contract(
            stream_swap_factory_code_id,
            test_accounts.admin.clone(),
            &msg,
            &[],
            "Factory".to_string(),
            None,
        )
        .unwrap();

    // Non permissioned in denom
    let create_stream_msg = get_create_stream_msg(
        "stream",
        None,
        &test_accounts.creator.to_string(),
        coin(100, "out_denom"),
        "invalid_in_denom",
        app.block_info().height + 100,
        app.block_info().height + 200,
        None,
    );
    let res = app
        .execute_contract(
            test_accounts.creator.clone(),
            factory_address.clone(),
            &create_stream_msg,
            &[coin(100, "fee_token"), coin(100, "out_denom")],
        )
        .unwrap_err();
    let err = res.source().unwrap();
    let error = err.downcast_ref::<FactoryError>().unwrap();
    assert_eq!(*error, FactoryError::InDenomIsNotAccepted {});

    // Same in and out denom
    let create_stream_msg = get_create_stream_msg(
        "stream",
        None,
        &test_accounts.creator.to_string(),
        coin(100, "in_denom"),
        "in_denom",
        app.block_info().height + 100,
        app.block_info().height + 200,
        None,
    );
    let res = app
        .execute_contract(
            test_accounts.creator.clone(),
            factory_address.clone(),
            &create_stream_msg,
            &[coin(100, "fee_token"), coin(100, "in_denom")],
        )
        .unwrap_err();

    let err = res.source().unwrap().source().unwrap();
    let error = err.downcast_ref::<StreamSwapError>().unwrap();
    assert_eq!(*error, StreamSwapError::SameDenomOnEachSide {});

    // Zero out supply
    let create_stream_msg = get_create_stream_msg(
        "stream",
        None,
        &test_accounts.creator.to_string(),
        coin(0, "out_denom"),
        "in_denom",
        app.block_info().height + 100,
        app.block_info().height + 200,
        Some(100),
    );

    let res = app
        .execute_contract(
            test_accounts.creator.clone(),
            factory_address.clone(),
            &create_stream_msg,
            &[coin(100, "fee_token")],
        )
        .unwrap_err();

    let err = res.source().unwrap().source().unwrap();
    let error = err.downcast_ref::<StreamSwapError>().unwrap();
    assert_eq!(*error, StreamSwapError::ZeroOutSupply {});

    // No funds sent
    let create_stream_msg = get_create_stream_msg(
        "stream",
        None,
        &test_accounts.creator.to_string(),
        coin(100, "out_denom"),
        "in_denom",
        app.block_info().height + 100,
        app.block_info().height + 200,
        None,
    );

    let res = app
        .execute_contract(
            test_accounts.creator.clone(),
            factory_address.clone(),
            &create_stream_msg,
            &[coin(100, "fee_token")],
        )
        .unwrap_err();

    let err = res.source().unwrap();
    let error = err.downcast_ref::<FactoryError>().unwrap();
    assert_eq!(
        *error,
        FactoryError::CustomPayment(CustomPaymentError::InsufficientFunds {
            expected: [coin(100, "fee_token"), coin(100, "out_denom")].to_vec(),
            actual: [coin(100, "fee_token")].to_vec()
        })
    );

    // Insufficient fee
    let create_stream_msg = get_create_stream_msg(
        "stream",
        None,
        &test_accounts.creator.to_string(),
        coin(100, "out_denom"),
        "in_denom",
        app.block_info().height + 100,
        app.block_info().height + 200,
        None,
    );

    let res = app
        .execute_contract(
            test_accounts.creator.clone(),
            factory_address.clone(),
            &create_stream_msg,
            &[coin(99, "fee_token"), coin(100, "out_denom")],
        )
        .unwrap_err();

    let err = res.source().unwrap();
    let error = err.downcast_ref::<FactoryError>().unwrap();
    assert_eq!(
        *error,
        FactoryError::CustomPayment(CustomPaymentError::InsufficientFunds {
            expected: [coin(100, "fee_token"), coin(100, "out_denom")].to_vec(),
            actual: [coin(99, "fee_token"), coin(100, "out_denom")].to_vec()
        })
    );

    // Extra funds sent
    let create_stream_msg = get_create_stream_msg(
        "stream",
        None,
        &test_accounts.creator.to_string(),
        coin(100, "out_denom"),
        "in_denom",
        app.block_info().height + 100,
        app.block_info().height + 200,
        None,
    );

    let res = app
        .execute_contract(
            test_accounts.creator.clone(),
            factory_address.clone(),
            &create_stream_msg,
            &[
                coin(100, "fee_token"),
                coin(100, "out_denom"),
                coin(100, "random"),
            ],
        )
        .unwrap_err();

    let err = res.source().unwrap();
    let error = err.downcast_ref::<FactoryError>().unwrap();
    assert_eq!(
        *error,
        FactoryError::CustomPayment(CustomPaymentError::InsufficientFunds {
            expected: [coin(100, "fee_token"), coin(100, "out_denom")].to_vec(),
            actual: [
                coin(100, "fee_token"),
                coin(100, "out_denom"),
                coin(100, "random")
            ]
            .to_vec()
        })
    );

    // Threshold zero
    let create_stream_msg = get_create_stream_msg(
        "stream",
        None,
        &test_accounts.creator.to_string(),
        coin(100, "out_denom"),
        "in_denom",
        app.block_info().height + 100,
        app.block_info().height + 200,
        Some(0),
    );

    let res = app
        .execute_contract(
            test_accounts.creator.clone(),
            factory_address.clone(),
            &create_stream_msg,
            &[coin(100, "fee_token"), coin(100, "out_denom")],
        )
        .unwrap_err();

    let err = res.source().unwrap().source().unwrap();
    let error = err.downcast_ref::<StreamSwapError>().unwrap();
    assert_eq!(
        *error,
        StreamSwapError::ThresholdError(ThresholdError::ThresholdZero {})
    );
}

#[test]
fn create_stream_failed_duration_checks() {
    let SetupResponse {
        mut app,
        test_accounts,
        stream_swap_code_id,
        stream_swap_factory_code_id,
    } = setup();

    let msg = get_factory_inst_msg(stream_swap_code_id, &test_accounts);
    let factory_address = app
        .instantiate_contract(
            stream_swap_factory_code_id,
            test_accounts.admin.clone(),
            &msg,
            &[],
            "Factory".to_string(),
            None,
        )
        .unwrap();

    // End block < start block
    let create_stream_msg = get_create_stream_msg(
        "stream",
        None,
        &test_accounts.creator.to_string(),
        coin(100, "out_denom"),
        "in_denom",
        app.block_info().height + 200,
        app.block_info().height + 100,
        None,
    );
    let res = app
        .execute_contract(
            test_accounts.creator.clone(),
            factory_address.clone(),
            &create_stream_msg,
            &[coin(100, "fee_token"), coin(100, "out_denom")],
        )
        .unwrap_err();
    let err = res.source().unwrap();
    let error = err.downcast_ref::<FactoryError>().unwrap();
    assert_eq!(*error, FactoryError::StreamInvalidEndBlock {});

    // Now block > start block
    let create_stream_msg = get_create_stream_msg(
        "stream",
        None,
        &test_accounts.creator.to_string(),
        coin(100, "out_denom"),
        "in_denom",
        app.block_info().height - 1,
        app.block_info().height + 200,
        None,
    );

    let res = app
        .execute_contract(
            test_accounts.creator.clone(),
            factory_address.clone(),
            &create_stream_msg,
            &[coin(100, "fee_token"), coin(100, "out_denom")],
        )
        .unwrap_err();
    let err = res.source().unwrap();
    let error = err.downcast_ref::<FactoryError>().unwrap();
    assert_eq!(*error, FactoryError::StreamInvalidStartBlock {});

    // Stream duration too short
    let create_stream_msg = get_create_stream_msg(
        "stream",
        None,
        &test_accounts.creator.to_string(),
        coin(100, "out_denom"),
        "in_denom",
        app.block_info().height + 1,
        app.block_info().height + 10,
        None,
    );

    let res = app
        .execute_contract(
            test_accounts.creator.clone(),
            factory_address.clone(),
            &create_stream_msg,
            &[coin(100, "fee_token"), coin(100, "out_denom")],
        )
        .unwrap_err();

    let err = res.source().unwrap();
    let error = err.downcast_ref::<FactoryError>().unwrap();
    assert_eq!(*error, FactoryError::StreamDurationTooShort {});

    // Stream starts too soon
    let create_stream_msg = get_create_stream_msg(
        "stream",
        None,
        &test_accounts.creator.to_string(),
        coin(100, "out_denom"),
        "in_denom",
        app.block_info().height + 1,
        app.block_info().height + 200,
        None,
    );

    let res = app
        .execute_contract(
            test_accounts.creator.clone(),
            factory_address.clone(),
            &create_stream_msg,
            &[coin(100, "fee_token"), coin(100, "out_denom")],
        )
        .unwrap_err();

    let err = res.source().unwrap();
    let error = err.downcast_ref::<FactoryError>().unwrap();
    assert_eq!(*error, FactoryError::StreamStartsTooSoon {});
}

#[test]
fn create_stream_happy_path() {
    let SetupResponse {
        mut app,
        test_accounts,
        stream_swap_code_id,
        stream_swap_factory_code_id,
    } = setup();

    let msg = get_factory_inst_msg(stream_swap_code_id, &test_accounts);
    let factory_address = app
        .instantiate_contract(
            stream_swap_factory_code_id,
            test_accounts.admin.clone(),
            &msg,
            &[],
            "Factory".to_string(),
            None,
        )
        .unwrap();

    let create_stream_msg = get_create_stream_msg(
        "stream",
        None,
        &test_accounts.creator.to_string(),
        coin(100, "out_denom"),
        "in_denom",
        app.block_info().height + 100,
        app.block_info().height + 200,
        Some(100),
    );

    let _res = app
        .execute_contract(
            test_accounts.creator.clone(),
            factory_address.clone(),
            &create_stream_msg,
            &[coin(100, "fee_token"), coin(100, "out_denom")],
        )
        .unwrap();

    // Query stream with id
    let query_msg = QueryMsg::LastStreamId {};
    let res: u32 = app
        .wrap()
        .query_wasm_smart(factory_address, &query_msg)
        .unwrap();
    assert_eq!(res, 1);
}