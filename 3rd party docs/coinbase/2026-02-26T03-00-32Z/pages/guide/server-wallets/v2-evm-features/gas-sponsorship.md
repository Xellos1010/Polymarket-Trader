# gas sponsorship

## Overview

Smart Accounts unlock the ability for developers to sponsor gas on their users’ transactions. With the CDP SDK, it’s simple to integrate any paymaster URL and cover gas costs for all transactions originating from your smart accounts. You can refer to this [guide](https://developer.chrome.com/paymaster/introduction/welcome) on how to set up a CDP Paymaster. You could also use any provider of your choice.

### Benefits:

-   Removes the friction of requiring users to hold ETH
-   Enables users to interact with your app immediately
-   Provides a more familiar Web2-like experience for new users
-   Allows you to subsidize or fully cover transaction costs for your users
-   Gives you control over which transactions you want to sponsor

## Prerequisites

-   A Coinbase Developer Platform account
-   The following environment variables in your `.env` file:

```
CDP_API_KEY_ID=your_api_key_id
CDP_API_KEY_SECRET=your_api_key_secret
CDP_WALLET_SECRET=your_wallet_secret

```

All user operations on Base Sepolia are sponsored by default. For mainnet, you can specify a paymaster of your choice as follows:

You can apply for [gas credits](https://docs.google.com/forms/d/1yPnBFW0bVUNLUN_w3ctCqYM9sjdIQO3Typ53KXlsS5g/viewform?edit_requested=true&pli=1) as you scale.

## Common Issues

-   If transactions fail, verify your gas estimates
-   If paymaster fails, check your paymaster client and network configuration
-   Make sure your environment variables are properly set in your `.env` file
-   Verify you’re on the correct network before proceeding with transactions

## Additional Resources

-   [Paymaster Documentation](https://developer.chrome.com/paymaster/introduction/welcome)
-   [Account Abstraction Basics](https://developer.chrome.com/paymaster/faqs#basics)
-   [Get started with Coinbase Developer Platform](https://developer.chrome.com/get-started/quickstart)

* * *