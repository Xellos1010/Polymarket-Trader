# list entity balances

#### Path Parameters

#### Query Parameters

A list of symbols by which to filter the response

Id to retrieve additional results strictly after

Number of items to retrieve

A type by which to filter aggregated balances, defaults to "TOTAL"

-   UNKNOWN\_BALANCE\_TYPE: nil (-- api-linter: core::0126::unspecified=disabled --)
-   TRADING\_BALANCES: Trading balances
-   VAULT\_BALANCES: Vault balances
-   TOTAL\_BALANCES: Total balances (The sum of vault and trading + prime custody)
-   PRIME\_CUSTODY\_BALANCES: Prime custody balances
-   UNIFIED\_TOTAL\_BALANCES: Unified total balance across networks and wallet types (vault + trading + prime custody)

Available options

:

`TRADING_BALANCES`,

`VAULT_BALANCES`,

`TOTAL_BALANCES`,

`PRIME_CUSTODY_BALANCES`,

`UNIFIED_TOTAL_BALANCES`

#### Response