# request to stake or delegate a wallet

Request to stake or delegate a wallet

#### Path Parameters

#### Body

StakingInitiateRequest represents a request to initiate a staking operation.

The client generated idempotency key for requested execution. Subsequent requests using the same key will fail

WalletStakeInputs contains the custom inputs for staking operations on a wallet. Requirements and supported fields vary by asset type.

#### Response

StakingInitiateResponse contains the response data from initiating a staking operation.

ID of the newly created transaction, can be used to fetch details of the current state of execution

The ID for the activity generated for this request