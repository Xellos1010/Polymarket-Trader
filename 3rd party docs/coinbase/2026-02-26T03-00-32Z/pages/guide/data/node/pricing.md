# pricing

## Overview

CDP Node API pricing operates on a pay-as-you-go model based on Billing Units (BU). BUs are calculated per method based on computational complexity and data requirements. You can get started now with 10M BUs free per month. As you grow, you only pay for what you use. To help you estimate costs, the following lists detail BUs per each JSON RPC method.

## Billing Units by Method

### Standard Methods (30 BU)

Most common JSON-RPC methods for basic blockchain interactions.

-   `eth_accounts`
-   `eth_blockNumber`
-   `eth_call`
-   `eth_chainId`
-   `eth_createAccessList`
-   `eth_feeHistory`
-   `eth_gasPrice`
-   `eth_getBalance`
-   `eth_getBlockByHash`
-   `eth_getBlockTransactionCountByHash`
-   `eth_getBlockTransactionCountByNumber`
-   `eth_getCode`
-   `eth_getFilterChanges`
-   `eth_getProof`
-   `eth_getStorageAt`
-   `eth_getTransactionByBlockHashAndIndex`
-   `eth_getTransactionByBlockNumberAndIndex`
-   `eth_getTransactionByHash`
-   `eth_getTransactionCount`
-   `eth_getUncleByBlockHashAndIndex`
-   `eth_getUncleByBlockNumberAndIndex`
-   `eth_getUncleCountByBlockHash`
-   `eth_getUncleCountByBlockNumber`
-   `eth_maxPriorityFeePerGas`
-   `eth_newBlockFilter`
-   `eth_newFilter`
-   `eth_newPendingTransactionFilter`
-   `eth_protocolVersion`
-   `eth_simulateV1`
-   `eth_subscribe`
-   `eth_syncing`
-   `eth_uninstallFilter`
-   `eth_unsubscribe`
-   `net_listening`
-   `net_version`
-   `web3_clientVersion`
-   `web3_sha3`

### Enhanced Methods (100 BU)

Methods requiring more computational resources for filtering and receipts.

-   `eth_estimateGas`
-   `eth_getBlockByNumber`
-   `eth_getBlockReceipts`
-   `eth_getFilterLogs`
-   `eth_getLogs`
-   `eth_getTransactionReceipt`

### Advanced Methods (500 BU)

Complex operations including traces, debugging, and transaction submission.

-   `debug_traceBlockByHash`
-   `debug_traceBlockByNumber`
-   `debug_traceCall`
-   `debug_traceTransaction`
-   `eth_estimateUserOperationGas`
-   `eth_sendRawTransaction`
-   `trace_block`
-   `trace_call`
-   `trace_filter`
-   `trace_get`
-   `trace_rawTransaction`
-   `trace_replayBlockTransactions`
-   `trace_replayTransaction`
-   `trace_transaction`

## Free Tier

Every project gets **10 million Billing Units free per month**. Your free tier resets on the first day of each calendar month—unused BUs do not roll over.

What does 10M BU get you?

The free tier is designed to support development, testing, and many production applications. Here’s what 10 million BUs translates to in actual API calls:

-   **~333,000 calls** if you only used 30 BU methods (like `eth_call`, `eth_getBalance`)
-   **~100,000 calls** if you only used 100 BU methods (like `eth_getLogs`, `eth_getTransactionReceipt`)
-   **~20,000 calls** if you only used 500 BU methods (like `debug_traceTransaction`, `eth_sendRawTransaction`)

In practice, most applications use a mix of different methods, so your total request count will vary based on which methods you call most frequently.

### Usage beyond the free tier

Once you exceed 10 million BUs in a month, you’ll be billed **$0.50 per million additional BUs**.

### Monitoring your usage

View your billing history in the [CDP Portal billing page](https://portal.cdp.coinbase.com/projects/billing).