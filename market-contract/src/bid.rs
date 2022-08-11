use crate::*;

#[near_bindgen]
impl Contract {
    //Allows users to deposit to offer
    //Optional account ID is to users can pay for other people.
    #[payable]
    pub fn buy_deposit(&mut self, account_id: Option<AccountId>) {
    }

    //Allows users to withdraw offer buy NFT
    #[payable]
    pub fn buy_cancel_and_withdraw(&mut self) {
    }

    //Allows users to deposit to rent
    //Optional account ID is to users can pay for other people.
    #[payable]
    pub fn rent_deposit(&mut self, account_id: Option<AccountId>) {

    }

    //Allows users to withdraw offer rent NFT
    #[payable]
    pub fn rent_cancel_and_withdraw(&mut self) {
    }
}