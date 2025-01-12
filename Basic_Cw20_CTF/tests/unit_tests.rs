use cosmwasm_std::{
    testing::*,
    Addr,
};
use cw_multi_test::IntoAddr;
use Basic_CW_20::{
    contract::{instantiate, query, execute},
    msg::{InstantiateMsg, ExecuteMsg, QueryMsg},
    error::ContractError,
    state::*,
};
    use cosmwasm_std::{from_json, OwnedDeps};

    const INITIAL_SUPPLY: u128 = 1_000_000;
    const MAX_SUPPLY: u128 = 2_000_000;
    struct TestAddresses {
        owner: Addr,
        user1: Addr,
        user2: Addr,
        fee_collector: Addr,
    }

    fn setup_contract() -> (OwnedDeps<MockStorage, MockApi, MockQuerier>, TestAddresses) {
        let mut deps = mock_dependencies();
        
        let addresses = TestAddresses {
            owner : "owner".into_addr(),
            user1 : "user1".into_addr(),
            user2 : "user2".into_addr(),
            fee_collector : "fee_collector".into_addr(),
        };

        let msg = InstantiateMsg {
            name: "Test Token".to_string(),
            symbol: "TEST".to_string(),
            decimals: 6,
            initial_supply: INITIAL_SUPPLY,
            max_supply: MAX_SUPPLY,
            owner: addresses.owner.to_string(),
            fee_collector: addresses.fee_collector.to_string(),
            fee_rate: 1, // 1% fee
        };

        let info = mock_info(addresses.owner.as_str(), &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        (deps, addresses)
    }

    fn query_balance(deps: &OwnedDeps<MockStorage, MockApi, MockQuerier>, address: &Addr) -> u128 {
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Balance { address: address.to_string() },
        )
        .unwrap();
        from_json(&res).unwrap()
    }

    #[test]
    fn proper_initialization() {
        let (deps, addresses) = setup_contract();
        
        let res = query(deps.as_ref(), mock_env(), QueryMsg::TokenInfo {}).unwrap();
        let token_info: TokenInfo = from_json(&res).unwrap();
        
        assert_eq!(token_info.name, "Test Token");
        assert_eq!(token_info.symbol, "TEST");
        assert_eq!(token_info.decimals, 6);
        assert_eq!(token_info.circulating_supply, INITIAL_SUPPLY);
        assert_eq!(token_info.max_supply, MAX_SUPPLY);
        assert_eq!(token_info.owner, addresses.owner);
        assert_eq!(token_info.fee_collector, addresses.fee_collector);
        
        let balance: u128 = query_balance(&deps, &addresses.owner);
        assert_eq!(balance, INITIAL_SUPPLY);
    }


    #[test]
    fn test_transfer() {
        let (mut deps, addresses) = setup_contract();
        let balance: u128 = query_balance(&deps, &addresses.owner);
        assert_eq!(balance, INITIAL_SUPPLY);

        let transfer_amount = 100_000u128;

        let info = mock_info(addresses.owner.as_str(), &[]);
        let msg = ExecuteMsg::Transfer {
            recipient: addresses.user1.to_string(),
            amount: transfer_amount,
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(res.attributes.len(), 5);

        // Check recipient balance (amount - 1% fee)
        let balance: u128 = query_balance(&deps, &addresses.user1);
        assert_eq!(balance, 99_000);

        // Check fee collector balance
        let fee_balance: u128 = query_balance(&deps, &addresses.fee_collector);
        assert_eq!(fee_balance, 1_000);
    }

    #[test]
    fn test_mint() {
        let (mut deps, addresses) = setup_contract();
        let mint_amount = 500_000u128;

        let info = mock_info(addresses.owner.as_str(), &[]);
        let msg = ExecuteMsg::Mint {
            recipient: addresses.user1.to_string(),
            amount: mint_amount,
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(res.attributes.len(), 3);

        // Verify recipient balance
        let balance: u128 = query_balance(&deps, &addresses.user1);
        assert_eq!(balance, mint_amount);

        // Verify updated total supply
        let res = query(deps.as_ref(), mock_env(), QueryMsg::TokenInfo {}).unwrap();
        let token_info: TokenInfo = from_json(&res).unwrap();
        assert_eq!(token_info.circulating_supply, INITIAL_SUPPLY + mint_amount);
    }

    #[test]
    fn test_burn() {
        let (mut deps, addresses) = setup_contract();
        let burn_amount = 100_000u128;

        let info = mock_info(addresses.owner.as_str(), &[]);
        let msg = ExecuteMsg::Burn { amount: burn_amount };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(res.attributes.len(), 3);

        // Verify updated balance
        let balance: u128 = query_balance(&deps, &addresses.owner);
        assert_eq!(balance, INITIAL_SUPPLY - burn_amount);

        // Verify updated total supply
        let res = query(deps.as_ref(), mock_env(), QueryMsg::TokenInfo {}).unwrap();
        let token_info: TokenInfo = from_json(&res).unwrap();
        assert_eq!(token_info.circulating_supply, INITIAL_SUPPLY - burn_amount);
    }

    #[test]
    fn test_allowances_and_transfer_from() {
        let (mut deps, addresses) = setup_contract();
        let allowance_amount = 100_000u128;

        // Owner increases allowance for user1
        let info = mock_info(addresses.owner.as_str(), &[]);
        let msg = ExecuteMsg::IncreaseAllowance {
            owner: addresses.owner.to_string(),
            spender:addresses.user1.to_string(),
            amount: allowance_amount,
        };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        assert_eq!(res.attributes.len(), 4);

        // User1 transfers from owner to user2
        let transfer_amount = 50_000u128;
        let info = mock_info(addresses.user1.as_str(), &[]);

        let msg = ExecuteMsg::TransferFrom {
            owner: addresses.owner.to_string(),
            recipient: addresses.user2.to_string(),
            amount: transfer_amount,
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(res.attributes.len(), 6);

        // Verify user2 balance (amount - 1% fee)
        let balance: u128 = query_balance(&deps, &addresses.user2);
        assert_eq!(balance, 49_500); // 50_000 - 1% fee
    }

    #[test]
    fn test_insufficient_funds() {
        let (mut deps, addresses) = setup_contract();
        let excess_amount = INITIAL_SUPPLY + 1;

        let info = mock_info(addresses.owner.as_str(), &[]);
        let msg = ExecuteMsg::Transfer {
            recipient: addresses.user1.to_string(),
            amount: excess_amount,
        };
        let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
        assert!(matches!(err, ContractError::InsufficientFunds { amount: _, balance: _ }));
    }

    #[test]
    fn test_max_supply_mint() {
        let (mut deps, addresses) = setup_contract();
        let excess_mint = MAX_SUPPLY - INITIAL_SUPPLY + 1;

        let info = mock_info(addresses.owner.as_str(), &[]);
        let msg = ExecuteMsg::Mint {
            recipient: addresses.user1.to_string(),
            amount: excess_mint,
        };
        let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
        assert!(matches!(err, ContractError::MaxSupplyReached { max_supply: _ }));
    }

  