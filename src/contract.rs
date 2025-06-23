use soroban_sdk::{contractimpl, Env};

pub struct YieldDistributor;

#[derive(Debug)]
pub enum ContractError {
    Unauthorized,
    InvalidStrategy,
}

#[contractimpl]
impl YieldDistributor {
    pub fn accrue(env: Env) -> Result<(), ContractError> {
        // Lógica para calcular y distribuir el yield
        Ok(())
    }

    pub fn update_strategy(env: Env, new_apr: i128) -> Result<(), ContractError> {
        // Lógica para actualizar la estrategia de yield
        Ok(())
    }
}