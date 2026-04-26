# wallets

A wallet is a collection of [addresses](https://developer.chrome.com/server-wallets/v1/concepts/addresses) on a network. Wallets come with a single default address. Wallets can hold a balance of one or more assets. A wallet’s assets are controlled via the addresses’ private keys, which in turn are derived from a seed. Think of a seed / private key as the password to a wallet. For more, see [What is a private key?](https://www.coinbase.com/learn/crypto-basics/what-is-a-private-key). Wallets created within the CDP SDK can be either [Coinbase-Managed](#coinbase-managed-wallets) or [Developer-Managed](#developer-managed-wallets), based on how the wallet’s private keys are managed. Users can also import existing wallets via the `import` method and using a seed phrase. Wallets can create new addresses, list their addresses, list their balances and transfer assets to other addresses or wallets. Wallets are created on a specific network. Certain features are only available on certain networks. For example, [faucets](https://developer.chrome.com/faucets/introduction/welcome) are only available on Base Sepolia and Ethereum Sepolia. [Trades](https://developer.chrome.com/server-wallets/v1/concepts/trades) are only available on Base Mainnet.

## Creating a Wallet

-   Typescript
    
-   Python
    

**SDK Documentation**Refer to the [Wallet class SDK docs](https://coinbase.github.io/coinbase-sdk-nodejs/classes/coinbase_wallet.Wallet.html) for a full list of supported methods.

```
let wallet = await Wallet.create();

```

A wallet starts with a single `defaultAddress`. You can also create more addresses in the wallet, and list them:

```
// Get the default_address in the wallet.
let address = await wallet.getDefaultAddress();
console.log(`Address: ${address}`);
// Create another address in the wallet.
let address2 = await wallet.createAddress();
console.log(`Address: ${address2}`);
// List the two addresses in the wallet.
let addresses = wallet.getAddresses();

```

**SDK Documentation**Refer to the [Wallet class SDK docs](https://coinbase.github.io/cdp-sdk-python/cdp.html#cdp.Wallet) for a full list of supported methods.

A wallet starts with a single `default_address`. You can also create more addresses in the wallet, and list them:

```
# Get the default_address in the wallet.
address1 = wallet.default_address
# Create another address in the wallet.
address2 = wallet.create_address()
# List the two addresses in the wallet.
wallet.addresses

```

By default, wallets are created for Base Sepolia. The CDP SDK also supports creating wallets for the following networks. To do that, pass the network ID as an argument:

## Securing a Wallet

There are [two types of wallets](https://developer.chrome.com/server-wallets/v1/concepts/wallet-types) that can be created using the CDP SDK, depending on how the private keys are managed: Coinbase-Managed (2-of-2) Wallets and Developer-Managed (1-of-1) Wallets. Developer-Managed wallets are best for rapid testing and prototyping, while Coinbase-Managed wallets are recommended for any production environments.

### Coinbase-Managed Wallets

Server Wallet offers a state-of-the-art Multi-Party Computation (MPC) option that splits private keys into two shares between Coinbase and the developer, ensuring improved security. Even if a developer’s share of the private key is compromised, assets will not be at risk as long as the CDP API keys and account credentials remain secure. These Coinbase-Managed (2-of-2) wallets use the [Server-Signer](https://developer.chrome.com/server-wallets/v1/concepts/server-signer), a deployable component that simplifies key management and provides a secure way to sign transactions. For production applications requiring maximal security, we recommend using Coinbase-Managed Wallets.

### Developer-Managed Wallets

For Developer-Managed (1-of-1) Wallets, it is your responsibility as the developer to securely store the data required to re-instantiate your wallets. For example, you may choose to store this data in an encrypted database. As with any 1-of-1 wallet solution, losing access to the wallet could result in a loss of funds. The CDP SDK provides two key pieces of information to persist Developer-Managed (1-of-1) Wallets:

-   Seed: a 32-byte hexadecimal string. This seed is used to derive all of the private keys in the wallet and provides access to spend the assets in the wallet.
-   Wallet ID: a string used to identify the wallet.

This information is encapsulated in a wallet’s export data, obtained by calling the `export` method:

It is your responsibility as the developer to securely store the seeds and wallet IDs required to re-instantiate your wallets. For example, you may choose to store this data in an encrypted database.

#### Persisting Locally

For convenience, we provide a method that stores the wallet seed to a local file that you specify.

To save your wallet seed, run the following:

#### Re-instantiating a Wallet

The seed and the ID of the wallet are required to re-instantiate a wallet when a new session is started. This data is encapsulated in the export data of a wallet, which should be securely persisted by the developer. The following code demonstrates how to import the data required to re-instantiate a wallet.

##### Hydrating a Wallet

Another method of re-instantiating a wallet is to “hydrate” it. Hydration consists of two parts:

-   Fetching the wallet from the server
-   Setting the correct seed on the wallet

A wallet that is fetched from the server is at first unhydrated, because only you, the developer, have access to the wallet’s seed, and the wallet is unaware of its own seed. Unhydrated wallets can perform read operations, such as viewing balances and addresses, but not write operations, such as creating new addresses or transferring funds. The code below demonstrates the process of fetching an unhydrated wallet, and hydrating it with a seed:

```
// Get the unhydrated wallet from the server.
const fetchedWallet = await Wallet.fetch(wallet.getId());
// The fact that fetchedWallet is unhydrated is encapsulated by the canSign method.
// For example, calling fetchedWallet.createAddress() would throw an error.
console.log(`fetchedWallet is hydrated: ${fetchedWallet.canSign()}`);
// To hydrate the wallet, set the correct seed on it.
fetchedWallet.setSeed(fetchedData.seed);
// The wallet is now hydrated, and can create addresses and sign transactions.
console.log(`fetchedWallet is hydrated: ${fetchedWallet.canSign()}`);

```

##### Hydrating Locally

## Importing a Wallet

The CDP SDK allows you to import your own wallet via a mnemonic seed phrase, so that you can bring your existing wallets into the CDP ecosystem.

-   **Easily import wallets from other tools**: Use your [BIP-39 mnemonic seed phrase](https://www.coinbase.com/learn/wallet/what-is-a-seed-phrase) to import your existing wallet (ie, from [MetaMask](https://metamask.io/), [Coinbase Wallet app](https://www.coinbase.com/wallet), etc.) into the CDP ecosystem, allowing you to create complex, programmatic, or agentic interactions.
-   **1-of-1 (Developer-Managed) security**: Your wallet’s seed and seed phrase will not leave your device.

To import a wallet, use the following commands. Please note that defining your seed phrase within an environment variable is recommended for security.

Once your wallet has been imported, you won’t need to import it again:

-   [Export](https://developer.chrome.com/server-wallets/v1/concepts/wallets) your wallet data (includes your `seed` and `wallet ID`) to your desired storage medium.
-   [Re-instantiate](#re-instantiating-a-wallet) your wallet at any time using your exported wallet data, without needing to use your mnemonic seed phrase.

## Retrieving Balances

To view the amount of [assets](https://developer.chrome.com/server-wallets/v1/concepts/assets) held in a wallet, call the following:

Note that `list` method only returns balances for the top 20 [assets supported by symbol](https://developer.chrome.com/server-wallets/v1/concepts/assets#assets-supported-by-symbol). For other assets, use `get` as follows.

## Creating webhook

You can create a webhook for the current wallet. The webhook allow you to receive real-time notifications of wallet activity directly to your application via a specified callback notification URL. By creating a webhook, you can monitor events related to all the addresses for a wallet. See [Webhook page](https://developer.chrome.com/data/webhooks/overview) for more details on supported event types, event payload and supported networks.

```
let webhook = await wallet.createWebhook('https://call_back_uri_for_webhook')

```

## Exporting wallets to an external provider

API Wallets provide exportable private keys compatible with all major wallet providers, such as Coinbase Wallet and MetaMask. To export the private key for an address that can be imported into an external provider, use the following code snippet:

## What to read next

-   [**Faucets**](https://developer.chrome.com/faucets/introduction/welcome): An overview of CDP Faucets and how to claim testnet funds onchain
-   [**Addresses**](https://developer.chrome.com/server-wallets/v1/concepts/addresses): Create and manage addresses in your wallet
-   [**Assets**](https://developer.chrome.com/server-wallets/v1/concepts/assets): View and manage assets in your wallet