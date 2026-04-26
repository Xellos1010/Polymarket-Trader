# security export

## Overview

While Embedded Wallets are designed to eliminate the complexity of private key management for users, there are scenarios where developers may need to export private keys for wallet migration, user preference, or other legitimate use cases. This page covers the security considerations, implementation, and best practices for handling private key exports.

## Prerequisites

-   A [CDP Portal](https://portal.cdp.coinbase.com/) account and project
-   Embedded Wallets enabled in your project with [configured domains](https://developer.chrome.com/embedded-wallets/domains)
-   [`@coinbase/cdp-react`](https://www.npmjs.com/package/@coinbase/cdp-react) installed and configured
-   User successfully authenticated with embedded wallet

## When to consider

Private key export should only be considered in specific scenarios:

## Best practices

1.  **Never log or store private keys in plaintext**
    -   Avoid console.log, file logging, or unencrypted persistent storage
    -   Clear private key variables from memory when done
    -   Let the browser’s garbage collector handle cleanup
2.  **Require explicit user consent**
    -   Don’t make key export automatic or hidden
    -   Show clear security warnings before export
    -   Make the export process deliberate, not accidental
3.  **Use secure UI patterns**
    -   Clipboard copy is safer than displaying keys on screen
    -   Provide clear instructions for secure handling
    -   Consider offering alternatives like asset transfer when appropriate
4.  **Educate users on security**
    -   Explain what private keys are and why they’re sensitive
    -   Provide guidance on secure storage options
    -   Link to general wallet security resources

### For users

For comprehensive guidance on private key security and storage best practices, refer to [Coinbase’s guide on private key security](https://www.coinbase.com/learn/crypto-basics/what-is-a-private-key).

## Export scenarios

Here are common scenarios where users might need to export their private keys from embedded wallets. Each scenario should be implemented with the security measures outlined above.

## Implementation

The SDK provides a secure, built-in UI component for wallet export that handles all security warnings, confirmations, and private key management automatically. This is the recommended approach as it ensures consistent security practices across all implementations.

### Why use the secure export component

The `ExportWalletModal` component provides critical security features that protect both you and your users:

### Using the secure export component

-   EVM
    
-   Solana
    

```
import { useEvmAddress, ExportWalletModal } from "@coinbase/cdp-react";
const Export = () => {
  const { evmAddress } = useEvmAddress();
  if (!evmAddress) return null;
  return (
    <ExportWalletModal address={evmAddress} />
  );
};

```

```
import { useSolanaAddress, ExportWalletModal } from "@coinbase/cdp-react";
const Export = () => {
  const { solanaAddress } = useSolanaAddress();
  if (!solanaAddress) return null;
  return (
    <ExportWalletModal address={solanaAddress} />
  );
};

```

The `ExportWalletModal` component:

-   Displays appropriate security warnings and confirmations
-   Securely handles the private key export flow
-   Provides a safe UI for the end user to copy the private key
-   Works with both EVM and Solana addresses

The `address` prop is required and accepts either an EVM address or Solana address.

## What to read next

-   **[React Hooks](https://developer.chrome.com/embedded-wallets/react-hooks)** - Learn about all available hooks for embedded wallet operations
-   **[CDP SDK Documentation](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend)** - Complete SDK reference and API documentation
-   **[Smart Accounts](https://developer.chrome.com/embedded-wallets/smart-accounts)** - Explore account abstraction as an alternative to private key management
-   **[Authentication Methods](https://developer.chrome.com/embedded-wallets/authentication-methods)**: Learn about available authentication options
-   **[Best Practices](https://developer.chrome.com/embedded-wallets/best-practices)**: Security recommendations and production readiness