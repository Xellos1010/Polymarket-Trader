# wallet management

AgentKit supports multiple wallet providers, with [CDP Server Wallet](https://developer.chrome.com/server-wallets/v1/introduction/welcome) being the default implementation. It supports the following operations, alongside compatibility with the full suite of CDP products including Onramp, and Commerce:

-   [Creating API Wallets](https://developer.chrome.com/server-wallets/v1/concepts/wallets)
-   [Signing transactions](https://developer.chrome.com/server-wallets/v1/introduction/onchain-interactions/message-signing)
-   [Deploying and interacting with tokens](https://developer.chrome.com/server-wallets/v1/introduction/onchain-interactions/smart-contract-deployments)
-   [Invoking smart contracts and querying chain state](https://developer.chrome.com/server-wallets/v1/introduction/onchain-interactions/smart-contract-interactions)

## Wallet Configuration

You can configure AgentKit to use a CDP wallet or a custom wallet provider.

-   CDP Wallet v2
    
-   CDP Wallet
    
-   Viem/EthAccount Wallet
    
-   Privy Server Wallet (EVM/Solana)
    
-   Solana Keypair Wallet
    

The [CDP Wallet v2 API](https://developer.chrome.com/server-wallets/v2/introduction/welcome) is the recommended wallet provider for AgentKit. It supports both EVM and Solana chains, seamlessly and securely handles private keys, and is compatible with viem.

-   Typescript
    
-   Python
    

```
import {
  AgentKit,
  CdpV2WalletProvider,
  CdpV2EvmWalletProvider, // for evm
  CdpV2SolanaWalletProvider, // for solana
} from "@coinbase/agentkit";
// Configure CDP Wallet Provider
const cdpWalletConfig = {
  apiKeyId: process.env.CDP_API_KEY_ID, // from https://portal.cdp.coinbase.com , see v2 wallet quickstart for more details
  apiKeySecret: process.env.CDP_API_KEY_SECRET,
  walletSecret: process.env.CDP_WALLET_SECRET,
  idempotencyKey: process.env.IDEMPOTENCY_KEY, // optional, used for account creation
  address: process.env.ADDRESS as `0x${string}` | undefined, // optional, used for existing account
  networkId: process.env.NETWORK_ID, // e.g. "base", "base-sepolia", "solana"
};
const walletProvider = await CdpV2WalletProvider.configureWithWallet(cdpWalletConfig);
// ...
// Initialize AgentKit
const agentkit = await AgentKit.from({
  walletProvider,
  actionProviders: [],
});

```

```
from coinbase_agentkit import (
    AgentKit,
    AgentKitConfig,
    CdpEvmServerWalletProvider,
    CdpEvmServerWalletProviderConfig,
)
# Initialize the wallet provider with the config
wallet_provider = CdpEvmServerWalletProvider(
    CdpEvmServerWalletProviderConfig(
        api_key_id=config.api_key_id,  # CDP API Key ID
        api_key_secret=config.api_key_secret,  # CDP API Key Secret
        wallet_secret=config.wallet_secret,  # CDP Wallet Secret
        network_id=config.network_id,  # Network ID - Optional, will default to 'base-sepolia'
        address=config.address,  # Wallet Address - Optional, will trigger idempotency flow if not provided
        idempotency_key=config.idempotency_key,  # Idempotency Key - Optional, seeds generation of a new wallet
    )
)
# Create AgentKit instance with wallet and action providers
agentkit = AgentKit(
    AgentKitConfig(
        wallet_provider=wallet_provider,
        action_providers=[],
    )
)

```

CDP Server Wallet is the default wallet provider for AgentKit. It is configured with an API key and optional network ID. You can find a list of supported networks [here](https://developer.chrome.com/get-started/supported-networks).

-   Typescript
    
-   Python
    

```
import { CdpWalletProvider } from "@coinbase/agentkit";
const walletProvider = await CdpWalletProvider.configureWithWallet({
    apiKeyName: "CDP API KEY NAME",
    apiKeyPrivate: "CDP API KEY PRIVATE KEY",
    networkId: "base-mainnet",
});
// ...
// Initialize AgentKit
const agentkit = await AgentKit.from({
  walletProvider,
  actionProviders: [],
});

```

**Configuring from an existing CDP API Wallet**If you already have a CDP API Wallet, you can configure the `CdpWalletProvider` by passing the `wallet` parameter to the `configureWithWallet` method.

```
import { CdpWalletProvider } from "@coinbase/agentkit";
import { Wallet } from "@coinbase/coinbase-sdk";
const walletProvider = await CdpWalletProvider.configureWithWallet({
    wallet,
    apiKeyName: "CDP API KEY NAME",
    apiKeyPrivate: "CDP API KEY PRIVATE KEY",
});

```

**Configuring from a mnemonic phrase**The `CdpWalletProvider` can be configured from a mnemonic phrase by passing the `mnemonicPhrase` parameter to the `configureWithWallet` method.

```
import { CdpWalletProvider } from "@coinbase/agentkit";
const walletProvider = await CdpWalletProvider.configureWithWallet({
    mnemonicPhrase: "MNEMONIC PHRASE",
});

```

**Exporting a wallet**The `CdpWalletProvider` can export a wallet by calling the `exportWallet` method.

```
import { CdpWalletProvider } from "@coinbase/agentkit";
const walletProvider = await CdpWalletProvider.configureWithWallet({
    mnemonicPhrase: "MNEMONIC PHRASE",
});
const walletData = await walletProvider.exportWallet();

```

**Importing a wallet from `WalletData` JSON string**The `CdpWalletProvider` can import a wallet from a `WalletData` JSON string by passing the `cdpWalletData` parameter to the `configureWithWallet` method.

```
import { CdpWalletProvider } from "@coinbase/agentkit";
const walletProvider = await CdpWalletProvider.configureWithWallet({
    cdpWalletData: "WALLET DATA JSON STRING",
    apiKeyName: "CDP API KEY NAME",
    apiKeyPrivate: "CDP API KEY PRIVATE KEY",
});

```

```
from coinbase_agentkit import CdpWalletProvider, CdpWalletProviderConfig
wallet_provider = CdpWalletProvider(CdpWalletProviderConfig(
    api_key_name="CDP API KEY NAME",
    api_key_private="CDP API KEY PRIVATE KEY",
    network_id="base-mainnet",
))
wallet_provider = CdpWalletProvider(cdp_config)
agentkit = AgentKit.from_options(AgentKitOptions(
    wallet_provider=wallet_provider,
    action_providers=[],
))

```

**Configuring from an existing CDP API Wallet**If you already have a CDP API Wallet, you can configure the `CdpWalletProvider` by passing the `wallet` parameter to the `configureWithWallet` method.

```
wallet_provider = CdpWalletProvider(CdpWalletProviderConfig(
    wallet=wallet,
    api_key_name="CDP API KEY NAME",
    api_key_private="CDP API KEY PRIVATE KEY",
))

```

**Configuring from a mnemonic phrase**The `CdpWalletProvider` can be configured from a mnemonic phrase by passing the `mnemonic_phrase` parameter to the `CdpWalletProviderConfig`.

```
from coinbase_agentkit import CdpWalletProvider, CdpWalletProviderConfig
wallet_provider = CdpWalletProvider(CdpWalletProviderConfig(
    mnemonic_phrase="MNEMONIC PHRASE",
))

```

**Exporting a wallet**The `CdpWalletProvider` can export a wallet by calling the `export_wallet` method.

```
from coinbase_agentkit import CdpWalletProvider
wallet_provider = CdpWalletProvider(CdpWalletProviderConfig(
    mnemonic_phrase="MNEMONIC PHRASE",
))
wallet_data = wallet_provider.export_wallet()

```

**Importing a wallet from `WalletData` JSON string**The `CdpWalletProvider` can import a wallet from a `WalletData` JSON string by passing the `cdp_wallet_data` parameter to the `CdpWalletProviderConfig`.

```
from coinbase_agentkit import CdpWalletProvider, CdpWalletProviderConfig
wallet_provider = CdpWalletProvider(CdpWalletProviderConfig(
    wallet_data="WALLET DATA JSON STRING",
    api_key_name="CDP API KEY NAME",
    api_key_private="CDP API KEY PRIVATE KEY",
))

```

-   Typescript
    
-   Python
    

The `ViemWalletProvider` is a wallet provider that uses the [Viem library](https://viem.sh/docs/getting-started). It is useful for interacting with any EVM-compatible chain.

```
import { ViemWalletProvider } from "@coinbase/agentkit";
import { privateKeyToAccount } from "viem/accounts";
import { baseSepolia } from "viem/chains";
import { http } from "viem/transports";
import { createWalletClient } from "viem";
const account = privateKeyToAccount(
  "0x4c0883a69102937d6231471b5dbb6208ffd70c02a813d7f2da1c54f2e3be9f38",
);
const client = createWalletClient({
  account,
  chain: baseSepolia,
  transport: http(),
});
const walletProvider = new ViemWalletProvider(client);

```

Example usage with a private key:

```
from eth_account import Account
from coinbase_agentkit import (
    AgentKit, 
    AgentKitConfig, 
    EthAccountWalletProvider, 
    EthAccountWalletProviderConfig
)
# See here for creating a private key:
# https://web3py.readthedocs.io/en/stable/web3.eth.account.html#creating-a-private-key
private_key = os.environ.get("PRIVATE_KEY")
assert private_key is not None, "You must set PRIVATE_KEY environment variable"
assert private_key.startswith("0x"), "Private key must start with 0x hex prefix"
account = Account.from_key(private_key)
wallet_provider = EthAccountWalletProvider(
    config=EthAccountWalletProviderConfig(
        account=account,
        chain_id=84532,
    )
)
agent_kit = AgentKit(AgentKitConfig(
    wallet_provider=wallet_provider
))

```

-   EVM
    
-   Solana
    

The `PrivyWalletProvider` is a wallet provider that uses [Privy Server Wallets](https://docs.privy.io/guide/server-wallets/). This implementation extends the `ViemWalletProvider`.

```
import { PrivyWalletProvider, PrivyWalletConfig } from "@coinbase/agentkit";
// Configure Wallet Provider
const config: PrivyWalletConfig = {
    appId: "PRIVY_APP_ID",
    appSecret: "PRIVY_APP_SECRET",
    chainId: "84532", // optional, defaults to 84532 (base-sepolia)
    walletId: "PRIVY_WALLET_ID", // optional, otherwise a new wallet will be created
    authorizationPrivateKey: PRIVY_WALLET_AUTHORIZATION_PRIVATE_KEY, // optional, required if your account is using authorization keys
    authorizationKeyId: PRIVY_WALLET_AUTHORIZATION_KEY_ID, // optional, only required to create a new wallet if walletId is not provided
};
const walletProvider = await PrivyWalletProvider.configureWithWallet(config);

```

The `PrivyWalletProvider` is a wallet provider that uses [Privy Server Wallets](https://docs.privy.io/guide/server-wallets/).

```
import { PrivyWalletProvider, PrivyWalletConfig } from "@coinbase/agentkit";
// Configure Wallet Provider
const config: PrivyWalletConfig = {
    appId: "PRIVY_APP_ID",
    appSecret: "PRIVY_APP_SECRET",
    chainType: "solana", // optional, defaults to "evm". Make sure to set this to "solana" if you want to use Solana!
    networkId: "solana-devnet", // optional, defaults to "solana-devnet"
    walletId: "PRIVY_WALLET_ID", // optional, otherwise a new wallet will be created
    authorizationPrivateKey: PRIVY_WALLET_AUTHORIZATION_PRIVATE_KEY, // optional, required if your account is using authorization keys
    authorizationKeyId: PRIVY_WALLET_AUTHORIZATION_KEY_ID, // optional, only required to create a new wallet if walletId is not provided
};
const walletProvider = await PrivyWalletProvider.configureWithWallet(config);

```

**Custom Solana Connection**Optionally, you can configure your own `@solana/web3.js` connection by passing the `connection` parameter to the `configureWithWallet` method.

```
import { PrivyWalletProvider, PrivyWalletConfig } from "@coinbase/agentkit";
const connection = new Connection("YOUR_RPC_URL");
// Configure Wallet Provider
const config: PrivyWalletConfig = {
    appId: "PRIVY_APP_ID",
    appSecret: "PRIVY_APP_SECRET",
    connection,
    chainType: "solana", // optional, defaults to "evm". Make sure to set this to "solana" if you want to use Solana!
    networkId: "solana-devnet", // optional, defaults to "solana-devnet"
    walletId: "PRIVY_WALLET_ID", // optional, otherwise a new wallet will be created
    authorizationPrivateKey: PRIVY_WALLET_AUTHORIZATION_PRIVATE_KEY, // optional, required if your account is using authorization keys
    authorizationKeyId: PRIVY_WALLET_AUTHORIZATION_KEY_ID, // optional, only required to create a new wallet if walletId is not provided
};
const walletProvider = await PrivyWalletProvider.configureWithWallet(config);

```

**Authorization Keys**Privy offers the option to use authorization keys to secure your server wallets.You can manage authorization keys from your [Privy dashboard](https://dashboard.privy.io/account) or programmatically [using the API](https://docs.privy.io/guide/server-wallets/authorization/signatures).When using authorization keys, you must provide the `authorizationPrivateKey` and `authorizationKeyId` parameters to the `configureWithWallet` method if you are creating a new wallet. Please note that when creating a key, if you enable “Create and modify wallets”, you will be required to use that key when creating new wallets via the PrivyWalletProvider.**Exporting Privy Wallet information**The `PrivyWalletProvider` can export wallet information by calling the `exportWallet` method.

```
const walletData = await walletProvider.exportWallet();
// walletData will be in the following format:
{
    walletId: string;
    authorizationKey: string | undefined;
    networkId: string | undefined;
}

```

The [`SolanaKeypairWalletProvider`](https://github.com/coinbase/agentkit/blob/main/typescript/agentkit/src/wallet-providers/solanaKeypairWalletProvider.ts) is a wallet provider that uses the [Solana web3.js API](https://solana-foundation.github.io/solana-web3.js/).**Solana Network Configuration**The `SolanaKeypairWalletProvider` can be configured to use a specific network by passing the `networkId` parameter to the `fromNetwork` method. The `networkId` is the ID of the Solana network you want to use. Valid values are `solana-mainnet`, `solana-devnet` and `solana-testnet`.The default RPC endpoints for each network are as follows:

-   `solana-mainnet`: `https://api.mainnet-beta.solana.com`
-   `solana-devnet`: `https://api.devnet.solana.com`
-   `solana-testnet`: `https://api.testnet.solana.com`

```
import { SOLANA_NETWORK_ID, SolanaKeypairWalletProvider } from "@coinbase/agentkit";
// Configure Solana Keypair Wallet Provider
const privateKey = process.env.SOLANA_PRIVATE_KEY;
const network = process.env.NETWORK_ID as SOLANA_NETWORK_ID;
const walletProvider = await SolanaKeypairWalletProvider.fromNetwork(network, privateKey);

```

**RPC URL Configuration**The `SolanaKeypairWalletProvider` can be configured to use a specific RPC url by passing the `rpcUrl` parameter to the `fromRpcUrl` method. The `rpcUrl` will determine the network you are using.

```
import { SOLANA_NETWORK_ID, SolanaKeypairWalletProvider } from "@coinbase/agentkit";
// Configure Solana Keypair Wallet Provider
const privateKey = process.env.SOLANA_PRIVATE_KEY;
const rpcUrl = process.env.SOLANA_RPC_URL;
const walletProvider = await SolanaKeypairWalletProvider.fromRpcUrl(rpcUrl, privateKey);

```

## Default Operations

By default, AgentKit supports the following basic wallet operations:

-   `get_wallet_details` - Get details about the Wallet, like the address
-   `transfer` - Transfer assets between addresses
-   `get_balance` - Get the balance of an asset

You can add additional actions or action providers upon agent instantiation.