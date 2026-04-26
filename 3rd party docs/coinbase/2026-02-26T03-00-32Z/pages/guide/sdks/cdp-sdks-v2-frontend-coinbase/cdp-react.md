# cdp react

This package contains ready-made, customizable React UI components that wrap the CDP frontend SDK.

## Components

-   [SignInModal](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/SignInModal) - a button that triggers a modal with the sign-in flow
-   [SignIn](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/SignIn) - the forms for the sign in flow, a logo, heading, and description text
-   [SignOutButton](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/SignOutButton) - a sign out button
-   [AuthButton](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/AuthButton) - the `SignOutButton` when logged in, and the `SignInModal` when logged out
-   [SendEvmTransactionButton](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/SendEvmTransactionButton) - a button that signs and sends an EVM transaction
-   [SendSolanaTransactionButton](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/SendSolanaTransactionButton) - a button that signs and sends a Solana transaction
-   [Fund](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/Fund) - the fund flow
-   [FundModal](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/FundModal) - a button that triggers a modal with the fund flow
-   [LinkAuth](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/LinkAuth) - a component to link or unlink auth methods
-   [LinkAuthModal](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/LinkAuthModal) - a button that triggers a modal with the link / unlink flow
-   [CopyAddress](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/CopyAddress) - a component that renders a truncated address with a copy button
-   [CopyEvmKeyButton](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/CopyEvmKeyButton) - a button for copying the private key of a user’s EVM EOA address
-   [CopySolanaKeyButton](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/CopySolanaKeyButton) - a button for copying the private key of a user’s Solana address
-   [ExportWallet](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/ExportWallet) - a component for exporting a user’s wallet; includes a title, a warning about keeping private keys secure, the CopyAddress component, and either the CopyEvmKeyButton or CopySolanaKeyButton depending on the type of address provided
-   [ExportWalletModal](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/ExportWalletModal) - a button that triggers a modal with the ExportWallet component
-   [EnrollMfa](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/EnrollMfa) - a component for MFA enrollment; guides users through setting up two-factor authentication via TOTP
-   [EnrollMfaModal](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/EnrollMfaModal) - a button that triggers a modal with the MFA enrollment flow
-   [VerifyMfa](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/VerifyMfa) - a component for MFA verification; prompts enrolled users to enter their authenticator code
-   [VerifyMfaModal](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/VerifyMfaModal) - a button that triggers a modal with the MFA verification flow
-   [VerifyMfaInline](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/VerifyMfaInline) - a wrapper component for embedding MFA verification within existing UI flows

## Quickstart

This guide will help you get started with `@coinbase/cdp-react`. You’ll learn how to install the package, set up the provider, and render your first component.

### Installation

First, add the package to your project using your preferred package manager.

```
# With pnpm
pnpm add @coinbase/cdp-react @coinbase/cdp-core @coinbase/cdp-hooks
# With yarn
yarn add @coinbase/cdp-react @coinbase/cdp-core @coinbase/cdp-hooks
# With npm
npm install @coinbase/cdp-react @coinbase/cdp-core @coinbase/cdp-hooks

```

### Gather your CDP Project information

1.  Sign in or create an account on the [CDP Portal](https://portal.cdp.coinbase.com/)
2.  On your dashboard, select a project from the dropdown at the at the top, and copy the Project ID

### Allowlist your local app

1.  Navigate to the [Embedded Wallet Configuration](https://portal.cdp.coinbase.com/products/embedded-wallets/cors) in CDP Portal, and click Add origin to include your local app
2.  Enter the origin of your locally running app - e.g., `http://localhost:3000`
3.  Click Add origin again to save your changes

### Setup Provider

Next, you need to wrap your application with the `CDPReactProvider`. `CDPReactProvider` provides the necessary context for hooks and components to work correctly. It also provides access to config data and theming. Update your main application file (e.g., `main.tsx` or `App.tsx`) to include the provider:

```
import React from 'react';
import ReactDOM from 'react-dom/client';
import { App } from './App'; // Your main App component
import { CDPReactProvider, type Config, type Theme } from '@coinbase/cdp-react';
// Config for your dapp
const config: Config = {
  projectId: "your-project-id", // Copy your Project ID here.
  appName: "My app", // the name of your application
  appLogoUrl: "https://picsum.photos/64", // logo will be displayed in select components
}
// You can customize the theme by overriding theme variables
const themeOverrides: Partial<Theme> = {
  "colors-bg-default": "black",
  "colors-bg-alternate": "#121212",
  "colors-fg-default": "white",
  "colors-fg-muted": "#999999",
}
ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <CDPReactProvider config={config} theme={themeOverrides}>
      <App />
    </CDPReactProvider>
  </React.StrictMode>,
);

```

#### Analytics Opt-Out

By default the SDK will emit usage analytics to help us improve the SDK. If you would like to opt-out, you can do so by setting the `disableAnalytics` configuration option to `true`.

```
<CDPReactProvider
  config={{
    projectId: "your-project-id",
    disableAnalytics: true,
  }}
>
  <App />
</CDPReactProvider>

```

### Render a Component

Now you can use the components from the library. Let’s add the `AuthButton` component to your application. This component handles both sign-in and sign-out functionality.

```
// In your App.tsx or any other component
import { AuthButton } from '@coinbase/cdp-react/components/AuthButton';
function App() {
  return (
    <div>
      <h1>Welcome</h1>
      <AuthButton />
    </div>
  );
}
export default App;

```

That’s it! You’ve successfully installed `@coinbase/cdp-react`, set up the provider, and rendered your first component.