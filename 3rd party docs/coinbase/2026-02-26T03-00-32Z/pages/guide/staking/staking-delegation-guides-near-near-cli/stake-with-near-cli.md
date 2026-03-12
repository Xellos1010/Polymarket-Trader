# stake with near cli

## To stake from an account…

1.  **Authenticate your NEAR wallet with the CLI**
    1.  Using the computer on which you installed the NEAR CLI, log into the [NEAR wallet site](https://wallet.near.org/).
    2.  Execute the command:
    3.  Click `Allow` in the new tab.
    4.  Enter your Wallet ID.
    5.  Click `Confirm`.
2.  **Deposit NEAR to the stakingpool contract**
    6.  Execute the command using Coinbase’s Staking Pool ID: `bisonstrails.poolv1.near`.
        
        ```
        near call <bisonstrails.poolv1.near> deposit '{}' --accountId <WALLET_ID> --amount <N>
        
        ```
        
    7.  Receive the response:
        
        ```
        Log [my_validator.stakingpool]: gbt_admin_testnet deposited 10000000000000000000000000. New unstaked balance is 10000000000000000000000000
        
        ```
        
3.  **Stake NEAR to the stakingpool contract**
    8.  Execute the command:
        
        ```
        near call <bisonstrails.poolv1.near> stake '{"amount": "N0000000000000000000000000"}' --accountId <WALLET_ID>
        
        ```
        
    9.  Receive the response:
        
        ```
        Log [my_validator.stakingpool]: gbt_admin_testnet staking 10000000000000000000000000 new staking shares. Total 0 unstaked balance and 10000000000000000000000000 staking shares
        
        ```
        

## To stake from a lockup contract…

1.  **Confirm ownership of your lockup contract**
    1.  From Terminal, execute the command:
        
        ```
        near view LOCKUP_CONTRACT get_owner_account_id '{}'
        
        ```
        
    2.  Match the `owner_account_id` to the lockup contract ID associated with that owner ID.
2.  **Designate the staking pool and deposit NEAR**
    3.  Execute the command:
        
        ```
        near call LOCKUP_CONTRACT select_staking_pool '{"staking_pool_account_id": "bisonstrails.poolv1.near"}' --account_id OWNER_ACCOUNT_ID
        
        ```
        
    4.  To deposit NEAR, execute the command:
        
        ```
        near call LOCKUP_CONTRACT deposit_to_staking_pool '{"amount": "N0000000000000000000000000"}' --accountId OWNER_ACCOUNT_ID --gas=150000000000000
        
        ```
        
    5.  Receive `TRUE` or `FALSE` value, along with your transaction receipt and a URL to view your transaction in the explorer.
3.  **Stake NEAR to the stakingpool contract**
    6.  Execute the command:
        
        ```
        near call LOCKUP_CONTRACT stake '{"amount": "N0000000000000000000000000"}' --accountId OWNER_ACCOUNT_ID --gas=150000000000000
        
        ```
        
    7.  Receive `TRUE` or `FALSE` value along with your transaction receipt and a URL to view your transaction in the explorer.