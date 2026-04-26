# quickstart

Get started with CDP Node in minutes. This guide shows you how to get your RPC endpoint and make your first blockchain request—both in the browser playground and programmatically in your code.

## Prerequisites

-   A free [CDP account](https://portal.cdp.coinbase.com/)

That’s it! No complex setup, no infrastructure to manage.

## 1\. Try it in the playground

CDP Node provides RPC endpoints for Base. With Node, you can:

-   Read blockchain state (blocks, transactions, balances, smart contract data)
-   Send transactions to the network
-   Monitor events and subscribe to logs
-   Call smart contracts on Base

Let's make your first blockchain call using the **Node Playground** in CDP Portal.

## 2\. Get your RPC endpoint

To use Node in your application, you need an **RPC endpoint URL**. This is the web address where you send blockchain requests—think of it like an API endpoint, but specifically for blockchain operations.

## 3\. Make your first request

Now let’s make your first blockchain request programmatically. We’ll query the current block number on Base.

-   cURL
    
-   JavaScript (fetch)
    
-   Python
    
-   Node.js
    

```
curl https://api.developer.coinbase.com/rpc/v1/base/YOUR_CLIENT_API_KEY \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "eth_blockNumber"
  }'

```

Response:

```
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "0x12a4b2c"
}

```

```
const rpcUrl = "https://api.developer.coinbase.com/rpc/v1/base/YOUR_CLIENT_API_KEY";
const response = await fetch(rpcUrl, {
  method: "POST",
  headers: {
    "Content-Type": "application/json",
  },
  body: JSON.stringify({
    jsonrpc: "2.0",
    id: 1,
    method: "eth_blockNumber",
  }),
});
const data = await response.json();
console.log("Current block:", parseInt(data.result, 16));

```

```
import requests
import json
rpc_url = "https://api.developer.coinbase.com/rpc/v1/base/YOUR_CLIENT_API_KEY"
payload = {
    "jsonrpc": "2.0",
    "id": 1,
    "method": "eth_blockNumber"
}
response = requests.post(rpc_url, json=payload)
result = response.json()
# Convert hex to decimal
block_number = int(result["result"], 16)
print(f"Current block: {block_number}")

```

```
const https = require("https");
const rpcUrl = "https://api.developer.coinbase.com/rpc/v1/base/YOUR_CLIENT_API_KEY";
const payload = JSON.stringify({
  jsonrpc: "2.0",
  id: 1,
  method: "eth_blockNumber",
});
const options = {
  method: "POST",
  headers: {
    "Content-Type": "application/json",
  },
};
const req = https.request(rpcUrl, options, (res) => {
  let data = "";
  res.on("data", (chunk) => (data += chunk));
  res.on("end", () => {
    const result = JSON.parse(data);
    console.log("Current block:", parseInt(result.result, 16));
  });
});
req.write(payload);
req.end();

```

## What to read next

-   **[Core EVM Methods](https://developer.chrome.com/api-reference/json-rpc-api/core)**: Explore all available JSON-RPC methods
-   **[Paymaster Methods](https://developer.chrome.com/api-reference/json-rpc-api/paymaster)**: Learn how to sponsor gas fees for your users
-   **[Wallet History Methods](https://developer.chrome.com/api-reference/json-rpc-api/wallet-history)**: Query historical wallet data
-   **[Rate Limits](https://developer.chrome.com/data/node/overview#rate-limits)**: Understand your usage limits and request increases
-   **[CDP Discord](https://discord.com/channels/1220414409550336183/1222183017284501535)**: Join #node for support and to request rate limit increases