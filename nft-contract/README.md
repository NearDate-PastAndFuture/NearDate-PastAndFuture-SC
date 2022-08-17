# NearDate-PastAndFuture

## Setup

Once you've created your near wallet go ahead and login to your wallet with your cli and follow the on-screen prompts

```=bash
near login
```

Once your logged in you have to deploy the contract. Make a subaccount with the name of your choosing 

```=bash 
near create-account nft-example.your-account.testnet --masterAccount your-account.testnet --initialBalance 10
```

After you've created your sub account deploy the contract to that sub account, set this variable to your sub account name

```=bash
NFT_CONTRACT_ID=nft-example.your-account.testnet

MAIN_ACCOUNT=your-account.testnet
```

Verify your new variable has the correct value
```=bash
echo $NFT_CONTRACT_ID

echo $MAIN_ACCOUNT
```


### Deploy Your Contract
```=bash
near deploy --accountId $NFT_CONTRACT_ID --wasmFile out/main.wasm
```

### Initialize Your NFT-Token Contract 

```=bash
near call $NFT_CONTRACT_ID new_default_meta '{"owner_id": "'$NFT_CONTRACT_ID'"}' --accountId $NFT_CONTRACT_ID
```

### View Contracts Meta Data

```=bash
near view $NFT_CONTRACT_ID nft_metadata
```
### Minting Token

```=bash
near call $NFT_CONTRACT_ID nft_mint '{"token_id": "20220816", "metadata": {"title": "NearDate Demo day"}, "receiver_id": "'$MAIN_ACCOUNT'", "message_url": "https://bafybeiaeolnfjsai6bvf56slqwvtpmsse2f5gljuv4b4tmm6vql2er6t54.ipfs.dweb.link/20220816_test.json"}' --accountId $MAIN_ACCOUNT --amount 0.1
```

After you've minted the token go to wallet.testnet.near.org to `your-account.testnet` and look in the collections tab and check out your new sample NFT! 



## View NFT Information

After you've minted your NFT you can make a view call to get a response containing the `token_id` `owner_id` and the `metadata`

```=bash
near view $NFT_CONTRACT_ID nft_token '{"token_id": "20210816"}'
```

List all NFT minted with month and day
```=bash
near view $NFT_CONTRACT_ID nft_tokens_by_date '{"date" : "0816"}'
```

List all NFT owned by accountID
```=bash
near view $NFT_CONTRACT_ID nft_tokens_for_owner '{"account_id" : "'$MAIN_ACCOUNT'"}'
```

Get random n NFT or get default 10 NFT (without number parameter)
```=bash
near view $NFT_CONTRACT_ID get_random_nfts '{"number" : n}'
```

## Update NFT message

Update message of NFT with the new link url
```=bash
near call $NFT_CONTRACT_ID   '{"token_id" : "20210816", "message_url" : ""}' --accountId $MAIN_ACCOUNT --depositYocto 1
```

## Transfering NFTs

To transfer an NFT go ahead and make another [testnet wallet account](https://wallet.testnet.near.org).

Then run the following
```=bash
MAIN_ACCOUNT_2=your-second-wallet-account.testnet
```

Verify the correct variable names with this

```=bash
echo $NFT_CONTRACT_ID

echo $MAIN_ACCOUNT

echo $MAIN_ACCOUNT_2
```

To initiate the transfer..

```bash=
near call $NFT_CONTRACT_ID nft_transfer '{"receiver_id": "$MAIN_ACCOUNT_2", "token_id": "20210816", "memo": ""}' --accountId $MAIN_ACCOUNT --depositYocto 1
```

In this call you are depositing 1 yoctoNEAR for security and so that the user will be redirected to the NEAR wallet.

