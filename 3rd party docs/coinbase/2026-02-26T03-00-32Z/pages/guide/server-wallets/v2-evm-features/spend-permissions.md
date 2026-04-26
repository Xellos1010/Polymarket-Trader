# spend permissions

## Overview

Spend Permissions let you designate a trusted spender that can spend tokens on behalf of your [Smart Account](https://developer.chrome.com/server-wallets/v2/evm-features/smart-accounts). After you sign the permission, the spender can initiate token spending within the limits you define. You can define limits based on token, time period and amount.

Spend Permissions utilize the [Spend Permission Manager contract](https://github.com/coinbase/spend-permissions) deployed on Base and [other networks](https://developer.chrome.com/server-wallets/v2/evm-features/spend-permissions#supported-networks). Some use cases this feature enables:

-   **Subscription payments** - Enable recurring payments for SaaS, content subscriptions, or membership fees
-   **Agentic payments** - Control your agent’s spending limits for autonomous operations
-   **Algorithmic trading** - Allow trading bots to execute trades within predefined limits
-   **Automated payouts** - Schedule regular distributions or reward payments
-   **Allowance management** - Give team members or family controlled access to funds
-   **Dollar-cost averaging** - Automate periodic investment purchases

## How Spend Permissions Work

There are two parties involved in a Spend Permission:

-   **Account** - The smart account that creates the Spend Permission and approves it onchain.
-   **Spender** - The entity that can spend tokens on behalf of the account within the limits defined by a Spend Permission. Can be a Smart Account or a regular account.

The [CDP SDK](https://github.com/coinbase/cdp-sdk) makes it easy to work with Spend Permissions as either the Account or Spender, offering methods to create and manage Spend Permissions, as well as methods to use Spend Permissions.

## Anatomy of a Spend Permission

These are the main components of a Spend Permission:

-   **Spender** - The entity that can spend tokens on behalf of the account.
-   **Token** - The token that the Spend Permission is for, and the amount of that token that the Spender is allowed to spend.
-   **Time Period** - The time period for which the Spend Permission is valid.
-   **Salt** - A random value used to differentiate between Spend Permissions with the same parameters. The SDK will generate a random salt for you, but you can also specify your own.
-   **Extra Data** - Arbitrary data that can be used to store additional information about the Spend Permission.

See the following sections for more details on the main components.

### Spender

The spender is specified in the `spender` field of the Spend Permission. It can be the address of any account, whether it’s a Smart Account or a regular account.

### Token

The token is specified in the `token` field of the Spend Permission, and the amount allowed to spend is specified in the `allowance` field. Spend Permissions support both native tokens and ERC-20 tokens. When using the CDP SDK, you have two options for specifying tokens:

1.  **Convenient shortcuts** - Use `"eth"` for native ETH or `"usdc"` for USDC, and the SDK will handle the conversion to the correct token address. This shortcut is only supported on Base or Base Sepolia.
2.  **ERC-20 contract addresses** - For other tokens, specify the token contract address as a string (e.g., `"0x4200000000000000000000000000000000000006"` for WETH).

The amount allowed to spend is specified in the `allowance` field, using the smallest unit of the token. For example, if the token is ETH, the allowance is specified in wei, and if the token is USDC, the allowance is specified in the smallest unit of USDC (6 decimals).

### Time Period

The time period is specified using the `periodInDays` field for simple day-based periods, or the `period`, `start` and `end` fields for more complex time controls. The `periodInDays` field provides a convenient way to specify common time periods (e.g., `periodInDays: 1` for daily limits, `periodInDays: 7` for weekly limits). For more advanced control, the `start` and `end` fields specify when the Spend Permission is valid. This means that the Spender can spend the amount specified in the `allowance` field after the `start` time and before the `end` time; attempting to spend outside of this time range will fail.

## Creating a Spend Permission

Here’s how to create a spend permission that allows a spender to withdraw up to 0.01 USDC per day:

### ERC-20 Token Permissions

To create spend permissions for ERC-20 tokens other than USDC, specify the token contract address:

```
const wethSpendPermission: SpendPermissionInput = {
  account: smartAccount.address,
  spender: spender.address,
  token: "0x4200000000000000000000000000000000000006", // WETH on Base Sepolia
  allowance: parseEther("0.00001"), // 0.00001 WETH
  periodInDays: 1, // Daily limit
};

```

## Using a Spend Permission

Once a spend permission is created, the designated spender can spend tokens on behalf of the account within the defined limits:

## Check Remaining Allowance

To check how much of a spend permission’s allowance remains in the current period, query the `getCurrentPeriod` function on the Spend Permission Manager contract. This returns the period start/end timestamps and the amount already spent.

Example: Calculate remaining spend allowance

## Managing Spend Permissions

### Listing Spend Permissions

The `listSpendPermissions` method works differently depending on whether you’re querying as the Account or as the Spender:

#### As the Account

When listing as the Account, you see all spend permissions you’ve granted to various spenders:

#### As the Spender

When listing as the Spender, you must query spend permissions granted by the account whose tokens you want to spend. You can then filter the results to find the spend permission you want to use.

### Revoking Spend Permissions

You can revoke a Spend Permission using the `revokeSpendPermission` method:

## Spend Permissions vs. Policies

At its core, a Spend Permission enables a spender to spend tokens on behalf of an account within a specified time period and amount. The tokens can be native ETH or an ERC-20 token. This section explores the differences between Spend Permissions and [CDP Policies](https://developer.chrome.com/server-wallets/v2/using-the-wallet-api/policies/overview).

Spend Permissions

Policies

**Evaluation Environment**

Evaluated entirely onchain through smart contracts, providing transparency and decentralization

Evaluated within Coinbase’s trusted infrastructure using TEE technology, providing secure off-chain evaluation

**Scope and Flexibility**

Specifically designed for spending assets on EVM chains

Can be applied to any arbitrary transaction type, giving full control over allowed transactions

**Platform Support**

Only available on EVM chains

Available on both EVM chains and Solana

**Account Scope**

Can grant permissions to any account onchain, including accounts outside your CDP project

Govern accounts within your CDP project only

## Best Practices

1.  **Set Reasonable Limits** - Use the minimum allowance necessary for your use case
2.  **Define Time Boundaries** - Set appropriate start and end times to limit exposure
3.  **Monitor Usage** - Track spending activity to detect any unusual behavior
4.  **Revoke When Necessary** - Implement logic to revoke permissions when no longer needed

## Supported Networks

Spend Permissions are currently supported on: Testnets:

-   Base Sepolia
-   Ethereum Sepolia

Mainnets:

-   Base
-   Ethereum
-   Optimism
-   Arbitrum
-   Polygon
-   Avalanche

## What to read next

-   [**Smart Accounts**](https://developer.chrome.com/server-wallets/v2/evm-features/smart-accounts): Learn more about ERC-4337 Smart Accounts
-   [**Gas Sponsorship**](https://developer.chrome.com/server-wallets/v2/evm-features/gas-sponsorship): Sponsor gas fees for your users’ transactions