#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Error};

#[contract]
pub struct YieldDistributor;

#[contractimpl]
impl YieldDistributor {
    pub fn accrue(_env: Env) -> Result<(), Error> {
        Ok(())
    }

    pub fn update_strategy(_env: Env, _new_apr: i128) -> Result<(), Error> {
        Ok(())
    }
}

