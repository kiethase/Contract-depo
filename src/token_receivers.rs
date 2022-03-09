use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{serde_json, PromiseOrValue};


use crate::*;

#[near_bindgen]
impl FungibleTokenReceiver for Contract{
    #[allow(unreachable_code)]
    fn ft_on_transfer(
        &mut self,
        receiver_id: ValidAccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        let token_in = env::predecessor_account_id();
        assert!(msg.is_empty(),"Msg much empty on deposit action!!");
        // self.()

        PromiseOrValue::Value(U128(0))
    }
}


impl Contract {
    pub fn internal_save_transfer_information(
        &mut self,
        account_id: &AccountId,
        token_id: &AccountId, 
        amount: Balance,
        
    ){
    

    }
}
