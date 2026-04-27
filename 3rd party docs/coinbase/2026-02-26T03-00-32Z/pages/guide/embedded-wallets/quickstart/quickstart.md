# quickstart

EVMSolana

## Overview

This guide demonstrates how to add embedded wallets to your existing React app with just a few lines of code.

What is an embedded wallet?

An **embedded wallet** is a self-custodial crypto wallet built directly into your app. Unlike traditional wallets (like MetaMask) that require browser extensions and seed phrases, embedded wallets let users sign in with familiar auth methods such as email, mobile SMS, and OAuth while maintaining full control of their assets.Key benefits:

-   **No downloads**: Works instantly in any browser
-   **Email sign-in**: No seed phrases to manage, but users retain full control
-   **You control the UX**: Seamlessly integrated into your app

**Choose your path:**

## Prerequisites

-   A free [CDP Portal](https://portal.cdp.coinbase.com/) account and project
-   [Node.js 22+](https://nodejs.org/en/download)
-   A node package manager installed (i.e., `npm`, `pnpm`, or `yarn`)
-   Basic familiarity with React and TypeScript
-   Configured your domain in CDP Portal (see below)

How to configure your domain in CDP Portal

**Step 1: Access CDP Portal**Navigate to the [Domains Configuration](https://portal.cdp.coinbase.com/products/embedded-wallets/domains) in CDP Portal, and click **Add domain** to include your local app.

**Step 2: Add your domain**

-   For local development: Use `http://localhost:3000` (or your preferred port)
-   For production: Use your actual domain (e.g., `https://yourapp.com`)

**Step 3: Save your changes**Click **Add domain** again to save your changes.

You should see your domain listed in the CDP Portal dashboard. The allowlist will take effect immediately upon saving.

## 1\. Install packages

Once you’ve completed the prerequisites above, install the required packages:

## 2\. Wrap your app with the provider

Add the CDP provider to your root component (typically `App.tsx` or `main.tsx`). Replace `"your-project-id"` with your actual project ID from [CDP Portal](https://portal.cdp.coinbase.com/).

```
import { CDPReactProvider } from "@coinbase/cdp-react";
function App() {
  return (
    <CDPReactProvider
      config={{
        projectId: "your-project-id",
        ethereum: { // if you want to create an EVM account on login
          createOnLogin: "eoa" // or "smart" for smart accounts
        },
        solana: { // if you want to create a Solana account on login
          createOnLogin: true
        },
        appName: "Your App Name"
      }}
    >
      <YourExistingApp />
    </CDPReactProvider>
  );
}

```

## 3\. Add authentication

### Option A: Use the AuthButton (recommended)

The simplest approach is to use the `AuthButton` component which handles the entire authentication flow:

```
import { AuthButton } from "@coinbase/cdp-react/components/AuthButton";
import { useIsSignedIn } from "@coinbase/cdp-hooks";
function AuthComponent() {
  const { isSignedIn } = useIsSignedIn();
  return (
    <div>
      {isSignedIn ? (
        <div>Welcome! You're signed in.</div>
      ) : (
        <div>
          <h2>Please sign in</h2>
          <AuthButton />
        </div>
      )}
    </div>
  );
}

```

### Option B: Build custom auth UI

For custom UIs, use the authentication hooks directly:

```
function CustomAuthComponent() {
    const { signInWithEmail } = useSignInWithEmail();
    const { verifyEmailOTP } = useVerifyEmailOTP();
    const { isSignedIn } = useIsSignedIn();
    const [flowId, setFlowId] = useState<string | null>(null);
    const [email, setEmail] = useState('');
    const [otp, setOtp] = useState('');
    const handleEmailSubmit = async () => {
        if (!email) return;
        try {
            const result = await signInWithEmail({ email });
            setFlowId(result.flowId);
        } catch (error) {
            console.error("Sign in failed:", error);
        }
    };
    const handleOtpSubmit = async () => {
        if (!flowId || !otp) return;
        try {
            const { user } = await verifyEmailOTP({ flowId, otp });
            console.log("Signed in!", user.evmAccounts?.[0]);
        } catch (error) {
            console.error("OTP verification failed:", error);
        }
    };
    if (isSignedIn) {
        return <div>Welcome! You're signed in.</div>;
    }
    return (
        <div>
            {flowId ? (
                <div>
                    <h2>Enter OTP</h2>
                    <input type="text" value={otp} onChange={(e) => setOtp(e.target.value)} placeholder="Enter OTP code" />
                    <button onClick={handleOtpSubmit}>Verify OTP</button>
                </div>
            ) : (
              <div>
                <h2>Sign in with Email</h2>
                <input type="email" value={email} onChange={(e) => setEmail(e.target.value)} placeholder="Enter your email" />
                <button onClick={handleEmailSubmit}>Send OTP</button>
              </div>
            )}
        </div>
    );
}

```

## 4\. Send transactions

### EVM transactions

Once authenticated, users automatically get a wallet address. Here’s how to send EVM transactions:

```
import { useEvmAddress } from "@coinbase/cdp-hooks";
import { SendEvmTransactionButton } from "@coinbase/cdp-react";
function SendTransaction(){
    const { evmAddress } = useEvmAddress();
    return (
        <div>
            <div>
                <h2>Send Transaction</h2>
                {evmAddress ? (
                  <SendEvmTransactionButton
                    account={evmAddress}
                    network="base-sepolia"
                    transaction={{
                        to: evmAddress,
                        value: 1000000000000n,
                        chainId: 84532,
                        type: "eip1559",
                    }}
                    onSuccess={(hash) => {
                        console.log('Transaction successful:', hash);
                        alert(`Transaction sent! Hash: ${hash}`);
                    }}
                    onError={(error) => {
                        console.error('Transaction failed:', error);
                        alert(`Transaction failed: ${error.message}`);
                    }}
                    pendingLabel="Sending transaction..."
                  />
                ) : (
                    <p>Wallet not ready yet...</p>
                )}
            </div>
        </div>
    );
}

```

That’s it! Your users now have embedded wallets and can send transactions.

### Solana transactions

Here’s how to send Solana transactions:

```
import { useSolanaAddress } from "@coinbase/cdp-hooks";
import { SendSolanaTransactionButton } from "@coinbase/cdp-react";
function SendTransaction(){
    const { solanaAddress } = useSolanaAddress();
    return (
        <div>
            <div>
                <h2>Send Transaction</h2>
                {solanaAddress ? (
                  <SendSolanaTransactionButton
                    account={solanaAddress}
                    network="solana-devnet"
                    transaction="base64-solana-transaction"
                    pendingLabel="Sending transaction..."
                  />
                ) : (
                    <p>Wallet not ready yet...</p>
                )}
            </div>
        </div>
    );
}

```

## What to read next