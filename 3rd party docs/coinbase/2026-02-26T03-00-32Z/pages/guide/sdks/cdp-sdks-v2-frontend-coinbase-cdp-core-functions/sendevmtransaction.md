# sendevmtransaction

```
function sendEvmTransaction(options: SendEvmTransactionOptions): Promise<SendEvmTransactionResult>;

```

Sends an EVM transaction.

## Parameters

Parameter

Type

Description

`options`

[`SendEvmTransactionOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SendEvmTransactionOptions)

The options for the sending.

## Returns

`Promise`<[`SendEvmTransactionResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SendEvmTransactionResult)\> The transaction hash of the sent transaction.

## Example

```
const user = await getCurrentUser();
const evmAccount = user?.evmAccountObjects[0]?.address;
const result = await sendEvmTransaction({
  evmAccount,
  network: "base-sepolia",
  transaction: {
    to: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    value: 100000000000000n, // 0.0001 ETH in wei
    nonce: 0,
    gas: 21000n,
    maxFeePerGas: 30000000000n,
    maxPriorityFeePerGas: 1000000000n,
    chainId: 84532, // Base Sepolia
    type: "eip1559",
  }
});

```