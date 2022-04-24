use crate::*;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Debug)]
pub struct Member {
    pub account_id: AccountId,
    pub contribute: u128,
    pub join_date: u64,
}

#[near_bindgen]
impl NephDao {
    pub fn join_dao(&mut self) {
        let account_id = env::signer_account_id();
        let join_date = env::block_timestamp();
        let contribute = 0;
        let member = Member {
            account_id: account_id.clone(),
            contribute,
            join_date,
        };
        env::log_str(
            format!(
                "Member Join '{:?}' for account '{}'",
                member,
                account_id.clone(),
            )
            .as_str(),
        );

        self.members.insert(account_id, member);
    }

    pub fn get_member(&self, account_id: AccountId) -> &Member {
        self.members.get(&account_id).expect("Member not found")
    }
}
