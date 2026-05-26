#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env};

use crate::FreshRoutePayContract;

mod tests {

    use super::*;

    #[test]
    fn happy_path_payment_release() {
        let env = Env::default();

        let buyer = Address::generate(&env);
        let farmer = Address::generate(&env);

        FreshRoutePayContract::initialize(
            env.clone(),
            buyer.clone(),
            farmer,
            500,
        );

        FreshRoutePayContract::confirm_delivery(
            env.clone(),
            buyer,
        );

        let result =
            FreshRoutePayContract::release_payment(env);

        assert_eq!(result.to_string(), "PAID");
    }

    #[test]
    #[should_panic(expected = "Delivery not confirmed")]
    fn edge_case_release_before_delivery() {
        let env = Env::default();

        let buyer = Address::generate(&env);
        let farmer = Address::generate(&env);

        FreshRoutePayContract::initialize(
            env.clone(),
            buyer,
            farmer,
            500,
        );

        FreshRoutePayContract::release_payment(env);
    }

    #[test]
    fn state_verification_paid_status() {
        let env = Env::default();

        let buyer = Address::generate(&env);
        let farmer = Address::generate(&env);

        FreshRoutePayContract::initialize(
            env.clone(),
            buyer.clone(),
            farmer,
            500,
        );

        FreshRoutePayContract::confirm_delivery(
            env.clone(),
            buyer,
        );

        FreshRoutePayContract::release_payment(
            env.clone(),
        );

        let status =
            FreshRoutePayContract::payment_status(env);

        assert!(status);
    }

    #[test]
    #[should_panic(expected = "Already paid")]
    fn double_payment_prevention() {
        let env = Env::default();

        let buyer = Address::generate(&env);
        let farmer = Address::generate(&env);

        FreshRoutePayContract::initialize(
            env.clone(),
            buyer.clone(),
            farmer,
            500,
        );

        FreshRoutePayContract::confirm_delivery(
            env.clone(),
            buyer,
        );

        FreshRoutePayContract::release_payment(
            env.clone(),
        );

        FreshRoutePayContract::release_payment(env);
    }

    #[test]
    fn initialize_contract() {
        let env = Env::default();

        let buyer = Address::generate(&env);
        let farmer = Address::generate(&env);

        FreshRoutePayContract::initialize(
            env.clone(),
            buyer,
            farmer,
            1000,
        );

        let status =
            FreshRoutePayContract::payment_status(env);

        assert!(!status);
    }
}


