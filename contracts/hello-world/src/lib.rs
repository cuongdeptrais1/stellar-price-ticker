#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};

// Storage keys
const ADMIN: Symbol = symbol_short!("ADMIN");
const PRICE: Symbol = symbol_short!("PRICE");

#[contract]
pub struct PriceTickerContract;

#[contractimpl]
impl PriceTickerContract {
    /// Initialize contract with admin and starting price
    /// Should only be called once after deployment
    pub fn init(env: Env, admin: Address, start_price: u64) {
        // Check if already initialized
        if env.storage().instance().has(&ADMIN) {
            panic!("Contract already initialized");
        }

        // Store admin address
        env.storage().instance().set(&ADMIN, &admin);

        // Store starting price
        env.storage().instance().set(&PRICE, &start_price);
    }

    /// Update price (admin only)
    pub fn set_price(env: Env, new_price: u64) {
        // Get admin address
        let admin: Address = env
            .storage()
            .instance()
            .get(&ADMIN)
            .expect("Admin not set. Call 'init' first.");

        // Require authentication
        admin.require_auth();

        // Update price
        env.storage().instance().set(&PRICE, &new_price);
    }

    /// Get current price (public view)
    pub fn get_price(env: Env) -> u64 {
        // Return price or 0 if not set
        env.storage().instance().get(&PRICE).unwrap_or(0_u64)
    }
}
