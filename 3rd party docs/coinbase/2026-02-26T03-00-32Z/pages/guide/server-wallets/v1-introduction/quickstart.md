# quickstart

The CDP SDK allows you to create wallets and send funds onchain within minutes. In this quickstart, you will learn how to create a wallet, fund it with testnet ETH, transfer funds between wallets, and trade assets.

## What You’ll Learn

-   How to install the CDP SDK
-   How to create a [Developer-Managed Wallet](https://developer.chrome.com/server-wallets/v1/concepts/wallets#developer-managed-wallets) and view its default address
-   How to fund your wallet with testnet ETH
-   How to transfer funds between wallets
-   How to trade assets in a wallet

## Requirements

-   Typescript
    
-   Python
    

**Node.js 18+**The Coinbase server-side SDK requires Node.js version 18 or higher and npm version 9.7.2 or higher. To view your currently installed versions of Node.js, run the following from the command-line:

We recommend installing and managing Node.js and npm versions with `nvm`. See [Installing and Updating](https://github.com/nvm-sh/nvm?tab=readme-ov-file#installing-and-updating) in the `nvm` README for instructions on how to install `nvm`.Once `nvm` has been installed, you can install and use the latest versions of Node.js and npm by running the following commands:

```
nvm install node # "node" is an alias for the latest version
nvm use node

```

**Python 3.10+**Before using the SDK, ensure that you have the correct version of Python installed and the `pip` package manager. The SDK requires Python 3.10 or higher. You can check your Python version and `pip` installation by running the following code:

```
python --version
pip --version

```

If you need to upgrade your Python version, you can download and install the latest version of Python from the [official Python website](https://www.python.org/downloads/). For `pip`, refer to the [official pip documentation](https://pip.pypa.io/en/stable/installation/) for installation instructions.

## Installation

-   Typescript
    
-   Python
    

#### Clone CDP SDK quickstart template

The CDP SDK provides a [quickstart template](https://github.com/coinbase/coinbase-sdk-nodejs/tree/master/quickstart-template) to get started with the SDK. Clone the repository and navigate to the quickstart template directory:

```
git clone git@github.com:coinbase/coinbase-sdk-nodejs.git; cd coinbase-sdk-nodejs/quickstart-template

```

Install the dependencies:

The file `index.js` contains the code to perform your first transfer with the CDP SDK. Let’s break down the content of this file.

#### Onchain Payment Replit Template

There is a [Replit template](https://replit.com/@CoinbaseDev/Python-Onchain-Payments?v=1#README.md) for enabling automated mass payments using the CDP SDK in Python.Run the following from the command line:

**For `python` REPL**Use the Python Standard REPL (`python`) to leverage Python’s built-in REPL and quickly explore the functionality of our SDK.Run the following from the command line:

After running `python`, import the module:

## Creating a Wallet

The following instructions illustrate how to create a Developer-Managed (1-of-1) Wallet from scratch, using the CDP SDK.

1.  [Create a CDP Secret API key](https://portal.cdp.coinbase.com/projects/api-keys)
2.  Initialize the CDP SDK by passing your downloaded API key file
3.  Create a new Developer-Managed (1-of-1) wallet

-   Typescript
    
-   Python
    

Initialize the SDK by passing your API key information:

```
import { Coinbase, Wallet } from "@coinbase/coinbase-sdk";
const apiKeyName = "Copy your secret API key name here."
const apiKeyPrivateKey = "Copy your secret API key's private key here."
Coinbase.configure(apiKeyName, apiKeyPrivateKey)

```

Another way to initialize the SDK is by sourcing the API key from the JSON file that contains your secret API key, downloaded from the CDP portal:

```
let coinbase = Coinbase.configureFromJson({ filePath: '~/Downloads/cdp_api_key.json' });

```

Now create a wallet:

```
// Create a new Wallet
let wallet = await Wallet.create();
console.log(`Wallet successfully created: `, wallet.toString());
// Wallets are not saved locally by default. Refer to the Wallets concept for more information.

```

Wallets are initialized with a single default Address, accessible via getDefaultAddress:

```
let address = await wallet.getDefaultAddress();
console.log(`Default address for the wallet: `, address.toString());

```

Initialize the SDK by passing your API key information:

```
api_key_name = "Copy your secret API key name here."
api_key_private_key = "Copy your secret API key's private key here."
Cdp.configure(api_key_name, api_key_private_key)
print("CDP SDK has been successfully configured with CDP API key.")

```

Another way to initialize the SDK is by sourcing the API key from the JSON file that contains your secret API key, downloaded from the CDP portal:

```
Cdp.configure_from_json("~/Downloads/cdp_api_key.json")
print("CDP SDK has been successfully configured from JSON file.")

```

Now create a wallet:

```
# Create a new wallet.
wallet = Wallet.create()
# Wallets are not saved locally by default. Refer to the Wallets concept for more information.

```

Wallets are initialized with a single default address, accessible via `default_address`:

```
address = wallet.default_address
print(f"Default address for the wallet: {address.address_id}")

```

The wallet created should be persisted to avoid losing access to it. Refer to [Persisting a wallet](https://developer.chrome.com/server-wallets/v1/concepts/wallets#developer-managed-wallets) section for more information.

## Importing a Wallet

The following instructions illustrate how to bring your own wallet into the CDP ecosystem, as a Developer-Managed (1-of-1) Wallet, using the CDP SDK.

1.  [Create a CDP Secret API key](https://portal.cdp.coinbase.com/projects/api-keys)
2.  Initialize the CDP SDK by passing your downloaded API key file
3.  Create a new Developer-Managed (1-of-1) wallet using your [BIP-39 mnemonic seed phrase](https://www.coinbase.com/learn/wallet/what-is-a-seed-phrase)

-   Typescript
    
-   Python
    

Initialize the SDK by passing your API key information:

```
import { Coinbase, Wallet } from "@coinbase/coinbase-sdk";
const apiKeyName = "Copy your secret API key name here."
const apiKeyPrivateKey = "Copy your secret API key's private key here."
Coinbase.configure(apiKeyName, apiKeyPrivateKey)

```

Another way to initialize the SDK is by sourcing the API key from the JSON file that contains your secret API key, downloaded from the CDP portal:

```
let coinbase = Coinbase.configureFromJson({ filePath: '~/Downloads/cdp_api_key.json' });

```

Now import your wallet:

```
// Import your Wallet into CDP using your BIP-39 mnemonic seed phrase.
// NOTE 1: For security reasons, we recommend storing your seed phrase in an environment variable.
// NOTE 2: Your wallet's seed and seed phrase will not leave your device.
let wallet = await Wallet.import({ mnemonicPhrase: process.env.MNEMONIC_PHRASE });
console.log(`Wallet successfully created: `, wallet.toString());
// Wallets are not saved locally by default. Refer to the Wallets concept for more information.

```

Wallets are initialized with a single default Address, accessible via getDefaultAddress:

```
let address = await wallet.getDefaultAddress();
console.log(`Default address for the wallet: `, address.toString());

```

Initialize the SDK by passing your API key information:

```
api_key_name = "Copy your secret API key name here."
api_key_private_key = "Copy your secret API key's private key here."
Cdp.configure(api_key_name, api_key_private_key)
print("CDP SDK has been successfully configured with CDP API key.")

```

Another way to initialize the SDK is by sourcing the API key from the JSON file that contains your secret API key, downloaded from the CDP portal:

```
Cdp.configure_from_json("~/Downloads/cdp_api_key.json")
print("CDP SDK has been successfully configured from JSON file.")

```

Now import your wallet:

```
# Import your Wallet into CDP using your BIP-39 mnemonic seed phrase.
# NOTE 1: For security reasons, we recommend storing your seed phrase in an environment variable.
# NOTE 2: Your wallet's seed and seed phrase will not leave your device.
imported_wallet = Wallet.import_wallet(MnemonicSeedPhrase(os.getenv("MNEMONIC_PHRASE")))
# Wallets are not saved locally by default. Refer to the Wallets concept for more information.

```

Wallets are initialized with a single default address, accessible via `default_address`:

```
address = wallet.default_address
print(f"Default address for the wallet: {address.address_id}")

```

Once initialized, your imported wallet should be stored as a Wallet data object, for easy re-instantiation. Refer to [Persisting a wallet](https://developer.chrome.com/server-wallets/v1/concepts/wallets#developer-managed-wallets) section for more information.

## Funding a Wallet

Wallets do not have funds on them to start. For Base Sepolia and Ethereum Sepolia testnets, we provide a faucet method to fund your wallet with testnet ETH.

## Transferring Funds

Now that your faucet transaction has successfully completed, you can send the funds in your wallet to another wallet. The code below creates another wallet, and sends testnet ETH from the first wallet to the second:

See [Transfers](https://developer.chrome.com/server-wallets/v1/concepts/transfers) for more information. You can create your wallet, fund it with testnet tokens and perform your first transfer by running the following command:

## Trading Assets

On `base-mainnet` you can trade between different assets from your wallet. Since trading is only supported on mainnet wallets, wallet should be funded with real assets before trading. The code below creates a wallet and trades some ETH to USDC and then all of the USDC to WETH:

-   Typescript
    
-   Python
    

Refer to `trade.js` for a complete example of trading assets.

```
import { Coinbase, Wallet } from "@coinbase/coinbase-sdk";
let coinbase = Coinbase.configureFromJson({ filePath: '~/Downloads/cdp_api_key.json' });
// Create a Wallet on base-mainnet to trade assets with.
let wallet = await Wallet.create({ networkId: Coinbase.networks.BaseMainnet });
// Fund the Wallet's default Address with ETH from an external source.
// Trade 0.00001 ETH to USDC.
let trade = await wallet.createTrade({
  amount: 0.00001,
  fromAssetId: Coinbase.assets.Eth,
  toAssetId: Coinbase.assets.Usdc
});
await trade.wait();
if (trade.getStatus() === 'complete') {
  console.log(`Trade successfully completed: `, trade.toString());
} else {
  console.log(`Trade failed on-chain: `, trade.toString());
}
// Trade the wallet's full balance of USDC to WETH.
let trade2 = await wallet.createTrade({
  amount: wallet.getBalance(Coinbase.assets.Usdc),
  fromAssetId: Coinbase.assets.Usdc,
  toAssetId: Coinbase.assets.Weth,
});
await trade2.wait();
if (trade2.getStatus() === "complete") {
  console.log(`Trade successfully completed: `, trade2.toString());
} else {
  console.log(`Trade failed on-chain: `, trade2.toString());
}

```

See [Trades](https://developer.chrome.com/server-wallets/v1/concepts/trades) for more information.

```
# Create a wallet on `base-mainnet` to trade assets with.
wallet = Wallet.create(network_id="base-mainnet")
print("Wallet successfully created: {wallet}")
# Fund wallet's default address with ETH from an external source.
# Trade 0.00001 ETH to USDC
trade = wallet.trade(0.00001, "eth", "usdc").wait()
if trade.status is Transaction.Status.COMPLETE:
  print(f"Trade successfully completed: {trade}")
else:
  print(f"Trade failed on-chain: {trade}")
# Trade the wallet's full balance of USDC to WETH
trade2 = wallet.trade(wallet.balance("usdc"), "usdc", "weth").wait()
if trade2.status is Transaction.Status.COMPLETE:
  print(f"Second trade successfully completed: {trade2}")
else:
  print(f"Second trade failed on-chain: {trade}")

```

See [Trades](https://developer.chrome.com/server-wallets/v1/concepts/trades) for more information.