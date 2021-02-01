use borsh::{ BorshDeserialize, BorshSerialize };
use near_sdk::{
    env, near_bindgen, AccountId, PublicKey, Promise,
    collections::{ UnorderedMap },
    json_types::{ U128, Base58PublicKey },
};
use serde::Serialize;

const DROP_AMOUNT: U128 = U128(100);

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]

#[derive(BorshDeserialize, BorshSerialize)]
pub struct SocialDrop {
    pub owner_id: AccountId,
    pub dropped: UnorderedMap<PublicKey, U128>,
    pub tokens: UnorderedMap<AccountId, U128>,
}

impl Default for SocialDrop {
    fn default() -> Self {
        panic!("Should be initialized before usage")
    }
}

#[near_bindgen]
impl SocialDrop {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Invalid owner account");
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner_id,
            dropped: UnorderedMap::new(b"dropped".to_vec()),
            tokens: UnorderedMap::new(b"tokens".to_vec()),
        }
    }

    pub fn drop(&mut self) {
        let public_key: PublicKey = env::signer_account_pk();
        assert_eq!(env::signer_account_id(), env::predecessor_account_id(), "Key not from app contract");
        let balance:U128 = self.dropped.get(&public_key.clone()).unwrap_or(U128(0));
        assert_eq!(balance, U128(0), "Tokens already dropped");
        self.dropped.insert(&public_key, &DROP_AMOUNT);
    }

    pub fn transfer(&mut self, account_id: AccountId) {
        let public_key: PublicKey = env::signer_account_pk();
        assert_eq!(env::signer_account_id(), env::predecessor_account_id(), "Key not from app contract");
        let balance:U128 = self.dropped.get(&public_key.clone()).unwrap_or(U128(0));
        assert_ne!(balance, U128(0), "No tokens");
        self.dropped.remove(&public_key);
        self.tokens.insert(&account_id.into(), &balance);
    }

    pub fn get_balance_dropped(&self, public_key: Base58PublicKey) -> U128 {
        self.dropped.get(&public_key.into()).unwrap_or(U128(0))
    }

    pub fn get_balance_tokens(&self, account_id: AccountId) -> U128 {
        self.tokens.get(&account_id.into()).unwrap_or(U128(0))
    }
}

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    
    fn ntoy(near_amount: u128) -> U128 {
        U128(near_amount * 10u128.pow(24))
    }

    fn get_context() -> VMContext {
        VMContext {
            predecessor_account_id: "alice.testnet".to_string(),
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "alice.testnet".to_string(),
            signer_account_pk: vec![0],
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 19,
            storage_usage: 1000
        }
    }

    #[test]
    fn create() {
        let mut context = get_context();
        context.signer_account_pk = Base58PublicKey::try_from("ed25519:Eg2jtsiMrprn7zgKKUk79qM1hWhANsFyE6JSX4txLEuy").unwrap().into();
        testing_env!(context.clone());
        let mut contract = SocialDrop::new(context.current_account_id.clone());
        contract.drop();
        let balance: U128 = contract.get_balance_dropped(context.signer_account_pk);
        assert_eq!(balance, DROP_AMOUNT);
    }

    #[test]
    fn transfer() {
        let mut context = get_context();
        context.signer_account_pk = Base58PublicKey::try_from("ed25519:Eg2jtsiMrprn7zgKKUk79qM1hWhANsFyE6JSX4txLEuy").unwrap().into();
        testing_env!(context.clone());
        let mut contract = SocialDrop::new(context.current_account_id.clone());
        contract.drop();
        contract.transfer("bob.testnet".to_string());
        let balance: U128 = contract.get_balance_tokens("bob.testnet".to_string());
        assert_eq!(balance, DROP_AMOUNT);
    }
    

    #[test]
    #[should_panic(
        expected = r#"Tokens already dropped"#
    )]
    fn panic_double_drop() {
        let mut context = get_context();
        context.signer_account_pk = Base58PublicKey::try_from("ed25519:Eg2jtsiMrprn7zgKKUk79qM1hWhANsFyE6JSX4txLEuy").unwrap().into();
        testing_env!(context.clone());
        let mut contract = SocialDrop::new(context.current_account_id.clone());
        contract.drop();
        contract.drop();
        
    }

    #[test]
    #[should_panic(
        expected = r#"No tokens"#
    )]
    fn panic_transfer_no_drop() {
        let mut context = get_context();
        context.signer_account_pk = Base58PublicKey::try_from("ed25519:Eg2jtsiMrprn7zgKKUk79qM1hWhANsFyE6JSX4txLEuy").unwrap().into();
        testing_env!(context.clone());
        let mut contract = SocialDrop::new(context.current_account_id.clone());
        contract.transfer("bob.testnet".to_string());
    }
}