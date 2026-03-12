# list transaction validators

List Transaction Validators

List ETH 0x02 validators associated with wallet-level stake transactions for a given portfolio. It will not return data for unstake transactions, portfolio stake transactions, transactions which staked different currencies, or which staked to Ethereum 0x01 validators.

#### Path Parameters

#### Body

List of transaction IDs to filter validators by. Maximum of 100 transaction IDs allowed per request.

Maximum number of transaction-validator associations to return per page. Default is 100, maximum is 1000.

Available options

:

`DESC`,

`ASC`

#### Response

List of transaction-to-validator associations. Each entry represents one transaction staking to one validator.