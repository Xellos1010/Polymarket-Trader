# faq

Welcome to the Coinbase CDP SDK FAQ section. Below are some of the most common questions developers have asked, along with detailed answers from our engineering team. This resource is designed to help you navigate and utilize CDP SDK effectively in your projects.

## General Questions

### What networks does CDP SDK Support? Can we add new ones?

Supported networks and capabilities are listed here. We do not have functionality for users to add new ones.

### How can I directly use the API?

See our [Server Wallet reference](https://developer.chrome.com/api-reference/introduction) for endpoints to use. Note: If using Server Wallet’s 2-of-2 MPC configuration, using APIs directly is not feasible.

### What assets are supported natively with the SDK?

With [natively-supported assets](https://developer.chrome.com/server-wallets/v1/concepts/assets), you can directly reference the asset name like ‘usdc’ when performing operations like transfers. For assets that aren’t natively supported, you need to reference their [contract addresses](https://developer.chrome.com/server-wallets/v1/concepts/assets#assets-supported-by-contract-address) as shown in our docs.

### Where can I find information about the CDP data API rate limit?

For all CDP APIs, we have rate limits at IP level and CDP project level. These limits vary based on the API, but we are lenient and have limits sufficient for most use cases. Please reach out on our [Discord](https://discord.com/invite/cdp) through the #cdp-sdk channel if hitting the limits.

### Is there a CDP SDK available for Rust, PHP, Java, etc…?

The CDP SDK is available in [TypeScript](https://coinbase.github.io/coinbase-sdk-nodejs/index.html), and [Python](https://coinbase.github.io/cdp-sdk-python/index.html). Feel free to submit requests/feedback on our [Discord](https://discord.com/invite/cdp) if you are interested in support for additional languages.

### Do wallets work cross-network?

No, Wallets created by the SDK are not currently interoperable across networks. If you want to export the seed to recover funds using a Wallet Ext such as CB Wallet or Metamask, you can [export the private key](https://developer.chrome.com/server-wallets/v1/concepts/wallets#exporting-wallets-to-an-external-provider).

### Can I simulate read calls?

No, simulating read calls is not currently possible in the SDK. [Viem Simulate](https://viem.sh/docs/actions/public/simulateCalls), or [Tenderly Transaction Previews](https://tenderly.co/transaction-previews) can be used for accomplishing this in the meantime.

### I am getting issues with my frontend

CDP SDK is for the backend. Please see the [Smart Wallet](https://docs.base.org/identity/smart-wallet/guides/react-native-integration) or [Onchainkit](https://docs.base.org/builderkits/onchainkit/getting-started) docs for help depending on which frontend product you are using.

### How do I import my key into Metamask or Coinbase Wallet?

In order to export the private key for your address that can be used from Metamask, use the export method in the address object. We support this in our [TypeScript and Python SDKs](https://developer.chrome.com/server-wallets/v1/concepts/wallets#exporting-wallets-to-an-external-provider).

### How can we get the gas price used by CDP-SDK?

We do not expose details on gas price today.

### When checking the balance of my wallet, only some of the tokens show up. Is there a predefined list of tokens for which the balance is shown?

ListBalances only shows a specific list of assets as listed [here](https://developer.chrome.com/server-wallets/v1/concepts/assets). For other assets, you can list the balance by passing the contract address as shown [here](https://developer.chrome.com/server-wallets/v1/concepts/wallets#retrieving-balances).

### When creating a wallet with the CDP, we can export the seed. When using Viem to create a wallet with the same seed, the address is different. How might I get Viem to create the wallet with the same address as Server Wallet?

Wallets created with the Server Wallet are HD wallets. Private keys for your address can be derived as shown [here](https://developer.chrome.com/server-wallets/v1/concepts/addresses)

### How do I get the transaction hash after a successful asset transfer?

The transfer object has a getTransactionHash() function in order to get the hash.

```
const hash = transfer.getTransactionHash()

```

### Can I implement paymaster for Server Wallet when interacting with a smart contract?

No, this currently isn’t possible with Server Wallet. You’ll want to create Smart wallets instead (see our [Smart Wallet docs](https://developer.chrome.com/server-wallets/v1/concepts/smart-wallets#paymaster).

### Can I see the list of wallets we created using CDP SDK in the Developer Portal?

Not currently. We are shipping the ability to see wallets created using the CDP SDK in the [Portal](https://portal.cdp.coinbase.com/) soon. Today, you can list wallets with a call to the SDK - [Wallet.list()](https://developer.chrome.com/api-reference/rest-api/wallets/list-wallets).

### How do I re-import my wallet previously made with CDP SDK?

You can import a wallet using the previously saved Seed and ID of the wallet. See [Re-Instantiating a Wallet](https://developer.chrome.com/server-wallets/v1/concepts/wallets#re-instantiating-a-wallet) for details.

### Can I use a wallet created through another provider or extension? Or Import a previously created seed phrase?

Yes, you can easily [import wallets](https://developer.chrome.com/server-wallets/v1/concepts/wallets#importing-a-wallet) from other tools like a BIP-39 mnemonic seed phrase from your existing wallet (ie, from MetaMask, Coinbase Wallet app, etc.) into the CDP ecosystem.

### What types of compliance checks does the [Transfer API](https://developer.chrome.com/server-wallets/v1/concepts/transfers) perform?

We perform OFAC sanction checks against the destination address for all sends.

## Getting Help

If you need additional support:

-   Join our [Discord community](https://discord.com/invite/cdp)
-   Check our [API Reference Documentation](https://coinbase.github.io/cdp-sdk-python/index.html)
-   Review our code examples in the documentation.