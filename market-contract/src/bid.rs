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
    starts_at: u64, // When rent NFT slot starts being valid, Unix epoch in milliseconds
    expires_at: u64, // When rent NFT slot expires, Unix epoch in milliseconds
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

        let bid_token = BidToken{
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
            //self.bid_token_by_token_id.insert(&token_id, &offers);
        }else{
            let offers = vec![bid];
            self.bid_token_by_account.insert(&account_id, &offers);
            //self.bid_token_by_token_id.insert(&token_id, &offers);
        }
        if let Some(mut offers) = self.bid_token_by_token_id.get(&token_id){
            offers.push(bid_token);
            self.bid_token_by_token_id.insert(&token_id, &offers);
        }else{
            let offers = vec![bid_token];
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
        let index_token = bids_by_token_id.iter().position(|a| a.bid_id == bid_id).unwrap();
        bids_by_token_id.remove(index_token);
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
    pub fn bid_rent(&mut self, token_id: TokenId, message: String, start_at: u64, expire_at: u64) {
    //get the deposit value which is how much the user wants to add to their storage
        let deposit = env::attached_deposit();

        //create BidToken object
        let account_id = env::predecessor_account_id();
        let bid = BidRent{
            bid_account_id : env::predecessor_account_id(),
            token_id : token_id.clone(),
            bid_id : self.bid_rent_id,
            price : U128(deposit),
            message_url: message.clone(),
            starts_at: start_at,
            expires_at: expire_at,
        };

        let bid_rent =  BidRent{
            bid_account_id : env::predecessor_account_id(),
            token_id : token_id.clone(),
            bid_id : self.bid_rent_id,
            price : U128(deposit),
            message_url: message,
            starts_at: start_at,
            expires_at: expire_at,
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
            //self.bid_rent_by_token_id.insert(&token_id, &offers);
        }else{
            let offers = vec![bid];
            self.bid_rent_by_account.insert(&account_id, &offers);
            //self.bid_rent_by_token_id.insert(&token_id, &offers);
        }
        if let Some(mut offers) = self.bid_rent_by_token_id.get(&token_id){
            offers.push(bid_rent);
            self.bid_rent_by_token_id.insert(&token_id, &offers);
        }else{
            let offers = vec![bid_rent];
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
        let index_rent = bids_by_token_id.iter().position(|a| a.bid_id == bid_id).unwrap();
        bids_by_token_id.remove(index_rent);
        self.bid_rent_by_token_id.insert(&token_id, &bids_by_token_id);
    }

    #[payable]
    pub fn accept_bid_rent(&mut self, token_id: TokenId ,bid_id: u64){
        assert_one_yocto();

        let account_id = env::predecessor_account_id();


        //assert_eq!(account_id, rent_by_token.owner_id, "must be owner to accept");
        //TODO ext call to check owner of tokenId

        let mut bids_by_token_id = self.bid_rent_by_token_id.get(&token_id).unwrap(); 
        let index = bids_by_token_id.iter().position(|a| a.bid_id == bid_id).unwrap();
        let bid = &bids_by_token_id[index];
        let price = bid.price.0.clone();
        let bid_account = bid.bid_account_id.clone();
        let token_id = bid.token_id.clone();

        let mut balance: u128 = self.bid_rent_deposits.get(&bid_account).unwrap_or(0);

        //transfer
        if(price <= balance)
        {
            Promise::new(account_id.clone()).transfer(price);
            balance -= price;
            self.bid_rent_deposits.insert(&bid_account, &balance);
        }
       
        //get 
        let mut rent_data = RentData{
            renting_account_id : bid_account.clone(),
            starts_at : bid.starts_at,
            expires_at : bid.expires_at,
            rent_message : bid.message_url.clone(),
        };
        let mut rent_data_ = RentData{
            renting_account_id : bid_account.clone(),
            starts_at : bid.starts_at,
            expires_at : bid.expires_at,
            rent_message : bid.message_url.clone(),
        };

        let mut update_rent_by_account : bool = false;
        if let Some(mut rent) = self.rent_by_token.get(&token_id)
        {
            //check expired?
            if(rent.rented_slots.len() < NFT_MAX_RENT_SLOT)
            {
                rent.rented_slots.push(rent_data);
            }else{
                let index_expired = rent.rented_slots.iter().position(|a| a.expires_at < env::block_timestamp()).unwrap_or(NFT_MAX_RENT_SLOT);
                if index_expired < NFT_MAX_RENT_SLOT
                {
                    rent.rented_slots[index_expired] = rent_data;
                    update_rent_by_account = true;
                }
                else
                {
                    assert_eq!(index_expired, NFT_MAX_RENT_SLOT, "all slot rented");
                }
            }
            self.rent_by_token.insert(&token_id, &rent);
        }
        else{
            //create object Rent and add to map to track

            let rent = Rent{
                owner_id : account_id,
                token_id : token_id.clone(),
                message_url : bid.message_url.clone(),
                rented_slots : vec![rent_data],
            };
            self.rent_by_token.insert(&token_id, &rent);
            update_rent_by_account = true;
        }

        if update_rent_by_account == true
        {
            if let Some(mut rent_by_account) = self.rent_by_account.get(&bid_account)
            {
                rent_by_account.push(rent_data_);
                self.rent_by_account.insert(&bid_account, &rent_by_account);
            }else{
                let rent_data_by_account = vec![rent_data_];
                self.rent_by_account.insert(&bid_account, &rent_data_by_account);
            }

        }
        //remove from bid token by account 
        bids_by_token_id.remove(index);
        self.bid_rent_by_token_id.insert(&token_id, &bids_by_token_id);

        //remove from bid token by token id
        let mut bids_by_account = self.bid_rent_by_account.get(&bid_account).unwrap();
        let index_rent = bids_by_account.iter().position(|a| a.bid_id == bid_id).unwrap();
        bids_by_account.remove(index_rent);
        self.bid_rent_by_account.insert(&bid_account, &bids_by_account);
    }

}