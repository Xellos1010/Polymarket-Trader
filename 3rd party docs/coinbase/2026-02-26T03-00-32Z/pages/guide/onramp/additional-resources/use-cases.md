# use cases

Coinbase Onramps delivers a seamless fiat to crypto funding experience for users. Some of the use cases that developers can support with Coinbase Onramp are listed below.

### Add funds from Coinbase to self-custody wallet

Crypto wallets can integrate Coinbase Onramp into their mobile app, browser extension, or web app. Wallet users can then trigger Coinbase Onramp and seamlessly add funds to their self-custody wallet by either (1) purchasing crypto with a saved payment method in their Coinbase account, or (2) transferring a crypto balance from their Coinbase account.

### Add funds to a linked self-custody wallet without leaving the DeFi protocol

DeFi protocols can integrate Coinbase Onramp to let their users seamlessly fund and top-up their linked self-custody wallets. For example, users can link their self-custody wallet to a DeFi protocol; and if they don’t have enough funds, they can use Onramp to add funds to their linked self-custody wallet without leaving the protocol.

### Buy/transfer ETH from Coinbase without leaving the NFT marketplace

NFT marketplaces can integrate Coinbase Onramp to let their users seamlessly fund and top-up their linked self-custody wallets. For example, users on NFT marketplaces may not have enough ETH in their linked self-custody wallet to complete an NFT purchase. With Onramp, users can purchase or transfer ETH from their Coinbase account and complete the NFT purchase without leaving the NFT marketplace.

### Maximize support by aggregating multiple exchanges

Crypto wallets want to give users the best price to the widest selection of assets across geographies, but not all exchanges offer the same level of coverage. A common solution is to integrate with multiple onramp providers, letting users decide which exchange is best for them. Coinbase Onramp fully supports aggregators with a suite of APIs to query for coverage for different countries, payment method support, assets, and purchase limits. Just call the [Buy Quote API](https://developer.chrome.com/onramp/coinbase-hosted-onramp/generating-quotes) to generate a quote, and in the case Coinbase Onramp is the best option, create an Onramp link to a [One-Click-Buy](https://developer.chrome.com/onramp/coinbase-hosted-onramp/generating-onramp-url) experience with all inputs set to the parameters of the quote.  
**See Also:**

-   [Overview of Onramp APIs](https://developer.chrome.com/onramp/coinbase-hosted-onramp/overview)