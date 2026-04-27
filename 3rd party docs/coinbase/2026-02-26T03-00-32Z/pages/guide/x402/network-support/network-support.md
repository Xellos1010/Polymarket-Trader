# network support

x402 supports multiple blockchain networks for payment processing. This page covers what the x402 reference SDKs support, what the CDP facilitator is configured for, and where to find other facilitators.

## Network Identifiers (CAIP-2)

x402 v2 uses [CAIP-2](https://github.com/ChainAgnostic/CAIPs/blob/main/CAIPs/caip-2.md) (Chain Agnostic Improvement Proposal) format for network identifiers. This provides a standardized way to identify blockchain networks across the ecosystem.

### CAIP-2 Format

The format is: `namespace:reference`

-   **EVM Networks**: `eip155:{chainId}` where `chainId` is the EVM chain ID
-   **Solana Networks**: `solana:{genesisHash}` where `genesisHash` is a truncated genesis hash

### Examples

```
// EVM networks
network: "eip155:8453"      // Base mainnet (chain ID 8453)
network: "eip155:84532"     // Base Sepolia (chain ID 84532)
network: "eip155:1"         // Ethereum mainnet (chain ID 1)
// Solana networks
network: "solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp"  // Solana mainnet
network: "solana:EtWTRABZaYq6iMfeYKouRu166VU2xqa1"  // Solana devnet

```

## Reference SDK Support

The x402 reference SDKs are designed to be flexible and extensible:

### EVM Support (`@x402/evm`)

The `@x402/evm` package supports **any EVM-compatible chain** you configure, as long as the payment asset implements [EIP-3009](https://eips.ethereum.org/EIPS/eip-3009) (Transfer With Authorization). This includes:

-   Any Ethereum L1 or L2
-   Any EVM-compatible chain with a valid chain ID
-   Any EIP-3009 compliant token (USDC, EURC, etc.)

### Solana Support (`@x402/svm`)

The `@x402/svm` package supports **any Solana cluster** with:

-   SPL Token Program tokens
-   Token2022 program tokens

### Other SDKs

Community and third-party SDKs may expand support to other protocol families or chains beyond what the reference SDKs currently support. Check the [x402 ecosystem](https://www.x402.org/ecosystem) for additional implementations.

## CDP Facilitator Support

The CDP (Coinbase Developer Platform) facilitator is configured to support the following networks and schemes:

Network

CAIP-2 Identifier

v1 Support

v2 Support

Scheme

Pricing

Base

`eip155:8453`

✅

✅

`exact`

Free tier\*

Base Sepolia

`eip155:84532`

✅

✅

`exact`

Free tier\*

Solana

`solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp`

✅

✅

`exact`

Free tier\*

Solana Devnet

`solana:EtWTRABZaYq6iMfeYKouRu166VU2xqa1`

✅

✅

`exact`

Free tier\*

\*1,000 transactions free per month, then $0.001/transaction. Gas is paid on-chain separately. See [Facilitator Pricing](https://developer.chrome.com/x402/core-concepts/facilitator#pricing) for details. **CDP Facilitator Endpoints:**

-   **Mainnet**: `https://api.cdp.coinbase.com/platform/v2/x402`
    -   Requires CDP API keys
    -   Supports: Base, Solana

## x402.org Testnet Facilitator

The x402.org testnet facilitator is available for testing and development:

Network

CAIP-2 Identifier

v1 Support

v2 Support

Scheme

Base Sepolia

`eip155:84532`

✅

✅

`exact`

Solana Devnet

`solana:EtWTRABZaYq6iMfeYKouRu166VU2xqa1`

✅

✅

`exact`

**Endpoint**: `https://www.x402.org/facilitator`

-   No API key required
-   For testnet use only

## Other Facilitators

The x402 protocol is permissionless, so anyone can run a facilitator. The ecosystem includes several community and third-party facilitators that support additional networks, tokens, and features.

## Chain ID Reference

Quick reference for common EVM chain IDs:

Chain ID

Network

CAIP-2 Format

1

Ethereum Mainnet

`eip155:1`

10

Optimism

`eip155:10`

137

Polygon

`eip155:137`

8453

Base

`eip155:8453`

84532

Base Sepolia

`eip155:84532`

42161

Arbitrum One

`eip155:42161`

43114

Avalanche C-Chain

`eip155:43114`

43113

Avalanche Fuji

`eip155:43113`

## Token Support

### EVM Networks (EIP-3009)

On EVM networks, x402 supports tokens that implement [EIP-3009](https://eips.ethereum.org/EIPS/eip-3009) (Transfer With Authorization). This enables gasless transfers where the user signs a message and the facilitator submits the transaction. **Common EIP-3009 Tokens:**

Token

Base

Base Sepolia

USDC

`0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913`

`0x036CbD53842c5426634e7929541eC2318f3dCF7e`

### Solana Networks (SPL / Token2022)

On Solana, x402 supports SPL Token Program tokens for v1 and v2, and Token2022 program tokens for v2 only. **Common Tokens:**

Token

Mainnet

Devnet

USDC

`EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v`

(faucet available)

## Price Formatting

x402 supports multiple price formats for convenience:

```
// Dollar string format (recommended for readability)
price: "$0.001"    // 0.001 USDC
price: "$1.00"     // 1 USDC
price: "$0.50"     // 0.50 USDC
// Atomic units (for precision)
amount: "1000"     // 0.001 USDC (6 decimals)
amount: "1000000"  // 1 USDC (6 decimals)

```

## Usage Examples

### Server-Side Configuration

-   Node.js
    
-   Go
    

```
import { paymentMiddleware, x402ResourceServer } from "@x402/express";
import { ExactEvmScheme } from "@x402/evm/exact/server";
import { HTTPFacilitatorClient } from "@x402/core/server";
const facilitatorClient = new HTTPFacilitatorClient({
  url: "https://www.x402.org/facilitator",
});
// Register scheme for Base Sepolia
const server = new x402ResourceServer(facilitatorClient)
  .register("eip155:84532", new ExactEvmScheme());
app.use(
  paymentMiddleware(
    {
      "GET /api": {
        accepts: [
          {
            scheme: "exact",
            price: "$0.001",
            network: "eip155:84532", // Base Sepolia
            payTo: "0xYourAddress",
          },
        ],
        description: "API endpoint",
      },
    },
    server,
  ),
);

```

```
import (
    x402 "github.com/coinbase/x402/go"
    x402http "github.com/coinbase/x402/go/http"
    evm "github.com/coinbase/x402/go/mechanisms/evm/exact/server"
)
network := x402.Network("eip155:84532") // Base Sepolia
r.Use(ginmw.X402Payment(ginmw.Config{
    Routes: x402http.RoutesConfig{
        "GET /api": {
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

### Using a Custom Network

Since `@x402/evm` supports any EVM chain, you can configure networks not listed above:

```
// Example: Using Ethereum mainnet with a custom facilitator
const facilitatorClient = new HTTPFacilitatorClient({
  url: "https://example.facilitator.com",
});
const server = new x402ResourceServer(facilitatorClient)
  .register("eip155:1", new ExactEvmScheme()); // Ethereum mainnet
app.use(
  paymentMiddleware(
    {
      "GET /api": {
        accepts: [
          {
            scheme: "exact",
            price: "$0.001",
            network: "eip155:1",
            payTo: "0xYourAddress",
            asset: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", // USDC on Ethereum
          },
        ],
        description: "API endpoint",
      },
    },
    server,
  ),
);

```

### Multi-Network Support

Support multiple networks on the same endpoint:

```
{
  "GET /api": {
    accepts: [
      {
        scheme: "exact",
        price: "$0.001",
        network: "eip155:8453",  // Base mainnet
        payTo: "0xYourEvmAddress",
      },
      {
        scheme: "exact",
        price: "$0.001",
        network: "solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp",  // Solana mainnet
        payTo: "YourSolanaAddress",
      },
    ],
    description: "Multi-network endpoint",
  },
}

```

## Running Your Own Facilitator

If you need to support networks or tokens not available through existing facilitators, you can run your own:

1.  **Use the reference implementation**: The [x402 GitHub repository](https://github.com/coinbase/x402) contains facilitator code
2.  **Use a community implementation**: Check the [x402 Ecosystem](https://www.x402.org/ecosystem?category=facilitators) for self-hostable options

## Next Steps

-   [Quickstart for Sellers](https://developer.chrome.com/x402/quickstart-for-sellers) - Start accepting payments
-   [Quickstart for Buyers](https://developer.chrome.com/x402/quickstart-for-buyers) - Start making payments
-   [Core Concepts: Facilitator](https://developer.chrome.com/x402/core-concepts/facilitator) - Learn about facilitators
-   [x402 Ecosystem](https://www.x402.org/ecosystem) - Explore facilitators and tools