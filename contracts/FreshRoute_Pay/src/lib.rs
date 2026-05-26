#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Symbol,
};

#[contract]
pub struct FreshRoutePayContract;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Buyer,
    Farmer,
    Amount,
    Delivered,
    Paid,
}

#[contractimpl]
impl FreshRoutePayContract {

    // Initialize escrow agreement
    pub fn initialize(
        env: Env,
        buyer: Address,
        farmer: Address,
        amount: i128,
    ) {
        buyer.require_auth();

        env.storage().instance().set(&DataKey::Buyer, &buyer);
        env.storage().instance().set(&DataKey::Farmer, &farmer);
        env.storage().instance().set(&DataKey::Amount, &amount);
        env.storage().instance().set(&DataKey::Delivered, &false);
        env.storage().instance().set(&DataKey::Paid, &false);
    }

    // Buyer confirms produce delivery
    pub fn confirm_delivery(env: Env, buyer: Address) {
        buyer.require_auth();

        let stored_buyer: Address = env
            .storage()
            .instance()
            .get(&DataKey::Buyer)
            .unwrap();

        if buyer != stored_buyer {
            panic!("Unauthorized buyer");
        }

        env.storage().instance().set(&DataKey::Delivered, &true);
    }

    // Release payment after delivery confirmation
    pub fn release_payment(env: Env) -> Symbol {
        let delivered: bool = env
            .storage()
            .instance()
            .get(&DataKey::Delivered)
            .unwrap();

        if !delivered {
            panic!("Delivery not confirmed");
        }

        let paid: bool = env
            .storage()
            .instance()
            .get(&DataKey::Paid)
            .unwrap();

        if paid {
            panic!("Already paid");
        }

        env.storage().instance().set(&DataKey::Paid, &true);

        symbol_short!("PAID")
    }

    // Check payment status
    pub fn payment_status(env: Env) -> bool {
        env.storage()
            .instance()
            .get(&DataKey::Paid)
            .unwrap()
    }
}

