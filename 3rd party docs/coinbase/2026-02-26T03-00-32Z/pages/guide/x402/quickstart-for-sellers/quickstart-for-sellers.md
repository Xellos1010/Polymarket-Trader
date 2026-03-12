# quickstart for sellers

This guide walks you through integrating with x402 to enable payments for your API or service. By the end, your API will be able to charge buyers and AI agents for access.

## Prerequisites

Before you begin, ensure you have:

-   A crypto wallet to receive funds (any EVM-compatible wallet, e.g., [CDP Wallet](https://developer.chrome.com/server-wallets/v2/introduction/quickstart))
-   (Optional) A [Coinbase Developer Platform](https://cdp.coinbase.com/) (CDP) account and API Keys
    -   Required for mainnet use until other facilitators go live
-   [Node.js](https://nodejs.org/en) and npm, [Go](https://go.dev/), or Python and pip installed
-   An existing API or server

## 1\. Install Dependencies

-   Node.js
    
-   Go
    
-   Python
    

-   Express
    
-   Next.js
    
-   Hono
    

Install the x402 Express middleware and EVM mechanism packages:

```
npm install @x402/express @x402/evm @x402/core

```

Install the x402 Next.js middleware and EVM mechanism packages:

```
npm install @x402/next @x402/evm @x402/core

```

Install the x402 Hono middleware and EVM mechanism packages:

```
npm install @x402/hono @x402/evm @x402/core

```

Add the x402 Go module to your project:

```
go get github.com/coinbase/x402/go

```

-   FastAPI
    
-   Flask
    

## 2\. Add Payment Middleware

Integrate the payment middleware into your application. You will need to provide:

-   The Facilitator URL or facilitator client. For testing, use `https://www.x402.org/facilitator` which works on Base Sepolia and Solana Devnet.
    -   For mainnet setup, see [Running on Mainnet](#running-on-mainnet)
-   The routes you want to protect
-   Your receiving wallet address

-   Node.js
    
-   Go
    
-   Python
    

-   Express
    
-   Next.js
    
-   Hono
    

Full example in the repo [here](https://github.com/coinbase/x402/tree/main/examples/typescript/servers/express).

```
import express from "express";
import { paymentMiddleware, x402ResourceServer } from "@x402/express";
import { ExactEvmScheme } from "@x402/evm/exact/server";
import { HTTPFacilitatorClient } from "@x402/core/server";
const app = express();
// Your receiving wallet address
const payTo = "0xYourAddress";
// Create facilitator client (testnet)
const facilitatorClient = new HTTPFacilitatorClient({
  url: "https://www.x402.org/facilitator"
});
// Create resource server and register EVM scheme
const server = new x402ResourceServer(facilitatorClient)
  .register("eip155:84532", new ExactEvmScheme());
app.use(
  paymentMiddleware(
    {
      "GET /weather": {
        accepts: [
          {
            scheme: "exact",
            price: "$0.001", // USDC amount in dollars
            network: "eip155:84532", // Base Sepolia (CAIP-2 format)
            payTo,
          },
        ],
        description: "Get current weather data for any location",
        mimeType: "application/json",
      },
    },
    server,
  ),
);
// Implement your route
app.get("/weather", (req, res) => {
  res.send({
    report: {
      weather: "sunny",
      temperature: 70,
    },
  });
});
app.listen(4021, () => {
  console.log(`Server listening at http://localhost:4021`);
});

```

Full example in the repo [here](https://github.com/coinbase/x402/tree/main/examples/typescript/fullstack/next).

```
// middleware.ts
import { paymentProxy, x402ResourceServer } from "@x402/next";
import { ExactEvmScheme } from "@x402/evm/exact/server";
import { HTTPFacilitatorClient } from "@x402/core/server";
const payTo = "0xYourAddress";
const facilitatorClient = new HTTPFacilitatorClient({
  url: "https://www.x402.org/facilitator"
});
const server = new x402ResourceServer(facilitatorClient)
  .register("eip155:84532", new ExactEvmScheme());
export const middleware = paymentProxy(
  {
    "/api/protected": {
      accepts: [
        {
          scheme: "exact",
          price: "$0.01",
          network: "eip155:84532",
          payTo,
        },
      ],
      description: "Access to protected content",
      mimeType: "application/json",
    },
  },
  server,
);
export const config = {
  matcher: ["/api/protected/:path*"],
};

```

Full example in the repo [here](https://github.com/coinbase/x402/tree/main/examples/typescript/servers/hono).

```
import { Hono } from "hono";
import { serve } from "@hono/node-server";
import { paymentMiddleware, x402ResourceServer } from "@x402/hono";
import { ExactEvmScheme } from "@x402/evm/exact/server";
import { HTTPFacilitatorClient } from "@x402/core/server";
const app = new Hono();
const payTo = "0xYourAddress";
const facilitatorClient = new HTTPFacilitatorClient({
  url: "https://www.x402.org/facilitator"
});
const server = new x402ResourceServer(facilitatorClient)
  .register("eip155:84532", new ExactEvmScheme());
app.use(
  paymentMiddleware(
    {
      "/protected-route": {
        accepts: [
          {
            scheme: "exact",
            price: "$0.10",
            network: "eip155:84532",
            payTo,
          },
        ],
        description: "Access to premium content",
        mimeType: "application/json",
      },
    },
    server,
  ),
);
app.get("/protected-route", (c) => {
  return c.json({ message: "This content is behind a paywall" });
});
serve({ fetch: app.fetch, port: 3000 });

```

-   Gin
    

Full example in the repo [here](https://github.com/coinbase/x402/tree/main/examples/go/servers/gin).

```
package main
import (
    "net/http"
    "time"
    x402 "github.com/coinbase/x402/go"
    x402http "github.com/coinbase/x402/go/http"
    ginmw "github.com/coinbase/x402/go/http/gin"
    evm "github.com/coinbase/x402/go/mechanisms/evm/exact/server"
    "github.com/gin-gonic/gin"
)
func main() {
    payTo := "0xYourAddress"
    network := x402.Network("eip155:84532") // Base Sepolia (CAIP-2 format)
    r := gin.Default()
    // Create facilitator client
    facilitatorClient := x402http.NewHTTPFacilitatorClient(&x402http.FacilitatorConfig{
        URL: "https://www.x402.org/facilitator",
    })
    // Apply x402 payment middleware
    r.Use(ginmw.X402Payment(ginmw.Config{
        Routes: x402http.RoutesConfig{
            "GET /weather": {
                Scheme:      "exact",
                PayTo:       payTo,
                Price:       "$0.001",
                Network:     network,
                Description: "Get weather data for a city",
                MimeType:    "application/json",
            },
        },
        Facilitator: facilitatorClient,
        Schemes: []ginmw.SchemeConfig{
            {Network: network, Server: evm.NewExactEvmScheme()},
        },
        Initialize: true,
        Timeout:    30 * time.Second,
    }))
    // Protected endpoint
    r.GET("/weather", func(c *gin.Context) {
        c.JSON(http.StatusOK, gin.H{
            "weather":     "sunny",
            "temperature": 70,
        })
    })
    r.Run(":4021")
}

```

-   FastAPI
    
-   Flask
    

Full example in the repo [here](https://github.com/coinbase/x402/tree/main/examples/python/servers/fastapi).

```
from typing import Any
from fastapi import FastAPI
from x402.http import FacilitatorConfig, HTTPFacilitatorClient, PaymentOption
from x402.http.middleware.fastapi import PaymentMiddlewareASGI
from x402.http.types import RouteConfig
from x402.mechanisms.evm.exact import ExactEvmServerScheme
from x402.server import x402ResourceServer
app = FastAPI()
# Your receiving wallet address
pay_to = "0xYourAddress"
# Create facilitator client (testnet)
facilitator = HTTPFacilitatorClient(
    FacilitatorConfig(url="https://x402.org/facilitator")
)
# Create resource server and register EVM scheme
server = x402ResourceServer(facilitator)
server.register("eip155:84532", ExactEvmServerScheme())
# Define protected routes
routes: dict[str, RouteConfig] = {
    "GET /weather": RouteConfig(
        accepts=[
            PaymentOption(
                scheme="exact",
                pay_to=pay_to,
                price="$0.001",  # USDC amount in dollars
                network="eip155:84532",  # Base Sepolia (CAIP-2 format)
            ),
        ],
        mime_type="application/json",
        description="Get current weather data for any location",
    ),
}
# Add payment middleware
app.add_middleware(PaymentMiddlewareASGI, routes=routes, server=server)
@app.get("/weather")
async def get_weather() -> dict[str, Any]:
    return {
        "report": {
            "weather": "sunny",
            "temperature": 70,
        }
    }
if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=4021)

```

Full example in the repo [here](https://github.com/coinbase/x402/tree/main/examples/python/servers/flask).

```
from flask import Flask, jsonify
from x402.http import FacilitatorConfig, HTTPFacilitatorClientSync, PaymentOption
from x402.http.middleware.flask import payment_middleware
from x402.http.types import RouteConfig
from x402.mechanisms.evm.exact import ExactEvmServerScheme
from x402.server import x402ResourceServerSync
app = Flask(__name__)
pay_to = "0xYourAddress"
facilitator = HTTPFacilitatorClientSync(
    FacilitatorConfig(url="https://x402.org/facilitator")
)
server = x402ResourceServerSync(facilitator)
server.register("eip155:84532", ExactEvmServerScheme())
routes: dict[str, RouteConfig] = {
    "GET /weather": RouteConfig(
        accepts=[
            PaymentOption(
                scheme="exact",
                pay_to=pay_to,
                price="$0.001",
                network="eip155:84532",
            ),
        ],
        mime_type="application/json",
        description="Get current weather data for any location",
    ),
}
payment_middleware(app, routes=routes, server=server)
@app.route("/weather")
def get_weather():
    return jsonify({
        "report": {
            "weather": "sunny",
            "temperature": 70,
        }
    })
if __name__ == "__main__":
    app.run(host="0.0.0.0", port=4021)

```

**Route Configuration Interface:**

```
interface RouteConfig {
  accepts: Array<{
    scheme: string; // Payment scheme (e.g., "exact")
    price: string; // Price in dollars (e.g., "$0.01")
    network: string; // Network in CAIP-2 format (e.g., "eip155:84532")
    payTo: string; // Your wallet address
  }>;
  description?: string; // Description of the resource
  mimeType?: string; // MIME type of the response
  extensions?: object; // Optional extensions (e.g., Bazaar)
}

```

When a request is made to these routes without payment, your server will respond with the HTTP 402 Payment Required code and payment instructions.

## 3\. Test Your Integration

To verify:

1.  Make a request to your endpoint (e.g., `curl http://localhost:4021/weather`).
2.  The server responds with a 402 Payment Required, including payment instructions in the `PAYMENT-REQUIRED` header.
3.  Complete the payment using a compatible client, wallet, or automated agent. This typically involves signing a payment payload, which is handled by the client SDK detailed in the [Quickstart for Buyers](https://developer.chrome.com/x402/quickstart-for-buyers).
4.  Retry the request, this time including the `PAYMENT-SIGNATURE` header containing the cryptographic proof of payment.
5.  The server verifies the payment via the facilitator and, if valid, returns your actual API response (e.g., `{ "data": "Your paid API response." }`).

When using the CDP facilitator, your endpoints can be listed in the [x402 Bazaar](https://developer.chrome.com/x402/bazaar), our discovery layer that helps buyers and AI agents find services. To enable discovery and improve visibility:

Example with Bazaar extension:

```
{
  "GET /weather": {
    accepts: [
      {
        scheme: "exact",
        price: "$0.001",
        network: "eip155:8453",
        payTo: "0xYourAddress",
      },
    ],
    description: "Get real-time weather data including temperature, conditions, and humidity",
    mimeType: "application/json",
    extensions: {
      bazaar: {
        discoverable: true,
        category: "weather",
        tags: ["forecast", "real-time"],
      },
    },
  },
}

```

Learn more about the discovery layer in the [x402 Bazaar documentation](https://developer.chrome.com/x402/bazaar).

## 5\. Error Handling

-   If you run into trouble, check out the examples in the [repo](https://github.com/coinbase/x402/tree/main/examples) for more context and full code.
-   Run `npm install` or `go mod tidy` to install dependencies

## Running on Mainnet

Once you’ve tested your integration on testnet, you’re ready to accept real payments on mainnet.

### Setting Up CDP Facilitator for Production

CDP’s facilitator provides enterprise-grade payment processing with compliance features:

### 1\. Set up CDP API Keys

To use the mainnet facilitator, you’ll need a Coinbase Developer Platform account:

1.  Sign up at [cdp.coinbase.com](https://cdp.coinbase.com/)
2.  Create a new project
3.  Generate API credentials
4.  Set the following environment variables:
    
    ```
    CDP_API_KEY_ID=your-api-key-id
    CDP_API_KEY_SECRET=your-api-key-secret
    
    ```
    

### 2\. Update Your Code

Replace the testnet configuration with mainnet settings:

-   Node.js
    
-   Go
    
-   Python (FastAPI)
    
-   Python (Flask)
    

```
import { paymentMiddleware, x402ResourceServer } from "@x402/express";
import { ExactEvmScheme } from "@x402/evm/exact/server";
import { HTTPFacilitatorClient } from "@x402/core/server";
import { facilitator } from "@coinbase/x402";
const facilitatorClient = new HTTPFacilitatorClient(facilitator);
const server = new x402ResourceServer(facilitatorClient)
  .register("eip155:8453", new ExactEvmScheme()); // Base mainnet
app.use(
  paymentMiddleware(
    {
      "GET /weather": {
        accepts: [
          {
            scheme: "exact",
            price: "$0.001",
            network: "eip155:8453", // Base mainnet (CAIP-2)
            payTo: "0xYourAddress",
          },
        ],
        description: "Weather data",
        mimeType: "application/json",
      },
    },
    server,
  ),
);

```

```
// Update network to mainnet
network := x402.Network("eip155:8453") // Base mainnet (CAIP-2)
// Create facilitator client for mainnet
facilitatorClient := x402http.NewHTTPFacilitatorClient(&x402http.FacilitatorConfig{
    URL: "https://api.cdp.coinbase.com/platform/v2/x402",
    // Add auth if required
})
r.Use(ginmw.X402Payment(ginmw.Config{
    Routes: x402http.RoutesConfig{
        "GET /weather": {
            Scheme:  "exact",
            PayTo:   payTo,
            Price:   "$0.001",
            Network: network,
        },
    },
    Facilitator: facilitatorClient,
    Schemes: []ginmw.SchemeConfig{
        {Network: network, Server: evm.NewExactEvmScheme()},
    },
}))

```

```
from x402.http import FacilitatorConfig, HTTPFacilitatorClient
facilitator = HTTPFacilitatorClient(
    FacilitatorConfig(url="https://api.cdp.coinbase.com/platform/v2/x402")
)

```

```
from x402.http import FacilitatorConfig, HTTPFacilitatorClientSync
facilitator = HTTPFacilitatorClientSync(
    FacilitatorConfig(url="https://api.cdp.coinbase.com/platform/v2/x402")
)

```

### 3\. Update Your Wallet

Make sure your receiving wallet address is a real mainnet address where you want to receive USDC payments.

### 4\. Test with Real Payments

Before going live:

1.  Test with small amounts first
2.  Verify payments are arriving in your wallet
3.  Monitor the facilitator for any issues

### Using Different Networks

CDP facilitator supports multiple networks. Simply change the network parameter using CAIP-2 format:

-   Base Network
    
-   Solana Network
    
-   Multi-Network
    

```
// Base mainnet
{
  scheme: "exact",
  price: "$0.001",
  network: "eip155:8453", // Base mainnet
  payTo: "0xYourAddress",
}
// Base Sepolia testnet
{
  scheme: "exact",
  price: "$0.001",
  network: "eip155:84532", // Base Sepolia
  payTo: "0xYourAddress",
}

```

```
// Solana mainnet
{
  scheme: "exact",
  price: "$0.001",
  network: "solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp", // Solana mainnet
  payTo: "YourSolanaWalletAddress",
}
// Solana devnet
{
  scheme: "exact",
  price: "$0.001",
  network: "solana:EtWTRABZaYq6iMfeYKouRu166VU2xqa1", // Solana devnet
  payTo: "YourSolanaWalletAddress",
}

```

```
// Support multiple networks on the same endpoint
{
  "GET /weather": {
    accepts: [
      {
        scheme: "exact",
        price: "$0.001",
        network: "eip155:8453",
        payTo: "0xYourEvmAddress",
      },
      {
        scheme: "exact",
        price: "$0.001",
        network: "solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp",
        payTo: "YourSolanaAddress",
      },
    ],
    description: "Weather data",
  },
}

```

## Network Identifiers (CAIP-2)

x402 v2 uses [CAIP-2](https://github.com/ChainAgnostic/CAIPs/blob/main/CAIPs/caip-2.md) format for network identifiers:

Network

CAIP-2 Identifier

Base Mainnet

`eip155:8453`

Base Sepolia

`eip155:84532`

Solana Mainnet

`solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp`

Solana Devnet

`solana:EtWTRABZaYq6iMfeYKouRu166VU2xqa1`

See [Network Support](https://developer.chrome.com/x402/network-support) for the full list.

## Next Steps

-   Looking for something more advanced? Check out the [Advanced Example](https://github.com/coinbase/x402/tree/main/examples/typescript/servers/advanced)
-   Get started as a [buyer](https://developer.chrome.com/x402/quickstart-for-buyers)
-   Learn about the [Bazaar discovery layer](https://developer.chrome.com/x402/bazaar)

For questions or support, join our [Discord](https://discord.gg/cdp).

## Summary

This quickstart covered:

-   Installing the x402 SDK and relevant middleware
-   Adding payment middleware to your API and configuring it
-   Testing your integration
-   Deploying to mainnet with CAIP-2 network identifiers

Your API is now ready to accept crypto payments through x402.