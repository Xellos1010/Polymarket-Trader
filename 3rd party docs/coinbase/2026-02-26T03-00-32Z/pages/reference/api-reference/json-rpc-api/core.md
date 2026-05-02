# core

## Constructing Requests

**Option 1**: You can use the [JSON-RPC playground](https://portal.cdp.coinbase.com/products/node) to easily create sample JSON-RPC requests. **Option 2**: Alternatively, you can construct your own requests by following the below steps:

1.  Go to the [Node](https://portal.cdp.coinbase.com/products/node) page in the CDP Portal.
2.  Ensure the correct network in your RPC URL (either `base` or `base-sepolia`).
3.  Open a terminal and make your requests based on the supported methods in this API reference page.

## Batch Requests

Below is an example of a [batch request](https://www.jsonrpc.org/specification#batch) for a JSON-RPC API method.

> Batch request example of `eth_getTransactionReceipt`

```
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '[{"jsonrpc": "2.0", "id": 1, "method": "eth_getTransactionReceipt", "params": ["0x633982a26e0cfba940613c52b31c664fe977e05171e35f62da2426596007e249"]}, { "jsonrpc": "2.0", "id": 2, "method": "eth_getTransactionReceipt", "params": ["0x3a7d521b20b5684e0e9ec14aeebe8ccab67137f7d5c2589efb55b0625fcc9c6d"]}]'

```

## Ethereum Namespace

Below are example requests for JSON-RPC API methods in the `eth_*` namespace.

### `eth_blockNumber`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# eth_blockNumber: https://ethereum.org/en/developers/docs/apis/json-rpc#eth_blocknumber
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_blockNumber"}'

```

### `eth_getBlockByNumber`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# eth_getBlockByNumber: https://ethereum.org/en/developers/docs/apis/json-rpc#eth_getblockbynumber
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_getBlockByNumber", "params": ["0xdad3c1", false]}'

```

### `eth_getBlockByHash`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# eth_getBlockByHash: https://ethereum.org/en/developers/docs/apis/json-rpc#eth_getblockbyhash
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_getBlockByHash", "params": ["0x849a3ac8f0d81df1a645701cdb9f90e58500d2eabb80ff3b7f4e8c13f025eff2", false]}'

```

### `eth_getBlockTransactionCountByHash`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# eth_getBlockTransactionCountByHash: https://ethereum.org/en/developers/docs/apis/json-rpc#eth_getblocktransactioncountbyhash
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_getBlockTransactionCountByHash", "params": ["0x849a3ac8f0d81df1a645701cdb9f90e58500d2eabb80ff3b7f4e8c13f025eff2"]}'

```

### `eth_getBlockTransactionCountByNumber`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# eth_getBlockTransactionCountByNumber: https://ethereum.org/en/developers/docs/apis/json-rpc#eth_getblocktransactioncountbynumber
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_getBlockTransactionCountByNumber", "params": ["0xdad3c1"]}'

```

### `eth_getTransactionByHash`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# eth_getTransactionByHash: https://ethereum.org/en/developers/docs/apis/json-rpc#eth_gettransactionbyhash
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_getTransactionByHash", "params": ["0x633982a26e0cfba940613c52b31c664fe977e05171e35f62da2426596007e249"]}'

```

### `eth_getTransactionReceipt`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# eth_getTransactionReceipt: https://ethereum.org/en/developers/docs/apis/json-rpc#eth_gettransactionreceipt
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_getTransactionReceipt", "params": ["0x633982a26e0cfba940613c52b31c664fe977e05171e35f62da2426596007e249"]}'

```

### `eth_getTransactionByBlockHashAndIndex`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# eth_getTransactionByBlockHashAndIndex: https://ethereum.org/en/developers/docs/apis/json-rpc#eth_gettransactionbyblockhashandindex
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_getTransactionByBlockHashAndIndex", "params": ["0x849a3ac8f0d81df1a645701cdb9f90e58500d2eabb80ff3b7f4e8c13f025eff2", "0x0"]}'

```

### `eth_getTransactionByBlockNumberAndIndex`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# eth_getTransactionByBlockNumberAndIndex: https://ethereum.org/en/developers/docs/apis/json-rpc#eth_gettransactionbyblocknumberandindex
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_getTransactionByBlockNumberAndIndex", "params": ["0xdad3c1", "0x0"]}'

```

### `eth_getLogs`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 100.

```
# eth_getLogs: https://ethereum.org/en/developers/docs/apis/json-rpc#eth_getlogs
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_getLogs", "params": [{"fromBlock": "0xdad3c1", "toBlock": "0xdad3c2"}]}'

```

### `eth_call`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# eth_call: https://ethereum.org/en/developers/docs/apis/json-rpc#eth_call
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_call", "params":[{ "to": "0x514910771af9ca656af840dff83e8264ecf986ca", "data": "0x70a08231000000000000000000000000f27eee60abacb983251fea941dd7350280a538ba"}, "latest"]}'

```

### `eth_getBalance`

```
# eth_getBalance: https://ethereum.org/en/developers/docs/apis/json-rpc#eth_getbalance
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_getBalance", "params":["0x8d97689c9818892b700e27f316cc3e41e17fbeb9", "latest"]}'

```

### `eth_getCode`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# eth_getCode: https://ethereum.org/en/developers/docs/apis/json-rpc#eth_getcode
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_getCode", "params":["0x7f268357a8c2552623316e2562d90e642bb538e5", "latest"]}'

```

### `eth_getTransactionCount`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# eth_getTransactionCount: https://ethereum.org/en/developers/docs/apis/json-rpc#eth_gettransactioncount
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_getTransactionCount", "params":["0xe222489ae12e15713cc1d65dd0ab2f5b18721bfd", "latest"]}'

```

### `eth_chainId`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# eth_chainId: https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_chainid
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_chainId"}'

```

### `eth_sendRawTransaction`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# eth_sendRawTransaction: https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_sendrawtransaction
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_sendRawTransaction", "params": ["0xf889808609184e72a00082271094000000000000000000000000000000000000000080a47f74657374320000000000000000000000000000000000000000000000000000006000571ca08a8bbf888cfa37bbf0bb965423625641fc956967b81d12e23709cead01446075a01ce999b56a8a88504be365442ea61239198e23d1fce7d00fcfc5cd3b44b7215f"]}'

```

### `eth_gasPrice`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# eth_gasPrice: https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_gasprice
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_gasPrice"}'

```

### `eth_getStorageAt`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# eth_getStorageAt: https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_getstorageat
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_getStorageAt", "params": ["0x6c8f2a135f6ed072de4503bd7c4999a1a17f824b", "0x0", "latest"]}'

```

### `eth_estimateGas`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 100.

```
# eth_estimateGas: https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_estimategas
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_estimateGas", "params": [{"from": "0x8d97689c9818892b700e27f316cc3e41e17fbeb9", "to": "0xd3cda913deb6f67967b99d67acdfa1712c293601", "value": "0x1"}]}'

```

### `eth_protocolVersion`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# eth_protocolVersion: https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_protocolversion
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_protocolVersion"}'

```

### `eth_syncing`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# eth_syncing: https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_syncing
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_syncing"}'

```

### `eth_feeHistory`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 100.

```
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_feeHistory", "params": [4, "latest", [25, 75]]}'

```

### `eth_mining`

```
# eth_mining: https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_mining
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_mining"}'

```

### `eth_hashrate`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# eth_hashrate: https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_hashrate
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_hashrate"}'

```

### `eth_accounts`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# eth_accounts: https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_accounts
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_accounts"}'

```

### `eth_newFilter`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# eth_newFilter: https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_newfilter
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_newFilter", "params":[{}]}'

```

### `eth_newBlockFilter`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# eth_newBlockFilter: https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_newblockfilter
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_newBlockFilter"}'

```

### `eth_uninstallFilter`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# eth_uninstallFilter: https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_uninstallfilter
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_uninstallFilter", "params":["0x81440f9af726125cb7fc671eb0f2d8728d6ad699989a"]}'

```

### `eth_getFilterChanges`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# eth_getFilterChanges: https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_getfilterchanges
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_getFilterChanges", "params":["0x81440f9af726125cb7fc671eb0f2d8728d6ad699989a"]}'

```

### `eth_getFilterLogs`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 100.

```
# eth_getFilterLogs: https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_getfilterlogs
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_getFilterLogs", "params":["0x81440f9af726125cb7fc671eb0f2d8728d6ad699989a"]}'

```

### `eth_getFilteth_getWorkerLogs`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# eth_getFilteth_getWorkerLogs: https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_getwork
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_getWork"}'

```

### `eth_submitWork`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# eth_submitWork: https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_submitwork
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_submitWork", "params": ["0x0000000000000001", "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef", "0xD1FE5700000000000000000000000000D1FE5700000000000000000000000000"]}'

```

### `eth_submitHashrate`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# eth_submitHashrate: https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_submithashrate
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "eth_submitHashrate", "params":["0x500000", "0x59daa26581d0acd1fce254fb7e85952f4c09d0915afd33d3886cd914bc7d283c"]}'

```

## Debug Namespace

Below are example requests for JSON-RPC API method in the `debug_*` namespace.

### `debug_traceBlockByHash`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 500.

```
# debug_traceBlockByHash: https://geth.ethereum.org/docs/rpc/ns-debug#debug_traceblockbyhash
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "debug_traceBlockByHash", "params": ["0xe075488f2716495e97c43f6eb2994964074a70245cca5844b308479ccbbb9ae7", {"tracer": "callTracer"}]}'

```

### `debug_traceBlockByNumber`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 500.

```
# debug_traceBlockByNumber: https://geth.ethereum.org/docs/rpc/ns-debug#debug_traceblockbynumber
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "debug_traceBlockByNumber", "params": ["0xe11130", {"tracer": "callTracer"}]}'

```

### `debug_traceCall`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 500.

```
# debug_traceCall: https://geth.ethereum.org/docs/rpc/ns-debug#debug_traceCall
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"debug_traceCall","params":[{"from":"0x000000000000000000000000000000000000dead","to":"0x111111111111111111111111111111111111dead","gas":"0x30D40","gasPrice":"0x3B9ACA00","value":"0x0","data":"0xa9059cbb000000000000000000000000222222222222222222222222222222222222dead00000000000000000000000000000000000000000000000000000000000000ff"},"latest",{"tracer":"callTracer"}]}'

```

## Net Namespace

Below are example requests for JSON-RPC API method in the `net_*` namespace.

### `net_version`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# net_version: https://ethereum.org/en/developers/docs/apis/json-rpc#net_version
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "net_version"}'

```

### `net_listening`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# net_listening: https://ethereum.org/en/developers/docs/apis/json-rpc/#net_listening
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "net_listening"}'

```

### `net_peercount`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# net_peercount: https://ethereum.org/en/developers/docs/apis/json-rpc/#net_peercount
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "net_peerCount"}'

```

## Web3 Namespace

Below are example requests for JSON-RPC API methods in the `web3_*` namespace.

### `web3_clientVersion`

The [Billing Unit](https://developer.chrome.com/data/node/pricing) value of this method is 30.

```
# web3_clientVersion: https://ethereum.org/en/developers/docs/apis/json-rpc/#web3_clientversion
curl -s {Your_Endpoint_URL} -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "id": 1, "method": "web3_clientVersion"}'

```