use crate::*;

//Todo Bid struct

#[near_bindgen]
impl Contract {
    //Allows users to deposit to offer
    //Optional account ID is to users can pay for other people.
    #[payable]
    pub fn bid_token(&mut self, account_id: Option<AccountId>) {
    }

    //Allows users to withdraw offer buy NFT
    #[payable]
    pub fn bid_token_cancel_and_withdraw(&mut self, token_id: TokenId) {
    }

    #[payable]
    pub fn accept_bid_token(&mut self, token_id: TokenId){

    }

    //Allows users to deposit to rent
    //Optional account ID is to users can pay for other people.
    #[payable]
    pub fn bid_rent(&mut self, token_id: TokenId, message: String ) {

    }

    //Allows users to withdraw offer rent NFT
    #[payable]
    pub fn bid_rent_cancel_and_withdraw(&mut self, token_id: TokenId) {
    }

    #[payable]
    pub fn accept_bid_rent(&mut self, token_id: TokenId){

    }
}