use crate::*;

//Bid for buy token structure
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct BidToken{
    bid_account_id: AccountId,
    token_id: TokenId,
    bid_id: u64,
    price: SalePriceInYoctoNear,
}

//Bid for rent structure
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct BidRent{
    bid_account_id: AccountId,
    token_id: TokenId,
    bid_id: u64,
    price: SalePriceInYoctoNear,
    message_url: String,
}

#[near_bindgen]
impl Contract {
    //Allows users to deposit to offer
    //Optional account ID is to users can pay for other people.
    #[payable]
    pub fn bid_token(&mut self, token_id: TokenId) {
        //get the deposit value which is how much the user wants to add to their storage
        let deposit = env::attached_deposit();

        //create BidToken object
        let account_id = env::predecessor_account_id();
        let bid = BidToken{
            bid_account_id : env::predecessor_account_id(),
            token_id : token_id.clone(),
            bid_id : self.bid_token_id,
            price : U128(deposit),
        };

        //get the balance of the account (if the account isn't in the map we default to a balance of 0)
        let mut balance: u128 = self.bid_token_deposits.get(&account_id).unwrap_or(0);
        //add the deposit to their balance
        balance += deposit;
        //insert the balance back into the map for that account ID
        self.bid_token_deposits.insert(&account_id, &balance);

        //update current Id
        self.bid_token_id += 1;
        //update bid_token_offers
        if let Some(mut offers) = self.bid_token_by_account.get(&account_id){
            offers.push(bid);
            self.bid_token_by_account.insert(&account_id, &offers);
            self.bid_token_by_token_id.insert(&token_id, &offers);
        }else{
            let offers = vec![bid];
            self.bid_token_by_account.insert(&account_id, &offers);
            self.bid_token_by_token_id.insert(&token_id, &offers);
        }
    }

    //Allows users to withdraw offer buy NFT
    #[payable]
    pub fn bid_token_cancel_and_widthdraw(&mut self, bid_id: u64) {
        //make sure the user attaches exactly 1 yoctoNEAR for security purposes.
        //this will redirect them to the NEAR wallet (or requires a full access key). 
        assert_one_yocto();

        //get price bidded
        let account_id = env::predecessor_account_id();
        let mut bids_by_account = self.bid_token_by_account.get(&account_id).unwrap(); 
        
        let index = bids_by_account.iter().position(|a| a.bid_id == bid_id).unwrap();
        let bid = &bids_by_account[index];
        let price = bid.price.0;
        let bid_account = bid.bid_account_id.clone();
        let token_id = bid.token_id.clone();

        assert_eq!(bid_account, account_id, "must be owner of bid to cancel");

        //widthdaw bidded amount 
        let mut balance: u128 = self.bid_token_deposits.get(&account_id).unwrap_or(0);

        if(price <= balance)
        {
            Promise::new(bid_account.clone()).transfer(price);
            balance -= price;
            self.bid_token_deposits.insert(&bid_account, &balance);
        }
        //remove from bid token by account 
        bids_by_account.remove(index);
        self.bid_token_by_account.insert(&account_id, &bids_by_account);

        //remove from bid token by token id
        let mut bids_by_token_id = self.bid_token_by_token_id.get(&token_id).unwrap();
        bids_by_token_id.remove(index);
        self.bid_token_by_token_id.insert(&token_id, &bids_by_token_id);
    }

    #[payable]
    pub fn accept_bid_token(&mut self, bid_id: u32){
        assert_one_yocto();
        let account_id = env::predecessor_account_id();

        //transfer nft and get Near bidded from contract

        //remove BidToken object

    }

    //Allows users to deposit to rent
    //Optional account ID is to users can pay for other people.
    #[payable]
    pub fn bid_rent(&mut self, token_id: TokenId, message: String ) {
//get the deposit value which is how much the user wants to add to their storage
        let deposit = env::attached_deposit();

        //create BidToken object
        let account_id = env::predecessor_account_id();
        let bid = BidRent{
            bid_account_id : env::predecessor_account_id(),
            token_id : token_id.clone(),
            bid_id : self.bid_rent_id,
            price : U128(deposit),
            message_url: message,
        };

        //get the balance of the account (if the account isn't in the map we default to a balance of 0)
        let mut balance: u128 = self.bid_rent_deposits.get(&account_id).unwrap_or(0);
        //add the deposit to their balance
        balance += deposit;
        //insert the balance back into the map for that account ID
        self.bid_rent_deposits.insert(&account_id, &balance);

        //update current Id
        self.bid_rent_id += 1;
        //update bid_token_offers
        if let Some(mut offers) = self.bid_rent_by_account.get(&account_id){
            offers.push(bid);
            self.bid_rent_by_account.insert(&account_id, &offers);
            self.bid_rent_by_token_id.insert(&token_id, &offers);
        }else{
            let offers = vec![bid];
            self.bid_rent_by_account.insert(&account_id, &offers);
            self.bid_rent_by_token_id.insert(&token_id, &offers);
        }
    }

    //Allows users to withdraw offer rent NFT
    #[payable]
    pub fn bid_rent_cancel_and_widthdraw(&mut self, bid_id: u64) {
        //make sure the user attaches exactly 1 yoctoNEAR for security purposes.
        //this will redirect them to the NEAR wallet (or requires a full access key). 
        assert_one_yocto();

        //get price bidded
        let account_id = env::predecessor_account_id();
        let mut bids_by_account = self.bid_rent_by_account.get(&account_id).unwrap(); 
        
        let index = bids_by_account.iter().position(|a| a.bid_id == bid_id).unwrap();
        let bid = &bids_by_account[index];
        let price = bid.price.0;
        let bid_account = bid.bid_account_id.clone();
        let token_id = bid.token_id.clone();

        assert_eq!(bid_account, account_id, "must be owner of bid to cancel");

        //widthdaw bidded amount 
        let mut balance: u128 = self.bid_rent_deposits.get(&account_id).unwrap_or(0);

        if(price <= balance)
        {
            Promise::new(bid_account.clone()).transfer(price);
            balance -= price;
            self.bid_rent_deposits.insert(&bid_account, &balance);
        }
        //remove from bid token by account 
        bids_by_account.remove(index);
        self.bid_rent_by_account.insert(&account_id, &bids_by_account);

        //remove from bid token by token id
        let mut bids_by_token_id = self.bid_rent_by_token_id.get(&token_id).unwrap();
        bids_by_token_id.remove(index);
        self.bid_rent_by_token_id.insert(&token_id, &bids_by_token_id);
    }

    #[payable]
    pub fn accept_bid_rent(&mut self, bid_id: u32){

    }

}