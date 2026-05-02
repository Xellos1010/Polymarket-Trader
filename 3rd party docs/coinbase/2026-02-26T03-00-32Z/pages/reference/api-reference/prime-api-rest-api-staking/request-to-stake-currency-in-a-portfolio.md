# request to stake currency in a portfolio

Request to stake currency in a portfolio

Creates an execution request to stake funds across a portfolio. This will stake funds in one or more wallets in the portfolio, with a total bondable balance up to the requested stake amount.

#### Path Parameters

#### Body

The client generated idempotency key (uuid required) for requested execution. Subsequent requests using the same key will not create new transactions.

The currency symbol to stake

The quantity of the chosen currency to stake

#### Response

The ID for the created activity

The ID for the created transaction