use members::members::Member;
// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::store::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault};

mod members;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct NephDao {
    members: UnorderedMap<AccountId, Member>,
}
#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Members,
}
#[near_bindgen]
impl NephDao {
    #[init]
    pub fn new() -> Self {
        Self {
            members: UnorderedMap::new(StorageKey::Members.try_to_vec().unwrap()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn mint_and_transfer_test() {
        let mut context = get_context(accounts(0));
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(0))
            .build());
        let mut contract = NephDao::new();
        contract.join_dao();
        let a = contract.get_member(accounts(0));
        println!("{:?}", a);
    }
}
