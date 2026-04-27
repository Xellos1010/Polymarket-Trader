# gelato integration

Learn how to integrate Coinbase Embedded Wallets as a wallet provider with the Gelato Bundler for sponsored user operations.

## Prerequisites

-   A free [CDP Portal](https://portal.cdp.coinbase.com/) account and project
-   [Node.js 22+](https://nodejs.org/en/download)
-   A node package manager installed (i.e., `npm`, `pnpm`, or `yarn`)
-   Basic familiarity with React and TypeScript
-   Configured your domain in CDP Portal (see below)

How to configure your domain in CDP Portal

**Step 1: Access CDP Portal**Navigate to the [Domains Configuration](https://portal.cdp.coinbase.com/products/embedded-wallets/domains) in CDP Portal, and click **Add domain** to include your local app.

**Step 2: Add your domain**

-   For local development: Use `http://localhost:3000` (or your preferred port)
-   For production: Use your actual domain (e.g., `https://yourapp.com`)

**Step 3: Save your changes**Click **Add domain** again to save your changes.

You should see your domain listed in the CDP Portal dashboard. The allowlist will take effect immediately upon saving.

-   A [Gelato](https://app.gelato.cloud/) account with an app and API key generated from `Paymaster & Bundler` > `API Keys`

Install the required CDP packages:

```
# npm
npm install @coinbase/cdp-react @coinbase/cdp-hooks @coinbase/cdp-core viem

```