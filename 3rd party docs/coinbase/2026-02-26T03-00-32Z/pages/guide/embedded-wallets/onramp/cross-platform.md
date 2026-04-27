# cross platform

EVMSolana

## Overview

Embedded Wallets provides components that work with [Coinbase’s Cross-Platform Onramp API](https://developer.chrome.com/onramp/headless-onramp/overview) to enable developers to move money from fiat to onchain economies. A user can fund their wallet with their Coinbase account or through guest checkout with a debit card. This guide shows how to get started with the `FundModal` component.

## Quickstart

Get started in under 5 minutes with Embedded Wallet’s [`create-cdp-app`](https://www.npmjs.com/package/@coinbase/create-cdp-app) package!

### Prerequisites

-   A free [CDP Portal](https://portal.cdp.coinbase.com/) account and project
-   [Node.js 22+](https://nodejs.org/en/download)
-   A node package manager installed (i.e., `npm`, `pnpm`, or `yarn`)
-   Basic familiarity with Next.js and React
-   A Coinbase Retail account, if you wish to fund your wallet with Coinbase

### 1\. Add your domain

To begin, add your domain to the list of [allowed domains](https://portal.cdp.coinbase.com/products/embedded-wallets/domains) in CDP Portal.

### 2\. Create the demo app

### 3\. Demo your new wallet

Now that your embedded wallet is configured and your app is running, let’s try it out.

## Manual setup

If you’d prefer to set integrate Onramp manually, this guide will show you how to do so.

### Prerequisites

-   A free [CDP Portal](https://portal.cdp.coinbase.com/) account and project
-   [Node.js 22+](https://nodejs.org/en/download)
-   A node package manager installed (i.e., `npm`, `pnpm`, or `yarn`)
-   Basic familiarity with Next.js and React
-   A CDP project with Embedded Wallets enabled
-   `@coinbase/cdp-core` and `@coinbase/cdp-hooks` installed
-   A Coinbase Retail account, if you wish to fund your wallet with Coinbase

### 1\. Create a Secret API Key

### 2\. Install `@coinbase/cdp-sdk`

The Onramp API requires authentication with a JWT. You can use [`@coinbase/cdp-sdk`](https://www.npmjs.com/package/@coinbase/cdp-sdk) to generate one.

### 3\. Create `lib/cdp-auth.ts`

Create a new file `lib/cdp-auth.ts` in your project root. This file exports helper functions to generate JWTs for authorizing Onramp API calls and provides the base URL for API requests.

```
import { generateJwt } from "@coinbase/cdp-sdk/auth";
interface CDPAuthConfig {
  requestMethod: string;
  requestHost: string;
  requestPath: string;
  audience?: string[];
}
/**
 * Get CDP API credentials from environment variables
 *
 * @throws Error if credentials are not configured
 */
export function getCDPCredentials() {
  const apiKeyId = process.env.CDP_API_KEY_ID;
  const apiKeySecret = process.env.CDP_API_KEY_SECRET;
  if (!apiKeyId || !apiKeySecret) {
    throw new Error("CDP API credentials not configured");
  }
  return { apiKeyId, apiKeySecret };
}
/**
 * Generate JWT token for CDP API authentication
 *
 * @param config - Configuration for JWT generation
 * @returns JWT token string
 */
export async function generateCDPJWT(config: CDPAuthConfig): Promise<string> {
  const { apiKeyId, apiKeySecret } = getCDPCredentials();
  return generateJwt({
    apiKeyId,
    apiKeySecret,
    requestMethod: config.requestMethod,
    requestHost: config.requestHost,
    requestPath: config.requestPath,
  });
}
/**
 * Base URL for ONRAMP API
 * Can change to api.cdp.coinbase.com/platform once session token endpoints are supported in v2 API
 */
export const ONRAMP_API_BASE_URL = "https://api.developer.coinbase.com";

```

This utility file provides:

-   `getCDPCredentials()`: Reads your API credentials from environment variables
-   `generateCDPJWT()`: Creates authenticated JWT tokens for API calls
-   `ONRAMP_API_BASE_URL`: The base URL for all Onramp API requests

These functions will be imported and used in your API routes in the next step.

### 4\. Set up server-side endpoints

You will need to create two server-side endpoints to interact with the Onramp API.

### 5\. `FundModal` component

Finally, you are ready to add the `FundModal` component to your app.

Funding a Solana wallet with FundModal

You may fund your Solana embedded wallets using the same FundModal as in the EVM example above. Just pass in the appropriate values for the `cryptoCurrency`, `network`, and `destinationAddress` props.

components/FundSolanaWallet.tsx

```
"use client";
import {
  FundModal,
  type FundModalProps,
} from "@coinbase/cdp-react";
import { useSolanaAddress } from "@coinbase/cdp-hooks";
import { useCallback } from "react";
import { getBuyOptions, createBuyQuote } from "@/lib/onramp-api";
/**
 * A component that wraps the FundModal component
 *
 * @param props - The props for the FundWallet component
 * @param props.onSuccess - The callback function to call when the onramp purchase is successful
 * @returns The FundWallet component
 */
export default function FundWallet({ onSuccess }: { onSuccess: () => void }) {
  const { solanaAddress } = useSolanaAddress();
  // Get the user's location (i.e. from IP geolocation)
  const userCountry = "US";
  // If user is in the US, the state is also required
  const userSubdivision = userCountry === "US" ? "CA" : undefined;
  // Call your buy quote endpoint
  const fetchBuyQuote: FundModalProps["fetchBuyQuote"] = useCallback(async params => {
    return createBuyQuote(params);
  }, []);
  // Call your buy options endpoint
  const fetchBuyOptions: FundModalProps["fetchBuyOptions"] = useCallback(async params => {
    return getBuyOptions(params);
  }, []);
  return (
    <FundModal
      country={userCountry}
      subdivision={userSubdivision}
      cryptoCurrency="sol"
      fiatCurrency="usd"
      fetchBuyQuote={fetchBuyQuote}
      fetchBuyOptions={fetchBuyOptions}
      network="solana"
      presetAmountInputs={[10, 25, 50]}
      onSuccess={onSuccess}
      destinationAddress={solanaAddress}
    />
  );
}

```

## Reference

Resource

Description

[Buy options API](https://developer.chrome.com/api-reference/rest-api/onramp-offramp/get-buy-options)

Coinbase Onramp Buy Options API reference

[Buy quote API](https://developer.chrome.com/api-reference/rest-api/onramp-offramp/create-buy-quote)

Coinbase Onramp Buy Quote API reference

[Fund README](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/Fund.README)

Component overview and usage

## What to read next

-   **[React Components](https://developer.chrome.com/embedded-wallets/react-components)**: Explore all available Embedded Wallet React components, including authentication, wallet management, and transaction components to build complete wallet experiences
-   **[Onramp Overview](https://developer.chrome.com/onramp/headless-onramp/overview)**: Learn about the complete Onramp API ecosystem, including advanced features like offramp, webhooks, and transaction monitoring for comprehensive fiat-to-crypto solutions