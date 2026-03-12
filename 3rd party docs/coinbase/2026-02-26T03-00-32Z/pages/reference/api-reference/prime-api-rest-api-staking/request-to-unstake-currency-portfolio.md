# request to unstake currency portfolio

Request to unstake currency across a portfolio

Creates an execution request to unstake funds across a portfolio. This will unstake funds in one or more wallets in the portfolio, with a total bonded balance up to the requested unstake amount.

#### Path Parameters

#### Body

The client generated idempotency key (uuid required) for requested execution. Subsequent requests using the same key will not create new transactions.

The currency symbol to unstake

The quantity of the chosen currency to unstake

#### Response

The ID for the created activity

The ID for the created transaction