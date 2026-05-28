use soroban_sdk::{Env, String, Vec};
use accountability_vault::{Contract, Error, Milestone};
use soroban_sdk::{Address, token::TokenClient};

#[test]
fn test_create_vault_success() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);

    let vault_id = String::from_str(&env, "vault_1");
    let creator = String::from_str(&env, "creator_address");
    let verifier = String::from_str(&env, "verifier_address");
    let success_destination = String::from_str(&env, "success_address");
    let failure_destination = String::from_str(&env, "failure_address");

    let milestones = Vec::from_array(
        &env,
        [
            Milestone {
                id: 1,
                title: String::from_str(&env, "Milestone 1"),
                amount: 100,
                due_date: 1000,
            },
            Milestone {
                id: 2,
                title: String::from_str(&env, "Milestone 2"),
                amount: 200,
                due_date: 2000,
            },
            Milestone {
                id: 3,
                title: String::from_str(&env, "Milestone 3"),
                amount: 300,
                due_date: 3000,
            },
        ],
    );

    let result = Contract::create_vault(
        env,
        vault_id.clone(),
        creator,
        600, // Total amount matches sum of milestones
        verifier,
        success_destination,
        failure_destination,
        milestones.clone(),
    );

    assert!(result.is_ok());
    let vault = result.unwrap();
    assert_eq!(vault.id, vault_id);
    assert_eq!(vault.amount, 600);
    assert_eq!(vault.milestones.len(), 3);
}

#[test]
fn test_create_vault_invalid_amount_negative() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);

    let vault_id = String::from_str(&env, "vault_1");
    let creator = String::from_str(&env, "creator_address");
    let verifier = String::from_str(&env, "verifier_address");
    let success_destination = String::from_str(&env, "success_address");
    let failure_destination = String::from_str(&env, "failure_address");

    let milestones = Vec::from_array(
        &env,
        [Milestone {
            id: 1,
            title: String::from_str(&env, "Milestone 1"),
            amount: 100,
            due_date: 1000,
        }],
    );

    let result = Contract::create_vault(
        env,
        vault_id,
        creator,
        -100, // Negative amount
        verifier,
        success_destination,
        failure_destination,
        milestones,
    );

    assert_eq!(result, Err(Error::InvalidAmount));
}

#[test]
fn test_create_vault_invalid_amount_zero() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);

    let vault_id = String::from_str(&env, "vault_1");
    let creator = String::from_str(&env, "creator_address");
    let verifier = String::from_str(&env, "verifier_address");
    let success_destination = String::from_str(&env, "success_address");
    let failure_destination = String::from_str(&env, "failure_address");

    let milestones = Vec::from_array(
        &env,
        [Milestone {
            id: 1,
            title: String::from_str(&env, "Milestone 1"),
            amount: 100,
            due_date: 1000,
        }],
    );

    let result = Contract::create_vault(
        env,
        vault_id,
        creator,
        0, // Zero amount
        verifier,
        success_destination,
        failure_destination,
        milestones,
    );

    assert_eq!(result, Err(Error::InvalidAmount));
}

#[test]
fn test_create_vault_amount_mismatch() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);

    let vault_id = String::from_str(&env, "vault_1");
    let creator = String::from_str(&env, "creator_address");
    let verifier = String::from_str(&env, "verifier_address");
    let success_destination = String::from_str(&env, "success_address");
    let failure_destination = String::from_str(&env, "failure_address");

    let milestones = Vec::from_array(
        &env,
        [
            Milestone {
                id: 1,
                title: String::from_str(&env, "Milestone 1"),
                amount: 100,
                due_date: 1000,
            },
            Milestone {
                id: 2,
                title: String::from_str(&env, "Milestone 2"),
                amount: 200,
                due_date: 2000,
            },
        ],
    );

    let result = Contract::create_vault(
        env,
        vault_id,
        creator,
        500, // Total amount doesn't match sum (300)
        verifier,
        success_destination,
        failure_destination,
        milestones,
    );

    assert_eq!(result, Err(Error::AmountMismatch));
}

#[test]
fn test_create_vault_overflow_extreme_amounts() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);

    let vault_id = String::from_str(&env, "vault_overflow");
    let creator = String::from_str(&env, "creator_address");
    let verifier = String::from_str(&env, "verifier_address");
    let success_destination = String::from_str(&env, "success_address");
    let failure_destination = String::from_str(&env, "failure_address");

    // Use extreme amounts that will cause i128 overflow when summed
    // i128 max is approximately 1.7e19, so using values near half of that
    let half_max = i128::MAX / 2;
    
    let milestones = Vec::from_array(
        &env,
        [
            Milestone {
                id: 1,
                title: String::from_str(&env, "Milestone 1"),
                amount: half_max,
                due_date: 1000,
            },
            Milestone {
                id: 2,
                title: String::from_str(&env, "Milestone 2"),
                amount: half_max,
                due_date: 2000,
            },
            Milestone {
                id: 3,
                title: String::from_str(&env, "Milestone 3"),
                amount: 100, // This will cause overflow
                due_date: 3000,
            },
        ],
    );

    let result = Contract::create_vault(
        env,
        vault_id,
        creator,
        i128::MAX, // Claim total is max, but sum will overflow
        verifier,
        success_destination,
        failure_destination,
        milestones,
    );

    // Should return Overflow error instead of panicking
    assert_eq!(result, Err(Error::Overflow));
}

#[test]
fn test_create_vault_overflow_single_large_milestone() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);

    let vault_id = String::from_str(&env, "vault_overflow_single");
    let creator = String::from_str(&env, "creator_address");
    let verifier = String::from_str(&env, "verifier_address");
    let success_destination = String::from_str(&env, "success_address");
    let failure_destination = String::from_str(&env, "failure_address");

    // Create milestones that will overflow when summed
    let large_value = i128::MAX - 100;
    
    let milestones = Vec::from_array(
        &env,
        [
            Milestone {
                id: 1,
                title: String::from_str(&env, "Milestone 1"),
                amount: large_value,
                due_date: 1000,
            },
            Milestone {
                id: 2,
                title: String::from_str(&env, "Milestone 2"),
                amount: 200, // This will cause overflow
                due_date: 2000,
            },
        ],
    );

    let result = Contract::create_vault(
        env,
        vault_id,
        creator,
        i128::MAX,
        verifier,
        success_destination,
        failure_destination,
        milestones,
    );

    // Should return Overflow error instead of panicking
    assert_eq!(result, Err(Error::Overflow));
}

#[test]
fn test_create_vault_milestone_negative_amount() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);

    let vault_id = String::from_str(&env, "vault_1");
    let creator = String::from_str(&env, "creator_address");
    let verifier = String::from_str(&env, "verifier_address");
    let success_destination = String::from_str(&env, "success_address");
    let failure_destination = String::from_str(&env, "failure_address");

    let milestones = Vec::from_array(
        &env,
        [
            Milestone {
                id: 1,
                title: String::from_str(&env, "Milestone 1"),
                amount: 100,
                due_date: 1000,
            },
            Milestone {
                id: 2,
                title: String::from_str(&env, "Milestone 2"),
                amount: -50, // Negative milestone amount
                due_date: 2000,
            },
        ],
    );

    let result = Contract::create_vault(
        env,
        vault_id,
        creator,
        50,
        verifier,
        success_destination,
        failure_destination,
        milestones,
    );

    assert_eq!(result, Err(Error::InvalidAmount));
}

#[test]
fn test_create_vault_milestone_zero_amount() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);

    let vault_id = String::from_str(&env, "vault_1");
    let creator = String::from_str(&env, "creator_address");
    let verifier = String::from_str(&env, "verifier_address");
    let success_destination = String::from_str(&env, "success_address");
    let failure_destination = String::from_str(&env, "failure_address");

    let milestones = Vec::from_array(
        &env,
        [
            Milestone {
                id: 1,
                title: String::from_str(&env, "Milestone 1"),
                amount: 100,
                due_date: 1000,
            },
            Milestone {
                id: 2,
                title: String::from_str(&env, "Milestone 2"),
                amount: 0, // Zero milestone amount
                due_date: 2000,
            },
        ],
    );

    let result = Contract::create_vault(
        env,
        vault_id,
        creator,
        100,
        verifier,
        success_destination,
        failure_destination,
        milestones,
    );

    assert_eq!(result, Err(Error::InvalidAmount));
}

#[test]
fn test_create_vault_empty_milestones() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);

    let vault_id = String::from_str(&env, "vault_1");
    let creator = String::from_str(&env, "creator_address");
    let verifier = String::from_str(&env, "verifier_address");
    let success_destination = String::from_str(&env, "success_address");
    let failure_destination = String::from_str(&env, "failure_address");

    let milestones = Vec::new(&env);

    let result = Contract::create_vault(
        env,
        vault_id,
        creator,
        0, // Empty milestones should require zero amount
        verifier,
        success_destination,
        failure_destination,
        milestones,
    );

    // Empty milestones with zero amount should fail due to InvalidAmount (amount <= 0)
    assert_eq!(result, Err(Error::InvalidAmount));
}

#[test]
fn test_create_vault_large_valid_amounts() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);

    let vault_id = String::from_str(&env, "vault_large");
    let creator = String::from_str(&env, "creator_address");
    let verifier = String::from_str(&env, "verifier_address");
    let success_destination = String::from_str(&env, "success_address");
    let failure_destination = String::from_str(&env, "failure_address");

    // Use large but valid amounts that won't overflow
    let large_amount = i128::MAX / 3;
    
    let milestones = Vec::from_array(
        &env,
        [
            Milestone {
                id: 1,
                title: String::from_str(&env, "Milestone 1"),
                amount: large_amount,
                due_date: 1000,
            },
            Milestone {
                id: 2,
                title: String::from_str(&env, "Milestone 2"),
                amount: large_amount,
                due_date: 2000,
            },
            Milestone {
                id: 3,
                title: String::from_str(&env, "Milestone 3"),
                amount: large_amount,
                due_date: 3000,
            },
        ],
    );

    let total = large_amount * 3;
    let result = Contract::create_vault(
        env,
        vault_id,
        creator,
        total,
        verifier,
        success_destination,
        failure_destination,
        milestones.clone(),
    );

    assert!(result.is_ok());
    let vault = result.unwrap();
    assert_eq!(vault.amount, total);
}


#[test]
fn test_reclaim_after_settlement_success() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);

    // Creator and a zero-staked vault
    let vault_id = String::from_str(&env, "vault_reclaim");
    let creator = String::from_str(&env, "creator_address");
    let verifier = String::from_str(&env, "verifier_address");
    let success_destination = String::from_str(&env, "success_address");
    let failure_destination = String::from_str(&env, "failure_address");

    let milestones = Vec::new(&env);

    // amount == 0 indicates no staked funds (settled)
    let vault = Contract::create_vault(
        env.clone(),
        vault_id,
        creator.clone(),
        0,
        verifier,
        success_destination,
        failure_destination,
        milestones,
    ).unwrap();

    // Create a dummy token address (no real token contract needed for this unit test)
    let token_addr = Address::from_string(&String::from_str(&env, "TOKEN"));

    // Allow the test environment to authorize the creator call
    env.mock_all_auths();

    // Should succeed even if balance is zero; function should return Ok(())
    let res = Contract::reclaim_after_settlement(env, vault, token_addr);
    assert_eq!(res, Ok(()));
}


#[test]
fn test_reclaim_after_settlement_fails_if_staked_remaining() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);

    let vault_id = String::from_str(&env, "vault_reclaim2");
    let creator = String::from_str(&env, "creator_address");
    let verifier = String::from_str(&env, "verifier_address");
    let success_destination = String::from_str(&env, "success_address");
    let failure_destination = String::from_str(&env, "failure_address");

    let milestones = Vec::new(&env);

    // amount != 0 means staked funds remain
    let vault = Contract::create_vault(
        env.clone(),
        vault_id,
        creator.clone(),
        100,
        verifier,
        success_destination,
        failure_destination,
        milestones,
    ).unwrap();

    let token_addr = Address::from_string(&String::from_str(&env, "TOKEN"));

    // Authorize the caller
    env.mock_all_auths();

    let res = Contract::reclaim_after_settlement(env, vault, token_addr);
    assert_eq!(res, Err(Error::StakedRemaining));
}
