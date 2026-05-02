# claim wallet staking rewards alpha

Claim Wallet Staking Rewards (Alpha)

Request to claim staking rewards. This feature is in alpha. Please reach out to your Coinbase Prime account manager for more information

#### Path Parameters

#### Body

The client generated idempotency key for requested execution. Any subsequent requests with the same key will return the original response

WalletClaimRewardsInputs contains the custom inputs for claim rewards operations on a wallet. Requirements and supported fields vary by asset type.

#### Response

ID of the newly created transaction, can be used to fetch details of the current state of execution

The ID for the activity generated for this request