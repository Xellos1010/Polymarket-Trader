# migration guide

This guide helps you migrate from x402 v1 to v2. The v2 protocol introduces standardized identifiers, improved type safety, and a more modular architecture.

## Overview

Aspect

v1

v2

Payment Header

`X-PAYMENT`

`PAYMENT-SIGNATURE`

Response Header

`X-PAYMENT-RESPONSE`

`PAYMENT-RESPONSE`

Network Format

String (`base-sepolia`)

CAIP-2 (`eip155:84532`)

Version Field

`x402Version: 1`

`x402Version: 2`

Packages

`x402`, `x402-express`, `x402-axios`

`@x402/core`, `@x402/express`, `@x402/axios`, `@x402/evm`

## Compatibility Matrix

Use this matrix to understand which SDK versions work with which facilitators and protocol versions.

### SDK Packages

#### Legacy (designed for v1)

Package

Recommended Migration

`x402`

`@x402/core`

`x402-express`

`@x402/express`

`x402-next`

`@x402/next`

`x402-hono`

`@x402/hono`

`x402-fetch`

`@x402/fetch`

`x402-axios`

`@x402/axios`

#### Current (use v2 by default)

### Facilitators

Facilitator

v1

v2

Networks

`https://www.x402.org/facilitator`

✅

✅

Base Sepolia, Solana Devnet (testnet only)

`https://api.cdp.coinbase.com/platform/v2/x402`

✅

✅

Base, Base Sepolia, Solana, Solana Devnet

Purpose

v1 Header

v2 Header

Payment signature (client → server)

`X-PAYMENT`

`PAYMENT-SIGNATURE`

Payment response (server → client)

`X-PAYMENT-RESPONSE`

`PAYMENT-RESPONSE`

Payment requirements (server → client)

(in body)

`PAYMENT-REQUIRED`

## For Buyers

### Before (v1)

-   TypeScript
    
-   Python
    

```
import { withPaymentInterceptor } from "x402-axios";
import { createWalletClient, http } from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { baseSepolia } from "viem/chains";
import axios from "axios";
const account = privateKeyToAccount(process.env.PRIVATE_KEY as `0x${string}`);
const walletClient = createWalletClient({
  account,
  chain: baseSepolia,
  transport: http(),
});
// v1 pattern
const api = withPaymentInterceptor(
  axios.create({ baseURL: "https://api.example.com" }),
  walletClient,
);
const response = await api.get("/paid-endpoint");

```

```
import os
from eth_account import Account
from x402.clients.httpx import x402HttpxClient
# v1 pattern
account = Account.from_key(os.getenv("PRIVATE_KEY"))
async with x402HttpxClient(account=account, base_url="https://api.example.com") as client:
    response = await client.get("/protected-endpoint")
    print(await response.aread())

```

### After (v2)

-   TypeScript
    
-   Python
    

```
import { x402Client, wrapAxiosWithPayment } from "@x402/axios";
import { registerExactEvmScheme } from "@x402/evm/exact/client";
import { privateKeyToAccount } from "viem/accounts";
import axios from "axios";
const signer = privateKeyToAccount(process.env.EVM_PRIVATE_KEY as `0x${string}`);
// v2 pattern: Create client and register scheme separately
const client = new x402Client();
registerExactEvmScheme(client, { signer });
// Wrap axios with payment handling
const api = wrapAxiosWithPayment(
  axios.create({ baseURL: "https://api.example.com" }),
  client,
);
const response = await api.get("/paid-endpoint");

```

```
import asyncio
import os
from eth_account import Account
from x402 import x402Client
from x402.http import x402HTTPClient
from x402.http.clients import x402HttpxClient
from x402.mechanisms.evm import EthAccountSigner
from x402.mechanisms.evm.exact.register import register_exact_evm_client
async def main() -> None:
    # v2 pattern: Create client and register scheme separately
    client = x402Client()
    account = Account.from_key(os.getenv("EVM_PRIVATE_KEY"))
    register_exact_evm_client(client, EthAccountSigner(account))
    http_client = x402HTTPClient(client)
    async with x402HttpxClient(client) as http:
        response = await http.get("https://api.example.com/paid-endpoint")
        await response.aread()
        print(f"Response: {response.text}")
        if response.is_success:
            settle_response = http_client.get_payment_settle_response(
                lambda name: response.headers.get(name)
            )
            print(f"Payment settled: {settle_response}")
asyncio.run(main())

```

### Key Changes

-   TypeScript
    
-   Python
    

1.  **Package rename**: `x402-axios` → `@x402/axios`
2.  **Function rename**: `withPaymentInterceptor` → `wrapAxiosWithPayment`
3.  **Wallet setup**: Use `x402Client` with `registerExactEvmScheme` helper instead of passing wallet directly
4.  **No chain-specific configuration**: The v2 client automatically handles network selection based on payment requirements

1.  **Import path changes**: `x402.clients.httpx` → `x402.http.clients`
2.  **Signer wrapper**: Wrap `eth_account.Account` with `EthAccountSigner`
3.  **Client construction**: Create `x402Client()` first, then register schemes
4.  **Environment variable**: `PRIVATE_KEY` → `EVM_PRIVATE_KEY`
5.  **Async/Sync variants**: Use `x402Client` for httpx (async), `x402ClientSync` for requests (sync)

## For Sellers

### Before (v1)

-   TypeScript
    
-   Python
    

```
import { paymentMiddleware, FacilitatorConfig } from "x402-express";
import express from "express";
const app = express();
const facilitatorConfig: FacilitatorConfig = {
  url: "https://x402.org/facilitator",
};
app.use(
  paymentMiddleware(facilitatorConfig, {
    "GET /weather": {
      price: "$0.001",
      network: "base-sepolia", // v1 string format
      config: {
        description: "Get weather data",
      },
    },
  }),
);

```

```
from typing import Any, Dict
from fastapi import FastAPI
from x402.fastapi.middleware import require_payment
app = FastAPI()
# v1 pattern
app.middleware("http")(
    require_payment(
        path="/weather",
        price="$0.001",
        pay_to_address="0xYourAddress",
        network="base-sepolia",  # v1 string identifier
    )
)
@app.get("/weather")
async def get_weather() -> Dict[str, Any]:
    return {"report": {"weather": "sunny", "temperature": 70}}

```

### After (v2)

-   TypeScript
    
-   Python
    

```
import express from "express";
import { paymentMiddleware } from "@x402/express";
import { x402ResourceServer, HTTPFacilitatorClient } from "@x402/core/server";
import { registerExactEvmScheme } from "@x402/evm/exact/server";
const app = express();
const payTo = "0xYourAddress";
// v2 pattern: Create facilitator client and resource server
const facilitatorClient = new HTTPFacilitatorClient({
  url: "https://x402.org/facilitator"
});
const server = new x402ResourceServer(facilitatorClient);
registerExactEvmScheme(server);
app.use(
  paymentMiddleware(
    {
      "GET /weather": {
        accepts: [
          {
            scheme: "exact",
            price: "$0.001",
            network: "eip155:84532", // v2 CAIP-2 format
            payTo,
          },
        ],
        description: "Get weather data",
        mimeType: "application/json",
      },
    },
    server,
  ),
);

```

```
from typing import Any
from fastapi import FastAPI
from x402.http import FacilitatorConfig, HTTPFacilitatorClient, PaymentOption
from x402.http.middleware.fastapi import PaymentMiddlewareASGI
from x402.http.types import RouteConfig
from x402.mechanisms.evm.exact import ExactEvmServerScheme
from x402.server import x402ResourceServer
app = FastAPI()
# v2 pattern: Create facilitator client and resource server
facilitator = HTTPFacilitatorClient(
    FacilitatorConfig(url="https://x402.org/facilitator")
)
server = x402ResourceServer(facilitator)
server.register("eip155:84532", ExactEvmServerScheme())
# v2: Route config uses accepts array with explicit scheme, network, and pay_to
routes: dict[str, RouteConfig] = {
    "GET /weather": RouteConfig(
        accepts=[
            PaymentOption(
                scheme="exact",
                pay_to="0xYourAddress",
                price="$0.001",
                network="eip155:84532",  # v2 CAIP-2 format
            ),
        ],
        mime_type="application/json",
        description="Get weather data",
    ),
}
app.add_middleware(PaymentMiddlewareASGI, routes=routes, server=server)
@app.get("/weather")
async def get_weather() -> dict[str, Any]:
    return {"report": {"weather": "sunny", "temperature": 70}}

```

### Key Changes

-   TypeScript
    
-   Python
    

1.  **Package rename**: `x402-express` → `@x402/express`
2.  **Configuration structure**: Route config now uses `accepts` array with explicit `scheme`, `network`, and `payTo`
3.  **Network format**: `base-sepolia` → `eip155:84532` (CAIP-2 standard)
4.  **Resource server**: Create `x402ResourceServer` with facilitator client and register schemes using helper functions
5.  **Price recipient**: Explicitly specify `payTo` address per route

1.  **Import path changes**: `x402.fastapi.middleware` → `x402.http.middleware.fastapi`
2.  **Middleware pattern**: `require_payment` decorator → `PaymentMiddlewareASGI` class
3.  **Configuration structure**: Route config now uses `RouteConfig` and `PaymentOption` Pydantic models
4.  **Network format**: `base-sepolia` → `eip155:84532` (CAIP-2 standard)
5.  **Resource server**: Create `x402ResourceServer` and register schemes explicitly
6.  **Type hints**: Use modern Python type hints (`dict[str, Any]` instead of `Dict[str, Any]`)
7.  **Async/Sync variants**: Use `x402ResourceServer` + `HTTPFacilitatorClient` for FastAPI (async), use `x402ResourceServerSync` + `HTTPFacilitatorClientSync` for Flask (sync)

## Network Identifier Mapping

v1 Name

v2 CAIP-2 ID

Chain ID

Description

`base-sepolia`

`eip155:84532`

84532

Base Sepolia Testnet

`base`

`eip155:8453`

8453

Base Mainnet

`ethereum`

`eip155:1`

1

Ethereum Mainnet

`sepolia`

`eip155:11155111`

11155111

Ethereum Sepolia Testnet

`solana-devnet`

`solana:devnet`

\-

Solana Devnet

`solana`

`solana:mainnet`

\-

Solana Mainnet

## Package Migration Reference

-   TypeScript
    
-   Python
    

v1 Package

v2 Package(s)

`x402`

`@x402/core`

`x402-express`

`@x402/express`

`x402-axios`

`@x402/axios`

`x402-fetch`

`@x402/fetch`

`x402-hono`

`@x402/hono`

`x402-next`

`@x402/next`

(built-in)

`@x402/evm` (EVM support)

(built-in)

`@x402/svm` (Solana support)

v1 Import Path

v2 Import Path

`x402.clients.httpx`

`x402.http.clients.x402HttpxClient`

`x402.clients.requests`

`x402.http.clients.x402_requests`

`x402.fastapi.middleware`

`x402.http.middleware.fastapi`

`x402.flask.middleware`

`x402.http.middleware.flask`

`x402.facilitator`

`x402.http.HTTPFacilitatorClient`

(new)

`x402.mechanisms.evm.EthAccountSigner`

(new)

`x402.mechanisms.evm.exact.register_exact_evm_client`

(new)

`x402.mechanisms.svm.KeypairSigner`

(new)

`x402.mechanisms.svm.exact.register_exact_svm_client`

(new)

`x402.server.x402ResourceServer`

**Installation with extras:**

```
# v1
pip install x402
# v2 - install with specific extras
pip install "x402[httpx]"      # For async HTTP clients
pip install "x402[requests]"   # For sync HTTP clients
pip install "x402[fastapi]"    # For FastAPI servers
pip install "x402[flask]"      # For Flask servers
pip install "x402[svm]"        # For Solana support

```

If you’re implementing custom HTTP handling, update your header names:

-   TypeScript
    
-   Python
    

```
// v1
const payment = req.header("X-PAYMENT");
res.setHeader("X-PAYMENT-RESPONSE", responseData);
// v2
const payment = req.header("PAYMENT-SIGNATURE");
res.setHeader("PAYMENT-RESPONSE", responseData);

```

```
# v1
payment = request.headers.get("X-PAYMENT")
response.headers["X-PAYMENT-RESPONSE"] = response_data
# v2
payment = request.headers.get("PAYMENT-SIGNATURE")
response.headers["PAYMENT-RESPONSE"] = response_data

```

## Troubleshooting

-   TypeScript
    
-   Python
    

### ”Cannot find module” errors

Ensure you’ve installed all v2 packages:

```
# For buyers
npm install @x402/axios @x402/evm
# For sellers (Express)
npm install @x402/express @x402/core @x402/evm

```

### “ModuleNotFoundError” errors

Ensure you’ve installed the x402 package with the correct extras:

```
# For async HTTP clients (httpx)
pip install "x402[httpx]"
# For sync HTTP clients (requests)
pip install "x402[requests]"
# For FastAPI servers
pip install "x402[fastapi]"
# For Flask servers
pip install "x402[flask]"
# For Solana support
pip install "x402[svm]"

```

### Payment verification failures

-   Check you’re using CAIP-2 network identifiers (`eip155:84532` not `base-sepolia`)
-   Verify your `payTo` address is correctly configured
-   Ensure the facilitator URL is correct for your network (testnet vs mainnet)

### Mixed v1/v2 compatibility

The facilitator supports both v1 and v2 protocols. During migration, your v2 server can still accept payments from v1 clients, but we recommend updating clients to v2 for full feature support.

## Next Steps

-   [Quickstart for Buyers](https://developer.chrome.com/x402/quickstart-for-buyers)
-   [Quickstart for Sellers](https://developer.chrome.com/x402/quickstart-for-sellers)
-   [Network Support](https://developer.chrome.com/x402/network-support)