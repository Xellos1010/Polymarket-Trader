# onchain commerce shop

PaymentsFintechOnchainKit

## Onchain Commerce Template

An Onchain Commerce Template build with [OnchainKit](https://onchainkit.xyz/), and ready to be deployed to Vercel. Play with it live on [https://onchain-commerce-template.vercel.app/](https://onchain-commerce-template.vercel.app/) Have fun! ⛵️

### Setup

To ensure all components work seamlessly, set the following environment variables in your `.env` file using `.local.env.example` as a reference. You can find the API key on the [Coinbase Developer Portal’s OnchainKit page](https://portal.cdp.coinbase.com/products/onchainkit). If you don’t have an account, you will need to create one. You can find your Coinbase Commerce API key on [Coinbase Commerce](https://beta.commerce.coinbase.com/). If you don’t have an account, you will need to create one.

```
# See https://portal.cdp.coinbase.com/products/onchainkit
NEXT_PUBLIC_ONCHAINKIT_API_KEY="GET_FROM_COINBASE_DEVELOPER_PLATFORM"
# See https://beta.commerce.coinbase.com/
NEXT_PUBLIC_COINBASE_COMMERCE_API_KEY="GET_FROM_COINBASE_COMMERCE"

```

### Enabling checkout

By default, the checkout functionality is disabled to prevent transactions in non-production environments. To enable the checkout flow for local development, you need to uncomment certain lines of code in the `OnchainStoreCart.tsx` component, along with the imports at the top of the file. You can also remove the `OnchainStoreModal` component and logic as well as the `MockCheckoutButton` as these were created for demo purposes only. Next, you’ll want to replace `products` in the `OnchainStoreProvider` with your own product items. After these changes, the actual OnchainKit checkout flow will be functional in your local environment.

### Running locally

```
# Install bun in case you don't have it
bun curl -fsSL <https://bun.sh/install> | bash
# Install packages
bun i
# Run Next app
bun run dev

```

### Resources

-   [OnchainKit documentation](https://onchainkit.xyz/)

### License

This project is licensed under the MIT License - see the [LICENSE.md](https://github.com/coinbase/onchain-commerce-template/blob/master/LICENSE) file for details