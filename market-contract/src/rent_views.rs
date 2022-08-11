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
}