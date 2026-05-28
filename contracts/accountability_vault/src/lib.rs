#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, panic_with_error, Env, String, Vec, Address};
use soroban_sdk::token::TokenClient;

/// Error types for the accountability vault contract
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// Invalid amount provided (negative or zero)
    InvalidAmount = 1,
    /// Milestone amounts do not sum to the total vault amount
    AmountMismatch = 2,
    /// Overflow occurred during amount summation
    Overflow = 3,
    /// Vault still has staked funds and cannot be reclaimed
    StakedRemaining = 4,
    /// Vault is not in a terminal state
    NotTerminal = 5,
}

/// Milestone structure
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Milestone {
    pub id: u64,
    pub title: String,
    pub amount: i128,
    pub due_date: u64,
}

/// Vault structure
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Vault {
    pub id: String,
    pub creator: String,
    pub amount: i128,
    pub verifier: String,
    pub success_destination: String,
    pub failure_destination: String,
    pub milestones: Vec<Milestone>,
}

pub struct Contract;

#[contractimpl]
impl Contract {
    /// Creates a new accountability vault with the specified parameters.
    ///
    /// # Arguments
    /// * `vault_id` - Unique identifier for the vault
    /// * `creator` - Address of the vault creator
    /// * `amount` - Total amount to be locked in the vault
    /// * `verifier` - Address authorized to validate milestones
    /// * `success_destination` - Address to receive funds on successful completion
    /// * `failure_destination` - Address to receive funds on failure
    /// * `milestones` - Vector of milestones with individual amounts
    ///
    /// # Errors
    /// * `InvalidAmount` - If amount is negative or zero
    /// * `AmountMismatch` - If milestone amounts don't sum to total amount
    /// * `Overflow` - If milestone amount summation overflows i128
    ///
    /// # Overflow Safety
    /// This function uses checked_add for all arithmetic operations to prevent
    /// integer overflow. If overflow occurs during milestone amount summation,
    /// the function returns an Overflow error instead of panicking.
    pub fn create_vault(
        env: Env,
        vault_id: String,
        creator: String,
        amount: i128,
        verifier: String,
        success_destination: String,
        failure_destination: String,
        milestones: Vec<Milestone>,
    ) -> Result<Vault, Error> {
        // Validate total amount
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        // Validate individual milestone amounts
        for milestone in milestones.iter() {
            if milestone.amount <= 0 {
                return Err(Error::InvalidAmount);
            }
        }

        // Sum milestone amounts using checked_add to prevent overflow
        // This is the critical overflow-safe summation as required by issue #361
        let mut sum: i128 = 0;
        for milestone in milestones.iter() {
            // Use checked_add to detect overflow and return typed error instead of panicking
            sum = match sum.checked_add(milestone.amount) {
                Some(result) => result,
                None => {
                    // Overflow occurred - return typed error instead of panicking
                    return Err(Error::Overflow);
                }
            };
        }

        // Verify that milestone amounts sum to the total vault amount
        // This invariant must be maintained: sum == amount
        if sum != amount {
            return Err(Error::AmountMismatch);
        }

        // Create and return the vault
        let vault = Vault {
            id: vault_id,
            creator,
            amount,
            verifier,
            success_destination,
            failure_destination,
            milestones,
        };

        Ok(vault)
    }

    /// Reclaim any residual token balance left in the contract after a vault
    /// has reached a terminal settlement. This transfers the contract's token
    /// balance to the vault creator.
    ///
    /// Requirements:
    /// - Caller must be the `creator` (authorization enforced)
    /// - Vault must be settled (no staked amount remaining)
    pub fn reclaim_after_settlement(
        env: Env,
        vault: Vault,
        token_address: Address,
    ) -> Result<(), Error> {
        // Ensure the caller is the creator
        let creator_addr = Address::from_string(&vault.creator);
        creator_addr.require_auth();

        // Conservatively require the tracked staked amount to be zero before
        // sweeping any residuals. This keeps semantics clear: reclaiming is
        // only allowed once the vault has no outstanding stake.
        if vault.amount != 0 {
            return Err(Error::StakedRemaining);
        }

        // Use the on-chain contract address as the token holder to sweep from
        let contract_addr = env.current_contract_address();
        let token = TokenClient::new(&env, &token_address);

        // Query contract's token balance and transfer any leftover to creator
        let bal: i128 = token.balance(&contract_addr);
        if bal > 0 {
            token.transfer(&contract_addr, &creator_addr, &bal);
        }

        Ok(())
    }
}
