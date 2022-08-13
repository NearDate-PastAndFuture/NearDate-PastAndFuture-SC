use crate::*;

#[near_bindgen]
impl Contract {
    /// views

    //get rent info of NFT
    pub fn get_rent(
        &self,
        nft_contract_token: ContractAndTokenId,
    ) ->Option<Rent> {

    }

    //get rent info of NFT
    pub fn get_rent_by_token_id(
        &self,
        token_id: TokenId,
    ) ->Vec<Rent> {

    }

    //
    pub fn get_rent_by_account_id(
        &self,
        account_id: AccountId,
    ) ->Vec<Rent> {

    }

    //
    pub fn get_bid_rent_by_account_id(
        &self,
        account_id: AccountId,
    )->Vec<BidRent>{

    }
}