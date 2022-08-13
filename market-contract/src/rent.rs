use crate::*;
use near_sdk::promise_result_as_success;

//struct that holds important information about each sale on the market

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct RentData {
    pub renting_account_id: AccountId, // AccountId rent this slot
    pub starts_at: Option<u64>, // When rent NFT slot starts being valid, Unix epoch in milliseconds
    pub expires_at: Option<u64>, // When rent NFT slot expires, Unix epoch in milliseconds
    pub rent_message: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Rent {
    //owner of the sale
    pub owner_id: AccountId,
    //nft contract where the token was minted
    pub nft_contract_id: String,
    //actual token ID for sale
    pub token_id: String,
    //rent price in yoctoNEAR that the token is listed for
    pub rent_conditions: SalePriceInYoctoNear,
    //message link
    pub message_url: String,
    //keep track rented slots
    pub rented_slots: HashMap<u32, RentData>,
}

#[near_bindgen]
impl Contract {
    
    //updates the price for a rent a nft on the market
    #[payable]
    pub fn set_rent_price(
        &mut self,
        nft_contract_id: AccountId,
        token_id: String,
        price: U128,
        message_url: String, 
    ) {
    }

    //place an offer on a specific rent. The rent will go through when owner accept
    #[payable]
    pub fn offer_rent(
        &mut self, 
        nft_contract_id: AccountId, 
        token_id: String, 
        message: String,
        starts_at: u64,
        expires_at: u64,
    ) {
        //
    }

    //accept an offer, transfer deposited token to owner of NFT
    //save message to ipfs and update to NFT data
    #[payable]
    pub fn accept_offer(&mut self, nft_contract_id: AccountId, token_id: String, message_url: String){
        //check expired ?
    }

    //cancel an offer and widthdraw deposited token
    #[payable]
    pub fn cancel_offer_rent(&mut self, nft_contract_id: AccountId, token_id: String){
    }

    //private function used when a rent is purchased. 
    //this will remove the rent slot, transfer and get the payout from the bid market contract
    #[private]
    pub fn process_purchase_rent(
        &mut self,
        nft_contract_id: AccountId,
        token_id: String,
        price: U128,
        renter_id: AccountId,
    ) {
        
    }
}

