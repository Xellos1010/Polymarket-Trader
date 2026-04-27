# cdp solana standard wallet

## @coinbase/cdp-solana-standard-wallet

A React hook and wallet implementation that integrates CDP (Coinbase Developer Platform) embedded Solana wallets with the [Wallet Standard](https://github.com/wallet-standard/wallet-standard).

## Features

-   🔗 **Wallet Standard Integration** - Works with any dapp that supports the wallet standard
-   🔐 **CDP Authentication** - Uses CDP’s secure embedded wallet infrastructure
-   ⚛️ **React Hooks** - Easy-to-use hooks for React applications
-   🎯 **TypeScript Support** - Full type safety with TypeScript
-   🚀 **Auto-Registration** - Automatically registers with wallet standard ecosystem
-   🔄 **Event-Driven** - Real-time updates via CDP auth state changes
-   📱 **Multi-Network** - Supports both Solana mainnet and devnet

## Installation

```
npm install @coinbase/cdp-solana-standard-wallet

```

### Peer Dependencies

```
npm install @coinbase/cdp-core react @wallet-standard/app @wallet-standard/base @wallet-standard/features @solana/wallet-standard bs58

```

### Optional Dependencies

Depending on your usage pattern, you may also need:

```
# For Option 1 (CDPHooksProvider)
npm install @coinbase/cdp-hooks
# For Option 2 (CDPReactProvider)
npm install @coinbase/cdp-react @coinbase/cdp-hooks

```

## Usage

### Option 1: With CDPHooksProvider

```
import { CDPHooksProvider } from '@coinbase/cdp-hooks';
import { useCdpSolanaStandardWallet } from '@coinbase/cdp-solana-standard-wallet';
// Wrap your app with CDPHooksProvider
function App() {
  return (
    <CDPHooksProvider config={{ projectId: 'your-project-id' }}>
      <MyWalletComponent />
    </CDPHooksProvider>
  );
}
// Use the hook in your component
function MyWalletComponent() {
  const { ready, wallet } = useCdpSolanaStandardWallet();
  if (!ready) {
    return <div>Loading wallet...</div>;
  }
  return (
    <div>
      <h3>CDP Solana Wallet Ready!</h3>
      <p>Name: {wallet.name}</p>
      <p>Accounts: {wallet.accounts.length}</p>
    </div>
  );
}

```

### Option 2: With CDPReactProvider

```
import { CDPReactProvider } from '@coinbase/cdp-react';
import { useCdpSolanaStandardWallet } from '@coinbase/cdp-solana-standard-wallet';
// Wrap your app with CDPReactProvider
function App() {
  return (
    <CDPReactProvider config={{
      projectId: 'your-project-id',
      solana: { createOnLogin: true }
    }}>
      <MyWalletComponent />
    </CDPReactProvider>
  );
}
// Use the hook in your component
function MyWalletComponent() {
  const { ready, wallet } = useCdpSolanaStandardWallet();
  if (!ready) {
    return <div>Loading wallet...</div>;
  }
  return (
    <div>
      <h3>CDP Solana Wallet Ready!</h3>
      <p>Name: {wallet.name}</p>
      <p>Accounts: {wallet.accounts.length}</p>
    </div>
  );
}

```

### Option 3: Standalone Usage

```
import { useCdpSolanaStandardWallet } from '@coinbase/cdp-solana-standard-wallet';
function MyWalletComponent() {
  // Pass config directly - hook will initialize CDP SDK
  const { ready, wallet } = useCdpSolanaStandardWallet({
    projectId: 'your-project-id'
  });
  if (!ready) {
    return <div>Initializing...</div>;
  }
  return <div>Wallet ready: {wallet.name}</div>;
}

```

### Analytics Opt-Out

By default the SDK will emit usage analytics to help us improve the SDK. If you would like to opt-out, you can do so by setting the `disableAnalytics` configuration option to `true`.

```
<CDPReactProvider config={{
  projectId: "your-project-id",
  disableAnalytics: true,
}}>

```

Or if you’re using the standalone usage, you can pass the `disableAnalytics` configuration option to the hook:

```
const { ready, wallet } = useCdpSolanaStandardWallet({
  projectId: "your-project-id",
  disableAnalytics: true,
});

```

### Using All Standard Wallets

```
import { useSolanaStandardWallets } from '@coinbase/cdp-solana-standard-wallet';
function WalletSelector() {
  const { wallets } = useSolanaStandardWallets();
  return (
    <div>
      <h3>Available Wallets:</h3>
      {wallets.map((wallet) => (
        <div key={wallet.name}>
          <img src={wallet.icon} alt={wallet.name} width="24" height="24" />
          <span>{wallet.name}</span>
          {wallet.features['cdp:'] && <span> (CDP)</span>}
        </div>
      ))}
    </div>
  );
}

```

### Direct Wallet Usage

```
import { getWallets } from '@wallet-standard/app';
// Find CDP wallet among all registered wallets
const wallets = getWallets();
const cdpWallet = wallets.get().find(w => w.features['cdp:']);
if (cdpWallet) {
  // Connect to the wallet
  const { accounts } = await cdpWallet.features['standard:connect'].connect();
  // Sign a transaction
  const result = await cdpWallet.features['solana:signTransaction'].signTransaction({
    transaction: transactionBytes,
    account: accounts[0],
    chain: 'solana:mainnet'
  });
  // Sign and send a transaction
  const sendResult = await cdpWallet.features['solana:signAndSendTransaction'].signAndSendTransaction({
    transaction: transactionBytes,
    account: accounts[0],
    chain: 'solana:mainnet'
  });
}

```

## API Reference

### `useCdpSolanaStandardWallet(config?)`

Hook that manages CDP Solana wallet creation and registration. **Parameters:**

-   `config` (optional): CDP configuration object with `projectId`

**Returns:**

```
{
  ready: boolean;           // True when wallet is ready to use
  wallet: CdpSolanaWallet | null;  // The wallet instance
}

```

### `useSolanaStandardWallets()`

Hook that returns all registered Solana wallets in the wallet standard. **Returns:**

```
{
  wallets: readonly Wallet[];  // Array of all registered wallets
}

```

### `CdpSolanaWallet`

The wallet implementation that integrates with CDP. **Properties:**

-   `name`: “CDP Solana Wallet”
-   `icon`: Solana logo as base64 data URI
-   `chains`: `['solana:mainnet', 'solana:devnet']`
-   `accounts`: Array of `ReadonlyWalletAccount`
-   `features`: Supported wallet standard features

**Supported Features:**

-   `standard:connect` - Connect to the wallet
-   `standard:disconnect` - Disconnect from the wallet
-   `standard:events` - Listen for wallet events
-   `solana:signTransaction` - Sign transactions
-   `solana:signAndSendTransaction` - Sign and send transactions
-   `solana:signMessage` - Sign arbitrary messages
-   `cdp:` - CDP-specific feature flag

## Authentication Flow

1.  **User Authentication**: User signs in via CDP (email/SMS OTP)
2.  **Wallet Creation**: CDP creates embedded Solana accounts
3.  **Hook Activation**: `useCdpSolanaStandardWallet` detects accounts
4.  **Wallet Registration**: Wallet is registered with wallet standard
5.  **Ready State**: `ready: true` indicates wallet is available
6.  **Dapp Integration**: Dapps can discover and use the wallet

## Network Support

-   **Mainnet**: `solana:mainnet` → CDP network: `solana`
-   **Devnet**: `solana:devnet` → CDP network: `solana-devnet`

## Error Handling

The hooks handle various error scenarios gracefully:

-   **Unauthenticated User**: `ready: false`, `wallet: null`
-   **No Solana Accounts**: `ready: false`, `wallet: null`
-   **CDP Initialization Failure**: Logs error, `ready: false`
-   **Network Errors**: Individual operations throw descriptive errors

## Security

-   All cryptographic operations performed by CDP’s secure infrastructure
-   Private keys never exposed to application code
-   Authentication handled via CDP’s secure auth flow
-   Wallet disconnect clears wallet accounts but does not sign out user (maintains session)