# quickstart

**Base URL:** `https://sandbox.cdp.coinbase.com`

## Prerequisites

Before you begin, make sure you have:

-   A CDP account with access to the [CDP Portal](https://portal.cdp.coinbase.com/)
-   A terminal with [cdpcurl](https://github.com/coinbase/cdpcurl) installed (or Postman)

## 1\. Create Sandbox API keys

## 2\. Install `cdpcurl`

[cdpcurl](https://github.com/coinbase/cdpcurl) is a command-line tool that handles JWT authentication automatically—just point it at your downloaded API key file and it takes care of signing requests for you. **Install via Homebrew:**

```
brew tap coinbase/cdpcurl    # Add the CDP tap to Homebrew
brew install cdpcurl          # Install the tool

```

**Or install via Go:**

```
go install github.com/coinbase/cdpcurl@latest

```

**Set your API key path:**

```
export CDP_API_KEY=~/Downloads/cdp_api_key.json

```

This lets you use `$CDP_API_KEY` in all commands instead of typing the full path each time.

## 3\. Create and fund an account

Create a Sandbox account and add test balances through the Portal UI:

## 4\. Verify balance(s)

Run the following to verify your account balance:

```
cdpcurl -k $CDP_API_KEY \
  'https://sandbox.cdp.coinbase.com/platform/v2/accounts/YOUR_ACCOUNT_ID' | sed '1d' | jq

```

## 5\. Alternative: Test with Postman

Prefer a GUI? See the [Postman guide](https://developer.chrome.com/api-reference/payment-apis/sandbox/postman) for setup instructions.

## Next steps

Now that you’re set up, explore the resource guides to test specific features:

## Transitioning to Production

When you’re ready to move from Sandbox to Production:

-   **Complete integration testing:** Ensure all features work correctly in Sandbox
-   **Create Production API keys:** Generate Production credentials in the CDP Portal
-   **Update configuration:** Switch from `sandbox.cdp.coinbase.com` to `api.cdp.coinbase.com`
-   **Start with small transactions:** Begin with small test transactions to verify everything works
-   **Set up monitoring:** Configure alerting for failed transactions and API errors