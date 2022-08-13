use crate::*;

#[near_bindgen]
impl Contract {
    //Allows users to deposit to offer
    //Optional account ID is to users can pay for other people.
    #[payable]
    pub fn get_bid_token(&mut self, token: TokenId) {
        
    }
}