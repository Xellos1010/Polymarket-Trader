# request to unstake a wallet

Request to unstake a wallet

#### Path Parameters

#### Body

StakingUnstakeRequest represents a request to initiate an unstaking operation.

The client generated idempotency key for requested execution. Subsequent requests using the same key will fail

WalletUnstakeInputs contains the custom inputs for unstaking operations on a wallet. Requirements and supported fields vary by asset type.

#### Response

StakingUnstakeResponse contains the response data from initiating an unstaking operation.

ID of the newly created transaction, can be used to fetch details of the current state of execution

The ID for the activity generated for this request