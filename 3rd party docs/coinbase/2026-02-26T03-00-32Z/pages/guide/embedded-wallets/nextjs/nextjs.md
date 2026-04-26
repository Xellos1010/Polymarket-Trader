# nextjs

## Overview

Build a Next.js app with CDP embedded wallets using our dedicated Next.js template. This guide shows you how to create a server-side rendered app with CDP’s client-side wallet functionality.

## Prerequisites

-   A free [CDP Portal](https://portal.cdp.coinbase.com/) account and project
-   [Node.js 22+](https://nodejs.org/en/download)
-   A node package manager installed (i.e., `npm`, `pnpm`, or `yarn`)
-   Basic familiarity with Next.js and React

## 1\. Create a Next.js app with CDP

## 2\. Configure

Follow the prompts to configure your app. When prompted, select **Next.js** as your template and enter your CDP Project ID:

```
✔ Project name: … cdp-app-nextjs
✔ Template: › Next.js Full Stack App
✔ CDP Project ID: … 8c21e60b-c8af-4286-a0d3-111111111111
✔ Account Type: › EVM EOA (Regular Accounts)
✔ Confirm you have whitelisted 'http://localhost:3000' … y

```

## 3\. Run

Navigate to your project and start the development server:

Your app will be available at [http://localhost:3000](http://localhost:3000/).

## How it works

The Next.js integration uses a clean separation between server and client components to leverage Next.js’s server-side rendering while maintaining CDP’s client-side functionality.

### Provider pattern

The template follows Next.js best practices by:

1.  Keeping the root layout as a server component (no CDP imports or hooks)
2.  Making the page component a client component that wraps the app with providers
3.  Containing all CDP functionality within client components

```
src/
├── app/
│   ├── layout.tsx              # Server component - metadata and basic HTML
│   ├── page.tsx                # Client component - wraps app with Providers
│   └── globals.css             # Global styles and CSS variables
└── components/
    ├── Providers.tsx           # CDPReactProvider setup with theme
    ├── ClientApp.tsx           # Main app logic with auth flow
    ├── SignInScreen.tsx        # Sign-in UI with AuthButton
    ├── SignedInScreen.tsx      # Post-auth UI with wallet features
    ├── Transaction.tsx         # Transaction sending component
    ├── UserBalance.tsx         # Balance display component
    ├── Header.tsx              # Header with wallet address
    └── theme.ts                # Theme customization using CSS variables

```

### Component architecture

#### Server component (root layout)

The layout remains a pure server component for metadata and basic HTML structure:

```
import type { Metadata } from "next";
import "./globals.css";
export const metadata: Metadata = {
  title: "CDP Embedded Wallet Demo",
  description: "A demo of the CDP Embedded Wallet",
};
export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}

```

#### Client component (page)

The page component is marked as a client component and handles provider setup:

```
"use client";
import ClientApp from "@/components/ClientApp";
import Providers from "@/components/Providers";
export default function Home() {
  return (
    <Providers>
      <ClientApp />
    </Providers>
  );
}

```

#### Themed providers

The Providers component wraps the app with CDP context and includes theme customization:

src/components/Providers.tsx

```
"use client";
import { CDPReactProvider } from "@coinbase/cdp-react/components/CDPReactProvider";
import { theme } from "@/components/theme";
import { CDP_CONFIG } from "./config.ts";
interface ProvidersProps {
  children: React.ReactNode;
}
const config = {
  ...CDP_CONFIG,
  appName: "CDP Next.js StarterKit",
  appLogoUrl: "http://localhost:3000/logo.svg",
};
export default function Providers({ children }: ProvidersProps) {
  return (
    <CDPReactProvider config={CDP_CONFIG} theme={theme}>
      {children}
    </CDPReactProvider>
  );
}

```

The theme uses CSS variables for easy customization:

```
import { type Theme } from "@coinbase/cdp-react/theme";
export const theme: Partial<Theme> = {
  "colors-background": "var(--cdp-example-card-bg-color)",
  "colors-text": "var(--cdp-example-text-color)",
  "colors-primary": "var(--cdp-example-accent-color)",
  // ... more theme overrides
};

```

### Authentication flow

The template implements a complete authentication flow:

1.  **ClientApp** checks initialization and authentication status
2.  **SignInScreen** displays the AuthButton for sign-in
3.  **SignedInScreen** shows wallet features after authentication

The template also uses [viem](https://viem.sh/) for reading blockchain data (like checking balances), while CDP handles all wallet operations and transaction signing.

### Example components

#### Sign-in screen

src/components/SignInScreen.tsx

```
"use client";
import { AuthButton } from "@coinbase/cdp-react/components/AuthButton";
export default function SignInScreen() {
  return (
    <main className="card card--login">
      <h1 className="sr-only">Sign in</h1>
      <p className="card-title">Welcome!</p>
      <p>Please sign in to continue.</p>
      <AuthButton />
    </main>
  );
}

```

#### Transaction component

The template includes a full-featured transaction component that demonstrates sending ETH on Base Sepolia:

src/components/Transaction.tsx

```
"use client";
import { useSendEvmTransaction, useEvmAddress } from "@coinbase/cdp-hooks";
import { Button } from "@coinbase/cdp-react/components/Button";
import { LoadingSkeleton } from "@coinbase/cdp-react/components/LoadingSkeleton";
import { type MouseEvent, useCallback, useMemo, useState } from "react";
interface Props {
  balance?: string;
  onSuccess?: () => void;
}
export default function Transaction(props: Props) {
  const { balance, onSuccess } = props;
  const { sendEvmTransaction } = useSendEvmTransaction();
  const { evmAddress } = useEvmAddress();
  const [isPending, setIsPending] = useState(false);
  const [transactionHash, setTransactionHash] = useState<string | null>(null);
  const handleSendTransaction = useCallback(
    async (e: MouseEvent<HTMLButtonElement>) => {
      if (!evmAddress) return;
      e.preventDefault();
      setIsPending(true);
      const { transactionHash } = await sendEvmTransaction({
        transaction: {
          to: evmAddress,              // Send to yourself for testing
          value: 1000000000000n,       // 0.000001 ETH in wei
          gas: 21000n,                 // Standard ETH transfer gas limit
          chainId: 84532,              // Base Sepolia testnet
          type: "eip1559",             // Modern transaction type
        },
        evmAccount: evmAddress,
        network: "base-sepolia",
      });
      setTransactionHash(transactionHash);
      setIsPending(false);
      onSuccess?.();
    },
    [evmAddress, sendEvmTransaction, onSuccess],
  );
  // Component renders transaction button or success state
  return (
    <Button onClick={handleSendTransaction} isPending={isPending}>
      Send Transaction
    </Button>
  );
}

```

Key transaction details:

-   **Base Sepolia**: A test network where you can experiment with fake ETH
-   **Gas**: The computational fee (21000 is standard for simple ETH transfers)
-   **Wei**: The smallest unit of ETH (1 ETH = 10^18 wei)
-   **EIP-1559**: Modern transaction format with predictable gas fees

## Troubleshooting

Common errors without 'use client'

If you forget the `"use client"` directive in components that use CDP functionality, you’ll see errors like:

-   `TypeError: createContext is not a function`
-   `Error: useContext must be used within a Provider`
-   `window is not defined` or `document is not defined`
-   `ReferenceError: localStorage is not defined`

**Solution**: Add `"use client"` as the first line in any component file that uses CDP hooks or components.

## What to read next

-   [**Quickstart**](https://developer.chrome.com/embedded-wallets/quickstart): Full walkthrough of building your first CDP app with detailed explanations
-   [**CDP React Hooks**](https://developer.chrome.com/embedded-wallets/react-hooks): Available hooks like `useSignInWithEmail`, `useEvmAddress`, and `useSendEvmTransaction`
-   [**CDP React Components**](https://developer.chrome.com/embedded-wallets/react-components): Pre-built UI components for authentication and wallet management
-   [**CDP Web SDK Reference**](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend): Comprehensive API documentation