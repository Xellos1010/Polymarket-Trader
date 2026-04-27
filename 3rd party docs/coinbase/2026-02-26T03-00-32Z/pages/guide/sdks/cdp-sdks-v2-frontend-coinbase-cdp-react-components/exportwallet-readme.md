# exportwallet readme

The `ExportWallet` component provides a secure interface for exporting private keys from embedded wallets. It supports both EVM (externally owned accounts) and Solana accounts, while properly handling edge cases like EVM smart accounts which cannot have their private keys exported. The core features include:

-   Support for EVM (EOA) and Solana account types
-   Secure private key copying via isolated iframe
-   Address display with copy functionality
-   Smart account detection with appropriate warnings
-   Composable UI that gives developers full control over layout and styling
-   Lifecycle callbacks for monitoring iframe state and copy operations

## Architecture

The component is built using a composition pattern that allows for maximum flexibility while providing sensible defaults when customization is not needed.

### Composition model

The `ExportWallet` component is composed of several subcomponents that work together to create the complete key export experience. This approach allows developers to customize the UI structure while maintaining the underlying security and functionality. The main components are:

-   `ExportWallet`: The root wrapper component that provides the `ExportWalletContext` and manages state
-   `ExportWalletTitle`: Renders the title for the export interface
-   `ExportWalletWarning`: Displays appropriate warning messages based on account type
-   `ExportWalletCopyAddress`: Shows the wallet address with copy functionality
-   `ExportWalletCopyKeyButton`: Renders the secure button for copying the private key
-   `ExportWalletFooter`: The “Secured by Coinbase” footer

### ExportWallet

The `ExportWallet` component accepts a required `address` prop and automatically detects the account type (EVM EOA, EVM smart, or Solana) by checking the current user’s accounts. It also provides callback props for monitoring the key export process. **Basic usage:**

```
function ExportWalletPage() {
  const { evmAddress } = useEvmAddress();
  return (
    <ExportWallet 
      address={evmAddress}
      onCopySuccess={() => console.log('Key copied!')}
      onIframeError={(error) => console.error('Error:', error)}
    />
  );
}

```

### ExportWalletTitle

The `ExportWalletTitle` component displays a title for the export interface. By default, it shows “Export your wallet” but can be customized by providing children. It accepts an `as` prop to render as different HTML elements (default is `h2`).

### ExportWalletWarning

The `ExportWalletWarning` component automatically displays the appropriate warning message based on the account type:

-   For EVM EOA and Solana accounts: “Do not share your private key with anyone”
-   For EVM smart accounts: “Cannot export a smart account’s private key; only the owner’s private key can be exported.”

### ExportWalletCopyAddress

The `ExportWalletCopyAddress` component displays the wallet address with a copy button. It uses the `address` from context and accepts a `label` prop (defaults to “Your wallet address”).

### ExportWalletCopyKeyButton

The `ExportWalletCopyKeyButton` component renders a secure button that uses an isolated iframe to handle private key operations. The button is rendered differently based on the account type:

-   For EVM EOA accounts: Uses `CopyEvmKeyButton`
-   For Solana accounts: Uses `CopySolanaKeyButton`
-   For EVM smart accounts: Returns `null` (no button is rendered)

The iframe-based approach ensures that private keys are never exposed to the main application context, maintaining maximum security.

The `ExportWalletFooter` component displays the standard Coinbase footer. Its visibility is controlled by the `showCoinbaseFooter` setting in the app config.

## Example: Basic usage with EVM address

**Export an EVM account’s private key:**

```
import { useEvmAddress } from '@coinbase/cdp-hooks';
import { ExportWallet } from '@coinbase/cdp-react';
function ExportEvmWallet() {
  const { evmAddress } = useEvmAddress();
  
  return (
    <ExportWallet address={evmAddress} />
  );
}

```

## Example: Basic usage with Solana address

**Export a Solana account’s private key:**

```
import { useSolanaAddress } from '@coinbase/cdp-hooks';
import { ExportWallet } from '@coinbase/cdp-react';
function ExportSolanaWallet() {
  const { solanaAddress } = useSolanaAddress();
  
  return (
    <ExportWallet 
      address={solanaAddress}
      onCopySuccess={() => {
        console.log('Solana private key copied');
      }}
    />
  );
}

```

## Example: Export with callbacks

**Monitor the export process with lifecycle callbacks:**

```
import { ExportWallet } from '@coinbase/cdp-react';
import { useEvmAddress } from '@coinbase/cdp-hooks';
function MonitoredExportWallet() {
  const { evmAddress } = useEvmAddress();
  
  const handleIframeReady = () => {
    console.log('Secure iframe is ready');
  };
  
  const handleCopySuccess = () => {
    console.log('Private key copied successfully');
  };
  
  const handleIframeError = (error?: string) => {
    console.error('Iframe error:', error);
  };
  
  return (
    <ExportWallet 
      address={evmAddress}
      onIframeReady={handleIframeReady}
      onCopySuccess={handleCopySuccess}
      onIframeError={handleIframeError}
    />
  );
}

```

## Example: Custom layout

**Customize the layout and styling:**

```
import {
  ExportWallet,
  ExportWalletTitle,
  ExportWalletWarning,
  ExportWalletCopyAddress,
  ExportWalletCopyKeyButton,
  ExportWalletFooter,
} from '@coinbase/cdp-react';
import { useEvmAddress } from '@coinbase/cdp-hooks';
function CustomExportWallet() {
  const { evmAddress } = useEvmAddress();
  
  return (
    <ExportWallet address={evmAddress}>
      <div className="custom-header">
        <ExportWalletTitle as="h1">
          Backup Your Wallet
        </ExportWalletTitle>
        <p className="subtitle">
          Save your private key in a secure location
        </p>
      </div>
      
      <div className="warning-section">
        <ExportWalletWarning />
      </div>
      
      <div className="export-details">
        <ExportWalletCopyAddress label="Wallet Address" />
        <ExportWalletCopyKeyButton />
      </div>
      
      <div className="help-text">
        <p>
          Your private key gives full control of your wallet.
          Store it safely and never share it with anyone.
        </p>
      </div>
      
      <ExportWalletFooter />
    </ExportWallet>
  );
}

```

## Example: Custom layout with session expiration handling

**Customize the layout based on session expiration state:**

```
import {
  ExportWallet,
  ExportWalletTitle,
  ExportWalletWarning,
  ExportWalletCopyAddress,
  ExportWalletCopyKeyButton,
} from '@coinbase/cdp-react';
import { useEvmAddress } from '@coinbase/cdp-hooks';
function CustomExportWalletWithExpiration() {
  const { evmAddress } = useEvmAddress();
  
  return (
    <ExportWallet address={evmAddress}>
      {({ isSessionExpired }) => (
        <>
          <ExportWalletTitle />
          {isSessionExpired ? (
             <div className="error-banner">
               Session expired. Please refresh the page.
             </div>
          ) : (
            <>
              <ExportWalletWarning />
              <ExportWalletCopyAddress />
              <ExportWalletCopyKeyButton />
            </>
          )}
        </>
      )}
    </ExportWallet>
  );
}

```

## API Reference

### Required Props

-   `address`: The wallet address to export. The component automatically detects whether this is an EVM EOA, EVM smart account, or Solana account.

### Optional Props

-   `onIframeReady`: Callback function invoked when the secure iframe is ready
-   `onCopySuccess`: Callback function invoked when the private key is successfully copied
-   `onIframeError`: Callback function invoked when an error occurs in the secure iframe
-   `onIframeSessionExpired`: Callback function invoked when the iframe session has expired
-   `children`: React nodes for custom layout, or a function that receives an object with `type` and `isSessionExpired` properties (if not provided, uses default layout)
-   `className`: Additional CSS classes to apply to the root element

## Security Considerations

-   **Isolated iframe**: Private key operations are handled in an isolated iframe to prevent exposure to the main application context
-   **Smart account protection**: The component automatically detects EVM smart accounts and prevents private key export, displaying an appropriate warning instead
-   **No key exposure**: Private keys are never exposed to JavaScript; they are only copied to the clipboard through the secure iframe
-   **User warning**: Clear warnings are displayed to remind users not to share their private keys

## Notes

-   **EVM smart accounts**: Smart accounts use account abstraction and do not have a private key that can be exported. The component will show a warning and hide the copy key button for these accounts. Only the EOA owner’s private key can be exported.
-   **Security best practices**: Always educate users about the importance of keeping their private keys secure and never sharing them