#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, contracterror, panic_with_error};
use soroban_sdk::{Env, Address};
// use soroban_sdk::token::TokenClient;


const ADMIN_KEY: &str = "ADMIN";
const INDEX_STRATEGY: &str = "STRATEGY";

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Strategy {
    pub percentage: i128,
    pub token: Address,
    pub expires_at: u64,
}

#[contracterror]
#[derive(Debug, PartialEq, Eq)]
pub enum YieldError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    ManagerNotSet = 3,
    InvalidPercentage = 4,
}

#[contract]
pub struct YieldDistributor;

#[contractimpl]
impl YieldDistributor {
    pub fn initialize(env: &Env, admin: Address) {
        if env.storage().instance().has(&ADMIN_KEY) {
            panic_with_error!(&env, YieldError::AlreadyInitialized);
        }

        admin.require_auth();

        env.storage().instance().set(&ADMIN_KEY, &admin);
    }

    fn get_admin(env: &Env) -> Address {
        let admin: Address = env.storage().instance().get(&ADMIN_KEY)
            .unwrap_or_else(|| panic_with_error!(env, YieldError::NotInitialized));
        admin
    }

    pub fn set_strategy(
        env: Env,
        percentage: i128,
        token: Address,
        expires_at: u64
    ) {
        let admin = Self::get_admin(&env);
        admin.require_auth();

        // Supongamos que percentage está en centésimas (100.00 = 10000, 0.01 = 1)
        if percentage < 1 || percentage > 10000 {
            panic_with_error!(&env, YieldError::InvalidPercentage);
        }

        let rule = Strategy {
            percentage,
            token: token.clone(),
            expires_at,
        };

        env.storage().instance().set(&INDEX_STRATEGY, &rule);
    }

    pub fn get_strategy(env: &Env) -> Strategy {
        env.storage().instance().get(&INDEX_STRATEGY)
            .unwrap_or_else(|| panic_with_error!(env, YieldError::NotInitialized))
    }

    pub fn accrue(_env: Env) -> Result<(), YieldError> {
        Ok(())
    }
}

