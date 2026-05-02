# paymaster proxy

This guide shows how to build a backend proxy for your CDP Paymaster endpoint. A proxy keeps your API key secure while allowing you to add custom validation logic for sponsored transactions.

## Why Use a Proxy?

Your CDP Paymaster endpoint URL contains your **Client API Key** (not a Secret API Key). While this isn’t as sensitive as a secret key, exposing it in client-side code allows anyone to use your Paymaster endpoint to sponsor transactions—potentially draining your gas credits or abusing your allowlisted contracts. A proxy solves this by:

1.  **Keeping your API key server-side** — The Paymaster URL never reaches the browser
2.  **Enabling custom validation** — Add rate limiting, user authentication, or business logic
3.  **Providing an audit trail** — Log all sponsorship requests for monitoring

For a deeper dive into security considerations, see the [Security](https://developer.chrome.com/paymaster/reference-troubleshooting/security) page.

## Basic Proxy Implementation

Here’s a minimal Next.js API route that proxies requests to your CDP Paymaster:

app/api/paymaster/route.ts

```
import { NextRequest, NextResponse } from "next/server";
const PAYMASTER_URL = process.env.CDP_PAYMASTER_URL!;
export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const response = await fetch(PAYMASTER_URL, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body),
    });
    const data = await response.json();
    return NextResponse.json(data, { status: response.status });
  } catch (error) {
    console.error("Paymaster proxy error:", error);
    return NextResponse.json(
      { error: "Internal server error" },
      { status: 500 }
    );
  }
}

```

Store your Paymaster URL in an environment variable (server-side only):

```
# Never prefix with NEXT_PUBLIC_ — this must stay server-side
CDP_PAYMASTER_URL=https://api.developer.coinbase.com/rpc/v1/base-sepolia/YOUR_API_KEY

```

## Using the Proxy

### With CDP Embedded Wallets

Pass your proxy URL to `useSendUserOperation`:

```
import { useSendUserOperation } from "@coinbase/cdp-hooks";
const { sendUserOperation } = useSendUserOperation();
await sendUserOperation({
  evmSmartAccount: smartAccount,
  network: "base-sepolia",
  calls: [{ to: contractAddress, value: 0n, data: callData }],
  paymasterUrl: "/api/paymaster", // Your proxy endpoint
});

```

### With Wagmi

Pass your proxy URL in the paymaster capabilities:

```
import { useWriteContracts, useCapabilities } from "wagmi/experimental";
const { data: capabilities } = useCapabilities({ account: address });
const paymasterCapabilities = useMemo(() => {
  if (capabilities?.[chainId]?.paymasterService?.supported) {
    return {
      paymasterService: {
        url: "/api/paymaster", // Your proxy endpoint
      },
    };
  }
  return {};
}, [capabilities, chainId]);
writeContracts({
  contracts: [/* your contract calls */],
  capabilities: paymasterCapabilities,
});

```

## Adding Custom Validation

The power of a proxy is adding validation logic beyond what CDP’s allowlists provide.

### Rate Limiting

Prevent abuse by limiting requests per user:

app/api/paymaster/route.ts

```
import { NextRequest, NextResponse } from "next/server";
import { Ratelimit } from "@upstash/ratelimit";
import { Redis } from "@upstash/redis";
const PAYMASTER_URL = process.env.CDP_PAYMASTER_URL!;
// Configure rate limiter (e.g., 10 requests per minute per IP)
const ratelimit = new Ratelimit({
  redis: Redis.fromEnv(),
  limiter: Ratelimit.slidingWindow(10, "1 m"),
});
export async function POST(request: NextRequest) {
  // Get client identifier (IP or authenticated user ID)
  const identifier = request.ip ?? request.headers.get("x-forwarded-for") ?? "anonymous";
  // Check rate limit
  const { success, remaining } = await ratelimit.limit(identifier);
  if (!success) {
    return NextResponse.json(
      { error: "Rate limit exceeded" },
      { status: 429 }
    );
  }
  const body = await request.json();
  const response = await fetch(PAYMASTER_URL, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });
  const data = await response.json();
  return NextResponse.json(data, {
    status: response.status,
    headers: { "X-RateLimit-Remaining": remaining.toString() },
  });
}

```

### User Authentication

Only sponsor transactions for authenticated users:

app/api/paymaster/route.ts

```
import { NextRequest, NextResponse } from "next/server";
import { getServerSession } from "next-auth";
const PAYMASTER_URL = process.env.CDP_PAYMASTER_URL!;
export async function POST(request: NextRequest) {
  // Verify user is authenticated
  const session = await getServerSession();
  if (!session?.user) {
    return NextResponse.json(
      { error: "Authentication required" },
      { status: 401 }
    );
  }
  const body = await request.json();
  // Optional: Log sponsorship request
  console.log(`Sponsoring transaction for user: ${session.user.email}`);
  const response = await fetch(PAYMASTER_URL, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });
  const data = await response.json();
  return NextResponse.json(data, { status: response.status });
}

```

### Custom Business Logic

Add application-specific rules:

app/api/paymaster/route.ts

```
import { NextRequest, NextResponse } from "next/server";
const PAYMASTER_URL = process.env.CDP_PAYMASTER_URL!;
// Example: Maximum sponsorship per user per day
const dailyLimits = new Map<string, number>();
export async function POST(request: NextRequest) {
  const session = await getServerSession();
  if (!session?.user?.id) {
    return NextResponse.json({ error: "Unauthorized" }, { status: 401 });
  }
  const userId = session.user.id;
  const todayKey = `${userId}-${new Date().toDateString()}`;
  const currentCount = dailyLimits.get(todayKey) ?? 0;
  // Limit to 5 sponsored transactions per day per user
  if (currentCount >= 5) {
    return NextResponse.json(
      { error: "Daily sponsorship limit reached" },
      { status: 403 }
    );
  }
  const body = await request.json();
  const response = await fetch(PAYMASTER_URL, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });
  if (response.ok) {
    dailyLimits.set(todayKey, currentCount + 1);
  }
  const data = await response.json();
  return NextResponse.json(data, { status: response.status });
}

```

## Express.js Example

If you’re not using Next.js, here’s an Express equivalent:

```
import express from "express";
import cors from "cors";
const app = express();
app.use(cors());
app.use(express.json());
const PAYMASTER_URL = process.env.CDP_PAYMASTER_URL!;
app.post("/api/paymaster", async (req, res) => {
  try {
    const response = await fetch(PAYMASTER_URL, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(req.body),
    });
    const data = await response.json();
    res.status(response.status).json(data);
  } catch (error) {
    console.error("Paymaster proxy error:", error);
    res.status(500).json({ error: "Internal server error" });
  }
});
app.listen(3001, () => {
  console.log("Paymaster proxy running on port 3001");
});

```

## Testing Your Proxy

### Local Development

For local development, you can temporarily use the Paymaster URL directly in your frontend to verify your smart account setup works. Once confirmed, switch to your proxy.

### Verify the Proxy Works

1.  Start your backend server
2.  Send a test request to your proxy endpoint:

```
curl -X POST http://localhost:3000/api/paymaster \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"pm_getPaymasterStubData","params":[{},{},"0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789","84532"]}'

```

3.  You should receive a JSON-RPC response (either success or an error about invalid params—both confirm the proxy is forwarding correctly)

## Production Checklist

Before deploying:

-   **Environment variable is server-side only** — No `NEXT_PUBLIC_` prefix
-   **Rate limiting configured** — Prevent abuse
-   **Authentication required** — Only sponsor for your users
-   **Error handling** — Graceful failures with appropriate status codes
-   **Logging** — Monitor sponsorship requests and failures
-   **Contract allowlist configured** — Set up in [CDP Portal](https://portal.cdp.coinbase.com/products/bundler-and-paymaster) as a secondary defense

## Next Steps

-   [Security](https://developer.chrome.com/paymaster/reference-troubleshooting/security) — Full security recommendations
-   [Errors](https://developer.chrome.com/paymaster/reference-troubleshooting/errors) — Common error codes and solutions
-   [Troubleshooting](https://developer.chrome.com/paymaster/reference-troubleshooting/troubleshooting) — Debugging failed transactions