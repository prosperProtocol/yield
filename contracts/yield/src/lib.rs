#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, contracterror, panic_with_error};
use soroban_sdk::{Env, Address};
use soroban_sdk::{IntoVal, symbol_short};


const ADMIN_KEY: &str = "ADMIN";
const INDEX_PERCENTAGE: &str = "PERCENTAGE";
const INDEX_TOKEN: &str = "TOKEN";

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StrategyStatus {
    Active,
    Expired,
    Completed,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Strategy {
    pub user: Address,
    pub amount: i128,
    pub percentage: i128,
    pub token: Address,
    pub created_at: u64,
    pub expires_at: u64,
    pub status: StrategyStatus,
}

#[contracterror]
#[derive(Debug, PartialEq, Eq)]
pub enum YieldError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    ManagerNotSet = 3,
    InvalidPercentage = 4,
    InvalidStatus = 5,
}

pub struct CustomTokenClient<'a> {
    env: &'a Env,
    contract_id: &'a Address,
}

impl<'a> CustomTokenClient<'a> {
    pub fn new(env: &'a Env, contract_id: &'a Address) -> Self {
        Self { env, contract_id }
    }

    fn mint(&self, to: &Address, amount: &i128) {
        self.env.invoke_contract::<()>(
            self.contract_id,
            &symbol_short!("mint"),
            (to, amount).into_val(self.env),
        );
    }

    fn read_balance(&self, user: &Address) -> i128 {
        self.env.invoke_contract::<i128>(
            self.contract_id,
            &symbol_short!("balance"),
            (user,).into_val(self.env),
        )
    }
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

    pub fn set_percentage(
        env: Env,
        percentage: i128
    ) {
        let admin = Self::get_admin(&env);
        admin.require_auth();

        // Supongamos que percentage está en centésimas (100.00 = 10000, 0.01 = 1)
        if percentage < 1 || percentage > 10000 {
            panic_with_error!(&env, YieldError::InvalidPercentage);
        }

        env.storage().instance().set(&INDEX_PERCENTAGE, &percentage);
    }

    fn get_percentage(env: &Env) -> i128 {
        env.storage().instance().get(&INDEX_PERCENTAGE)
            .unwrap_or_else(|| panic_with_error!(env, YieldError::NotInitialized))
    }

    pub fn set_token(
        env: &Env,
        token: Address,
    ) {
        let admin = Self::get_admin(&env);
        admin.require_auth();

        env.storage().instance().set(&INDEX_TOKEN, &token);
    }

    fn get_token(env: &Env) -> Address {
        env.storage().instance().get(&INDEX_TOKEN)
            .unwrap_or_else(|| panic_with_error!(env, YieldError::NotInitialized))
    }

    /// Funciones con User como parametro
    pub fn set_strategy(
        env: Env,
        user: Address,
        amount: i128
    ) {
        let admin = Self::get_admin(&env);
        admin.require_auth();
        let percentage = Self::get_percentage(&env);
        let token = Self::get_token(&env);
        let created_at: u64 = env.ledger().timestamp();
        let expires_at: u64 = created_at + 2592000;

        let rule = Strategy {
            user: user.clone(),
            amount,
            percentage,
            token,
            created_at,
            expires_at,
            status: StrategyStatus::Active,
        };

        env.storage().instance().set(&user, &rule);
    }

    pub fn get_strategy(env: &Env, user: Address) -> Strategy {
        env.storage().instance().get(&user)
            .unwrap_or_else(|| panic_with_error!(env, YieldError::NotInitialized))
    }

    pub fn set_strategy_status_expired(env: Env, user: Address) {
        let admin = Self::get_admin(&env);
        admin.require_auth();
        let mut strategy = Self::get_strategy(&env, user.clone());
        if strategy.status != StrategyStatus::Active {
            panic_with_error!(&env, YieldError::InvalidStatus);
        }
        let porcentage = strategy.percentage / 12;
        let balance = strategy.amount;
        let yield_amount = balance * porcentage;
        let token_client: CustomTokenClient<'_> = CustomTokenClient::new(&env, &strategy.token);
        token_client.mint(&user, &yield_amount);
        strategy.status = StrategyStatus::Expired;
        env.storage().instance().set(&user, &strategy);
    }

    pub fn set_strategy_status_completed(env: Env, user: Address) {
        let admin = Self::get_admin(&env);
        admin.require_auth();
        let mut strategy = Self::get_strategy(&env, user.clone());
        if strategy.status != StrategyStatus::Expired {
            panic_with_error!(&env, YieldError::InvalidStatus);
        }
        strategy.status = StrategyStatus::Completed;
        env.storage().instance().set(&user, &strategy);
    }

    fn set_apy(
        env: &Env,
        to: Address,
        amount: i128,
    ) {
        let admin = Self::get_admin(env);
        admin.require_auth();

        let strategy = Self::get_strategy(env, to.clone());
        let token_client: CustomTokenClient<'_> = CustomTokenClient::new(env, &strategy.token);
        token_client.mint(&to, &amount);
    }

    pub fn get_apy(env: &Env, user: Address) -> i128 {
        let strategy = Self::get_strategy(env, user.clone());
        let token_client = CustomTokenClient::new(env, &strategy.token);
        token_client.read_balance(&user)
    }

    pub fn accrue(env: Env, user: Address, amount: i128) -> Result<(), YieldError> {
        let admin = Self::get_admin(&env);
        admin.require_auth();
        let strategy_exists = env.storage().instance().has(&user);
        if !strategy_exists {
            Self::set_strategy(env.clone(), user.clone(), amount);
        }
        Self::set_apy(&env, user, amount);
        Ok(())
    }
}
