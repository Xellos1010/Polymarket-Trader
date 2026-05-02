# quickstart for buyers

This guide walks you through how to use **x402** to interact with services that require payment. By the end of this guide, you will be able to programmatically discover payment requirements, complete a payment, and access a paid resource. The x402 helper packages for various languages greatly simplify your integration with x402. You’ll be able to automatically detect payment challenges, authorize payments onchain, and retry requests with minimal code. The packages will automatically trigger the following flow:

1.  Makes the initial request (if using Fetch) or intercepts the initial request (if using Axios)
2.  If a 402 response is received, parses the payment requirements from the `PAYMENT-REQUIRED` header
3.  Creates a payment payload using the configured x402Client and registered schemes
4.  Retries the request with the `PAYMENT-SIGNATURE` header

## Prerequisites

Before you begin, ensure you have:

-   A crypto wallet with USDC (any EVM-compatible wallet, e.g., [CDP Wallet](https://developer.chrome.com/server-wallets/v2/introduction/quickstart), [AgentKit](https://developer.chrome.com/agent-kit/welcome))
-   [Node.js](https://nodejs.org/en) and npm, [Go](https://go.dev/), or Python and pip installed
-   A service that requires payment via x402

## 1\. Install Dependencies

-   Node.js
    
-   Go
    
-   Python
    

Install the x402 client packages:

```
# For fetch-based clients
npm install @x402/fetch @x402/evm
# For axios-based clients
npm install @x402/axios @x402/evm
# For Solana support, also add:
npm install @x402/svm
# For Bazaar discovery (optional):
npm install @x402/core @x402/extensions

```

Add the x402 Go module to your project:

```
go get github.com/coinbase/x402/go

```

Install the [x402 package](https://pypi.org/project/x402/) with your preferred HTTP client:

```
# For httpx (async) - recommended
pip install "x402[httpx]"
# For requests (sync)
pip install "x402[requests]"
# For Solana support, also add:
pip install "x402[svm]"

```

## 2\. Create a Wallet Client

Create a wallet client using CDP’s [Server Wallet](https://developer.chrome.com/server-wallets/v2/introduction/quickstart) (recommended) or a standalone wallet library ([viem](https://viem.sh/) for EVM on Node.js, or Go’s crypto libraries).

### CDP Server Wallet (Recommended)

First, create an account at [cdp.coinbase.com](https://cdp.coinbase.com/) and get the following API keys from the portal to store as environment variables:

```
# store in .env or using the command `export <name>="secret-info"`
CDP_API_KEY_ID=your-api-key-id
CDP_API_KEY_SECRET=your-api-key-secret
CDP_WALLET_SECRET=your-wallet-secret

```

Then, install the required packages:

```
npm install @coinbase/cdp-sdk dotenv

```

Finally, instantiate the CDP client as suggested by the [Server Wallet Quickstart](https://developer.chrome.com/server-wallets/v2/introduction/quickstart):

```
import { CdpClient } from "@coinbase/cdp-sdk";
import { toAccount } from "viem/accounts";
import dotenv from "dotenv";
dotenv.config();
const cdp = new CdpClient();
const cdpAccount = await cdp.evm.createAccount();
const signer = toAccount(cdpAccount);

```

### Standalone Wallet Libraries

If you prefer to use your own wallet, you can use standalone libraries:

#### EVM (Node.js with viem)

```
import { privateKeyToAccount } from "viem/accounts";
// Create a signer from private key (use environment variable)
const signer = privateKeyToAccount(
  process.env.EVM_PRIVATE_KEY as `0x${string}`,
);

```

#### EVM (Go)

```
import (
    "crypto/ecdsa"
    "github.com/ethereum/go-ethereum/crypto"
)
// Load private key from environment
privateKey, _ := crypto.HexToECDSA(os.Getenv("EVM_PRIVATE_KEY"))

```

#### EVM (Python)

Install the required package:

Then instantiate the wallet signer:

```
import os
from eth_account import Account
from x402.mechanisms.evm import EthAccountSigner
account = Account.from_key(os.getenv("EVM_PRIVATE_KEY"))
signer = EthAccountSigner(account)

```

#### Solana (SVM)

Use [SolanaKit](https://www.solanakit.com/) to instantiate a signer:

```
import { createKeyPairSignerFromBytes } from "@solana/kit";
import { base58 } from "@scure/base";
// 64-byte base58 secret key (private + public)
const svmSigner = await createKeyPairSignerFromBytes(
  base58.decode(process.env.SOLANA_PRIVATE_KEY!),
);

```

## 3\. Make Paid Requests Automatically

You can automatically handle 402 Payment Required responses and complete payment flows using the x402 helper packages.

-   Node.js
    
-   Go
    
-   Python (httpx)
    
-   Python (requests)
    

You can use either `@x402/fetch` or `@x402/axios`:

-   @x402/fetch
    
-   @x402/axios
    

**@x402/fetch** extends the native `fetch` API to handle 402 responses and payment headers for you. [Full example here](https://github.com/coinbase/x402/tree/main/examples/typescript/clients/fetch)

```
import { x402Client, wrapFetchWithPayment, x402HTTPClient } from "@x402/fetch";
import { registerExactEvmScheme } from "@x402/evm/exact/client";
import { privateKeyToAccount } from "viem/accounts";
// Create signer
const signer = privateKeyToAccount(process.env.EVM_PRIVATE_KEY as `0x${string}`);
// Create x402 client and register schemes
const client = new x402Client();
registerExactEvmScheme(client, { signer });
// Wrap fetch with payment handling
const fetchWithPayment = wrapFetchWithPayment(fetch, client);
// Make request - payment is handled automatically
const response = await fetchWithPayment("https://api.example.com/paid-endpoint", {
  method: "GET",
});
const body = await response.json();
console.log("Response:", body);
// Get payment receipt from response headers
if (response.ok) {
  const httpClient = new x402HTTPClient(client);
  const paymentResponse = httpClient.getPaymentSettleResponse(
    (name) => response.headers.get(name)
  );
  console.log("Payment settled:", paymentResponse);
}

```

**Features:**

-   Automatically handles 402 Payment Required responses
-   Verifies payment and generates `PAYMENT-SIGNATURE` headers
-   Retries the request with proof of payment
-   Supports all standard fetch options

**@x402/axios** adds a payment interceptor to Axios, so your requests are retried with payment headers automatically. [Full example here](https://github.com/coinbase/x402/tree/main/examples/typescript/clients/axios)

```
import { x402Client, withPaymentInterceptor, x402HTTPClient } from "@x402/axios";
import { registerExactEvmScheme } from "@x402/evm/exact/client";
import { privateKeyToAccount } from "viem/accounts";
import axios from "axios";
// Create signer
const signer = privateKeyToAccount(process.env.EVM_PRIVATE_KEY as `0x${string}`);
// Create x402 client and register schemes
const client = new x402Client();
registerExactEvmScheme(client, { signer });
// Create an Axios instance with payment handling
const api = withPaymentInterceptor(
  axios.create({ baseURL: "https://api.example.com" }),
  client,
);
// Make request - payment is handled automatically
const response = await api.get("/paid-endpoint");
console.log("Response:", response.data);
// Get payment receipt
const httpClient = new x402HTTPClient(client);
const paymentResponse = httpClient.getPaymentSettleResponse(
  (name) => response.headers[name.toLowerCase()]
);
console.log("Payment settled:", paymentResponse);

```

**Features:**

-   Automatically handles 402 Payment Required responses
-   Retries requests with payment headers
-   Exposes payment response in headers

[Full example here](https://github.com/coinbase/x402/tree/main/examples/go/clients/http)

```
package main
import (
    "context"
    "encoding/json"
    "fmt"
    "net/http"
    "os"
    "time"
    x402 "github.com/coinbase/x402/go"
    evm "github.com/coinbase/x402/go/mechanisms/evm/exact/client"
)
func main() {
    privateKey := os.Getenv("EVM_PRIVATE_KEY")
    url := "http://localhost:4021/weather"
    // Create x402 client and register EVM scheme
    client := x402.NewX402Client()
    evm.RegisterExactEvmScheme(client, &evm.Config{
        PrivateKey: privateKey,
    })
    // Wrap HTTP client with payment handling
    httpClient := x402.WrapHTTPClient(client)
    // Make request - payment is handled automatically
    ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
    defer cancel()
    req, _ := http.NewRequestWithContext(ctx, "GET", url, nil)
    resp, err := httpClient.Do(req)
    if err != nil {
        fmt.Printf("Request failed: %v\n", err)
        return
    }
    defer resp.Body.Close()
    // Read response
    var data map[string]interface{}
    json.NewDecoder(resp.Body).Decode(&data)
    fmt.Printf("Response: %+v\n", data)
    // Check payment response header
    paymentHeader := resp.Header.Get("PAYMENT-RESPONSE")
    if paymentHeader != "" {
        fmt.Println("Payment settled successfully!")
    }
}

```

**httpx** provides async HTTP client support with automatic 402 payment handling.[Full HTTPX example](https://github.com/coinbase/x402/tree/main/examples/python/clients/httpx) | [Full Requests example](https://github.com/coinbase/x402/tree/main/examples/python/clients/requests)

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

**requests** provides sync HTTP client support with automatic 402 payment handling.[Full Requests example](https://github.com/coinbase/x402/tree/main/examples/python/clients/requests)

```
import os
from eth_account import Account
from x402 import x402ClientSync
from x402.http import x402HTTPClientSync
from x402.http.clients import x402_requests
from x402.mechanisms.evm import EthAccountSigner
from x402.mechanisms.evm.exact.register import register_exact_evm_client
def main() -> None:
    client = x402ClientSync()
    account = Account.from_key(os.getenv("EVM_PRIVATE_KEY"))
    register_exact_evm_client(client, EthAccountSigner(account))
    http_client = x402HTTPClientSync(client)
    with x402_requests(client) as session:
        response = session.get("https://api.example.com/paid-endpoint")
        print(f"Response: {response.text}")
        if response.ok:
            settle_response = http_client.get_payment_settle_response(
                lambda name: response.headers.get(name)
            )
            print(f"Payment settled: {settle_response}")
main()

```

### Multi-Network Client Setup

You can register multiple payment schemes to handle different networks:

-   TypeScript
    
-   Go
    
-   Python
    

```
import { x402Client, wrapFetchWithPayment } from "@x402/fetch";
import { registerExactEvmScheme } from "@x402/evm/exact/client";
import { registerExactSvmScheme } from "@x402/svm/exact/client";
import { privateKeyToAccount } from "viem/accounts";
import { createKeyPairSignerFromBytes } from "@solana/kit";
import { base58 } from "@scure/base";
// Create signers
const evmSigner = privateKeyToAccount(
  process.env.EVM_PRIVATE_KEY as `0x${string}`,
);
const svmSigner = await createKeyPairSignerFromBytes(
  base58.decode(process.env.SOLANA_PRIVATE_KEY!),
);
// Create client with multiple schemes
const client = new x402Client();
registerExactEvmScheme(client, { signer: evmSigner });
registerExactSvmScheme(client, { signer: svmSigner });
const fetchWithPayment = wrapFetchWithPayment(fetch, client);
// Now handles both EVM and Solana networks automatically!

```

```
import (
    x402 "github.com/coinbase/x402/go"
    x402http "github.com/coinbase/x402/go/http"
    evm "github.com/coinbase/x402/go/mechanisms/evm/exact/client"
    svm "github.com/coinbase/x402/go/mechanisms/svm/exact/client"
    evmsigners "github.com/coinbase/x402/go/signers/evm"
    svmsigners "github.com/coinbase/x402/go/signers/svm"
)
// Create signers
evmSigner, _ := evmsigners.NewClientSignerFromPrivateKey(os.Getenv("EVM_PRIVATE_KEY"))
svmSigner, _ := svmsigners.NewClientSignerFromPrivateKey(os.Getenv("SVM_PRIVATE_KEY"))
// Create client with multiple schemes
x402Client := x402.Newx402Client().
    Register("eip155:*", evm.NewExactEvmScheme(evmSigner)).
    Register("solana:*", svm.NewExactSvmScheme(svmSigner))
// Wrap HTTP client with payment handling
httpClient := x402http.WrapHTTPClientWithPayment(
    http.DefaultClient,
    x402http.Newx402HTTPClient(x402Client),
)
// Now handles both EVM and Solana networks automatically!

```

```
import asyncio
import os
from eth_account import Account
from x402 import x402Client
from x402.http.clients import x402HttpxClient
from x402.mechanisms.evm import EthAccountSigner
from x402.mechanisms.evm.exact.register import register_exact_evm_client
from x402.mechanisms.svm import KeypairSigner
from x402.mechanisms.svm.exact.register import register_exact_svm_client
async def main() -> None:
    client = x402Client()
    # Register EVM scheme
    account = Account.from_key(os.getenv("EVM_PRIVATE_KEY"))
    register_exact_evm_client(client, EthAccountSigner(account))
    # Register SVM scheme
    svm_signer = KeypairSigner.from_base58(os.getenv("SVM_PRIVATE_KEY"))
    register_exact_svm_client(client, svm_signer)
    async with x402HttpxClient(client) as http:
        response = await http.get("https://api.example.com/paid-endpoint")
        print(f"Response: {response.text}")
asyncio.run(main())

```

## 4\. Discover Available Services (Optional)

Instead of hardcoding endpoints, you can use the x402 Bazaar to dynamically discover available services. This is especially powerful for building autonomous agents that can find and use new capabilities.

-   Node.js
    
-   Go
    
-   Python
    

```
import { HTTPFacilitatorClient } from "@x402/core/http";
import { withBazaar } from "@x402/extensions/bazaar";
// Create facilitator client with Bazaar extension
const facilitatorClient = withBazaar(
  new HTTPFacilitatorClient({
    url: "https://api.cdp.coinbase.com/platform/v2/x402"
  })
);
// Query available services
const discovery = await facilitatorClient.extensions.discovery.listResources({
  type: "http",
  limit: 20,
});
// Filter services by criteria
const affordableServices = discovery.items.filter((item) =>
  item.accepts.some((req) => Number(req.amount) < 100000) // Under $0.10
);
console.log("Available services:", affordableServices);

```

```
import (
    "encoding/json"
    "net/http"
)
// Fetch available services
resp, _ := http.Get("https://api.cdp.coinbase.com/platform/v2/x402/discovery/resources")
defer resp.Body.Close()
var services struct {
    Items []map[string]interface{} `json:"items"`
}
json.NewDecoder(resp.Body).Decode(&services)
fmt.Printf("Found %d services\n", len(services.Items))

```

```
import asyncio
import httpx
async def main() -> None:
    async with httpx.AsyncClient() as client:
        response = await client.get(
            "https://api.cdp.coinbase.com/platform/v2/x402/discovery/resources",
            params={"type": "http", "limit": 20},
        )
        services = response.json()
        # Filter services by criteria
        affordable_services = [
            item for item in services.get("items", [])
            if any(int(req.get("amount", 0)) < 100000 for req in item.get("accepts", []))
        ]
        print(f"Available services: {affordable_services}")
asyncio.run(main())

```

## 5\. Error Handling

Clients will throw errors if:

-   No scheme is registered for the required network
-   The request configuration is missing
-   A payment has already been attempted for the request
-   There is an error creating the payment header

Common error handling:

```
try {
  const response = await fetchWithPayment(url, { method: "GET" });
  // Handle success
} catch (error) {
  if (error.message.includes("No scheme registered")) {
    console.error("Network not supported - register the appropriate scheme");
  } else if (error.message.includes("Payment already attempted")) {
    console.error("Payment failed on retry");
  } else {
    console.error("Request failed:", error);
  }
}

```

## Summary

-   Install x402 client packages (`@x402/fetch` or `@x402/axios`) and mechanism packages (`@x402/evm`, `@x402/svm`)
-   Create a wallet signer
-   Create an `x402Client` and register payment schemes
-   Use the provided wrapper/interceptor to make paid API requests
-   (Optional) Use the x402 Bazaar to discover services dynamically
-   Payment flows are handled automatically for you

## References:

-   [@x402/fetch on npm](https://www.npmjs.com/package/@x402/fetch)
-   [@x402/axios on npm](https://www.npmjs.com/package/@x402/axios)
-   [@x402/evm on npm](https://www.npmjs.com/package/@x402/evm)
-   [x402 Go module](https://github.com/coinbase/x402/tree/main/go)
-   [x402 Python package on PyPI](https://pypi.org/project/x402/)
-   [x402 Bazaar documentation](https://developer.chrome.com/x402/bazaar) - Discover available services
-   [X402 with Embedded Wallets](https://developer.chrome.com/embedded-wallets/x402-payments) - User-facing applications with embedded wallets

For questions or support, join our [Discord](https://discord.gg/cdp).