# overview

The Address History API provides indexed onchain data across multiple blockchain networks, offering comprehensive information on asset transactions and balance histories. Unlike the SQL API which allows custom queries across all Base blockchain data, Address History focuses specifically on wallet-centric JSON RPC operations with pre-built endpoints. Key features include:

-   **Multi-chain support**: Query wallet data across different blockchains
-   **Asset transaction history**: Get all transactions for a specific wallet address
-   **Balance history tracking**: Track balance changes over time
-   **Indexed data for efficient queries**: Fast responses without complex query writing
-   **Simple API endpoints**: No SQL knowledge required

There are various ways to query address history data, including:

-   [JSON-RPC](https://developer.chrome.com/api-reference/json-rpc-api/address-history)
-   [CDP API](https://developer.chrome.com/api-reference/introduction) e.g. [ListBalanceHistories](https://developer.chrome.com/api-reference/rest-api/addresses/get-address-balance-history-for-asset)
-   [CDP SDK](https://coinbase.github.io/coinbase-sdk-nodejs/index.html) e.g. [NodeJS address.listHistoricalBalance](https://coinbase.github.io/coinbase-sdk-nodejs/classes/coinbase_address.Address.html#listHistoricalBalances)

Here are some [quickstart examples](https://github.com/coinbase/coinbase-sdk-nodejs/tree/master/quickstart-template) for using the Address History API.