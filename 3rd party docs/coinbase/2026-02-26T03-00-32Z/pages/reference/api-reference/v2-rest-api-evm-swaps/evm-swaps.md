# evm swaps

A swap refers to the act of exchanging one token for another. The EVM Swap APIs enable you to create and manage swaps. A swap process involves:

1.  A user approves token spending via smart contract.
2.  The smart contract determines the exchange rate and facilitates the swap between the two tokens.
3.  The transaction is completed atomically and the tokens are transferred to the user’s wallet.

The Swap API supports:

-   **Getting a price**: Estimate the amount of tokens you will receive for a given amount of exchanged tokens.
-   **Creating a swap quote**: Return a swap transaction payload that you can use to sign and submit in order to execute the swap.

For the easiest experience creating, signing, and submitting a swap, we recommend using the [CDP SDK](https://github.com/coinbase/cdp-sdk/).

-   [Python SDK examples](https://github.com/coinbase/cdp-sdk/tree/main/python#evm-swaps)
-   [TypeScript SDK examples](https://github.com/coinbase/cdp-sdk/tree/main/typescript#evm-swaps)

To read more about using the EVM Swap APIs, please see our [Swap API docs](https://docs.cdp.coinbase.com/swaps/docs/welcome).