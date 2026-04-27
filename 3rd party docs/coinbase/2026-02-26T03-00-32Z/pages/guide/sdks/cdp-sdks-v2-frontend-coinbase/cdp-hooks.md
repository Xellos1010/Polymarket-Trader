# cdp hooks

This package provides React hooks for conveniently accessing embedded wallet functionality. Built on top of `@coinbase/cdp-core`, it offers a React-friendly interface for end user authentication and embedded wallet operations.

## Quickstart

This guide will help you get started with @coinbase/cdp-hooks. You’ll learn how to install the package, set up the provider, and use the hooks in both web and React Native applications.

### Installation

#### Web Applications

For web applications, add the package to your project using your preferred package manager:

```
# With npm
npm install @coinbase/cdp-core @coinbase/cdp-hooks
# With pnpm
pnpm add @coinbase/cdp-core @coinbase/cdp-hooks
# With yarn
yarn add @coinbase/cdp-core @coinbase/cdp-hooks

```

#### React Native Applications

For React Native applications, you’ll need additional crypto polyfills and dependencies:

```
# Core packages
npm install @coinbase/cdp-core @coinbase/cdp-hooks
# Install this polyfill with expo for better compatibility
npx expo install react-native-quick-crypto
# Required crypto polyfills for React Native
npm install react-native-get-random-values @ungap/structured-clone
# AsyncStorage for React Native storage
npm install @react-native-async-storage/async-storage

```

**React Native Setup Code** You’ll need to initialize the crypto polyfills before importing your app. Create or update your entry point file (typically `index.js` or `index.ts`):

```
import structuredClone from "@ungap/structured-clone";
import { install } from "react-native-quick-crypto";
import "react-native-get-random-values";
// Install crypto polyfills
if (!("structuredClone" in globalThis)) {
  globalThis.structuredClone = structuredClone as any;
}
install(); // Install react-native-quick-crypto
// Import your app after polyfills are installed
import App from "./App";
// Register your app component
import { registerRootComponent } from "expo"; // For Expo apps
registerRootComponent(App);

```

**Why these dependencies?**

-   `react-native-quick-crypto`: Provides Web Crypto API compatibility for asymmetric key generation (ECDSA, RSA) required for JWT signing and encryption
-   `react-native-get-random-values`: Provides secure random number generation via `crypto.getRandomValues()`
-   `@ungap/structured-clone`: Polyfills `structuredClone` for object cloning compatibility
-   `@react-native-async-storage/async-storage`: Provides persistent storage for auth tokens and secrets

### Gather your CDP Project Information

1.  Sign in or create an account on the [CDP Portal](https://portal.cdp.coinbase.com/)
2.  On your dashboard, select a project from the dropdown at the at the top, and copy the Project ID

### Allowlist your local app

1.  Navigate to the [Embedded Wallet Configuration](https://portal.cdp.coinbase.com/products/embedded-wallets/cors) in CDP Portal, and click Add origin to include your local app
2.  Enter the origin of your locally running app - e.g., `http://localhost:3000`
3.  Click Add origin again to save your changes

### Setup Provider

Next, you need to wrap your application with the CDPHooksProvider, which provides the necessary context for hooks to work correctly.

#### Web Applications

Update your main application file (e.g., main.tsx) to include the provider:

```
import React from "react";
import { CDPHooksProvider } from "@coinbase/cdp-hooks";
import { App } from './App'; // Your main App component
function App() {
  return (
    <CDPHooksProvider 
      config={{
        // Copy and paste your project ID here.
        projectId: "your-project-id",
      }}
    >
      <App />
    </CDPHooksProvider>
  );
}

```

#### React Native Applications

For React Native, the setup is identical.

```
import React from "react";
import { CDPHooksProvider } from "@coinbase/cdp-hooks";
import { App } from "./App";
export default function App() {
  return (
    <CDPHooksProvider config={{
      projectId: "your-project-id",
    }}>
      <App />
    </CDPHooksProvider>
  );
}

```

#### Analytics Opt-Out

By default the SDK will emit usage analytics to help us improve the SDK. If you would like to opt-out, you can do so by setting the `disableAnalytics` configuration option to `true`.

```
function App() {
  return (
    <CDPHooksProvider config={{
      projectId: "your-project-id",
      disableAnalytics: true,
    }}>
      <App />
    </CDPHooksProvider>
  );
}

```

#### Smart Account Configuration

You can configure the provider to automatically create Smart Accounts for new users:

```
function App() {
  return (
    <CDPHooksProvider
      config={{
        projectId: "your-project-id",
        ethereum: {
          createOnLogin: "smart", // Creates Smart Accounts instead of EOAs
        },
      }}
    >
      <App />
    </CDPHooksProvider>
  );
}

```

-   When `ethereum.createOnLogin` is set to `"smart"`, new users will automatically get both an EOA and a Smart Account.

#### Solana Configuration

You can configure the provider to create Solana accounts for new users:

```
function App() {
  return (
    <CDPHooksProvider
      config={{
        projectId: "your-project-id",
        solana: {
          createOnLogin: true, // Creates Solana accounts
        },
      }}
    >
      <App />
    </CDPHooksProvider>
  );
}

```

-   When `solana.createOnLogin` is set to `true`, new users will automatically get a Solana account instead of EVM accounts.

#### Deferred Account Creation

You can omit `createOnLogin` entirely to prevent automatic account creation and instead create accounts manually when needed:

```
function App() {
  return (
    <CDPHooksProvider
      config={{
        projectId: "your-project-id",
        // No ethereum or solana createOnLogin configuration
      }}
    >
      <App />
    </CDPHooksProvider>
  );
}

```

When `createOnLogin` is omitted, the SDK will:

1.  Not create any accounts automatically upon user login
2.  Require manual account creation using the account creation hooks (see below)
3.  Give you full control over when and what types of accounts to create

### Sign In a User

End user authentication proceeds in two steps:

1.  The user inputs their email address to initiate the authentication flow, which will send the user a One Time Password (OTP) and return a `flowId`
2.  The user submits the six-digit OTP and `flowId`, after which the user will be authenticated, returning a User object.

#### Web Applications

```
import { useSignInWithEmail, useVerifyEmailOTP } from "@coinbase/cdp-hooks";
function SignIn() {
  const { signInWithEmail } = useSignInWithEmail();
  const { verifyEmailOTP } = useVerifyEmailOTP();
  const handleSignIn = async (email: string) => {
    try {
      // Start sign in flow
      const { flowId } = await signInWithEmail({ email });
      // In a real application, you would prompt the user for the OTP they received
      // in their email. Here, we hardcode it for convenience.
      const otp = "123456";
      // Complete sign in
      const { user, isNewUser } = await verifyEmailOTP({
        flowId,
        otp
      });
      console.log("Signed in user:", user);
      
      // Access different account types based on configuration
      // Using new account objects (recommended)
      if (user.evmAccountObjects?.length > 0) {
        user.evmAccountObjects.forEach((account, index) => {
          console.log(`EVM Account ${index + 1}:`, account.address);
          console.log(`Created:`, new Date(account.createdAt).toLocaleDateString());
        });
      }
      if (user.evmSmartAccountObjects?.length > 0) {
        user.evmSmartAccountObjects.forEach((account, index) => {
          console.log(`Smart Account ${index + 1}:`, account.address);
          console.log(`Owners:`, account.ownerAddresses.join(', '));
        });
      }
      if (user.solanaAccountObjects?.length > 0) {
        user.solanaAccountObjects.forEach((account, index) => {
          console.log(`Solana Account ${index + 1}:`, account.address);
        });
      }
      // Legacy deprecated arrays (for backward compatibility)
      // ⚠️ Use *AccountObjects instead
      if (user.evmAccounts?.length > 0) {
        console.log("User EVM address (EOA):", user.evmAccounts[0]);
      }
      if (user.evmSmartAccounts?.length > 0) {
        console.log("User Smart Account:", user.evmSmartAccounts[0]);
      }
      if (user.solanaAccounts?.length > 0) {
        console.log("User Solana address:", user.solanaAccounts[0]);
      }
    } catch (error) {
      console.error("Sign in failed:", error);
    }
  };
  return <button onClick={() => handleSignIn("user@example.com")}>Sign In</button>;
}

```

### Working with Multiple Accounts

Users can have up to 10 accounts per blockchain type (EVM, Solana). The SDK provides both single-account and multi-account hooks to support different use cases.

#### Single Account Hooks

These hooks return the **first account (index 0)**. They’re ideal for simple applications where most users have one account:

-   `useEvmAddress()` - Returns first smart account, then first EOA
-   `useSolanaAddress()` - Returns first Solana account

```
function SimpleWallet() {
  const { evmAddress } = useEvmAddress(); // Returns first EVM address
  const { solanaAddress } = useSolanaAddress(); // Returns first Solana address
  return (
    <div>
      <p>EVM: {evmAddress ?? 'No account'}</p>
      <p>Solana: {solanaAddress ?? 'No account'}</p>
    </div>
  );
}

```

#### Multi-Account Hooks (All Accounts with Metadata)

These hooks return **all accounts** with additional metadata (creation timestamp, owner addresses). Use these when:

-   Users may have multiple accounts
-   You need account metadata (creation date, etc.)
-   You’re building an account selector UI

Available hooks:

-   `useEvmAccounts()` - All EVM EOA accounts with metadata
-   `useSolanaAccounts()` - All Solana accounts with metadata
-   `useEvmSmartAccounts()` - All EVM smart accounts with metadata

```
function MultiAccountWallet() {
  const { evmAccounts } = useEvmAccounts();
  const { solanaAccounts } = useSolanaAccounts();
  return (
    <div>
      <h3>EVM Accounts</h3>
      {evmAccounts?.map((account, idx) => (
        <div key={account.address}>
          <p>Account {idx + 1}: {account.address}</p>
          <p>Created: {new Date(account.createdAt).toLocaleDateString()}</p>
        </div>
      ))}
      <h3>Solana Accounts</h3>
      {solanaAccounts?.map((account, idx) => (
        <div key={account.address}>
          <p>Account {idx + 1}: {account.address}</p>
          <p>Created: {new Date(account.createdAt).toLocaleDateString()}</p>
        </div>
      ))}
    </div>
  );
}

```

#### When to Use Which?

Use Case

Hook to Use

Simple wallet display

`useEvmAddress()` / `useSolanaAddress()`

Account selector dropdown

`useEvmAccounts()` / `useSolanaAccounts()`

Display creation dates

`useEvmAccounts()` / `useSolanaAccounts()`

Show owner addresses (smart accounts)

`useEvmSmartAccounts()`

Most users have 1 account

`useEvmAddress()` / `useSolanaAddress()`

Advanced multi-account features

`useEvmAccounts()` / `useSolanaAccounts()`

#### React Native Applications

For React Native, you’ll use native UI components and handle the sign-in flow similarly:

```
import React, { useState } from "react";
import { View, Text, TextInput, TouchableOpacity, Alert } from "react-native";
import { useSignInWithEmail, useVerifyEmailOTP } from "@coinbase/cdp-hooks";
function SignInScreen() {
  const { signInWithEmail } = useSignInWithEmail();
  const { verifyEmailOTP } = useVerifyEmailOTP();
  const [email, setEmail] = useState("");
  const [otp, setOtp] = useState("");
  const [flowId, setFlowId] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const handleSignIn = async () => {
    if (!email) {
      Alert.alert("Error", "Please enter an email address");
      return;
    }
    setIsLoading(true);
    try {
      const result = await signInWithEmail({ email });
      setFlowId(result.flowId);
      Alert.alert("Success", "OTP sent to your email!");
    } catch (error) {
      Alert.alert("Error", error instanceof Error ? error.message : "Failed to sign in");
    } finally {
      setIsLoading(false);
    }
  };
  const handleVerifyOTP = async () => {
    if (!otp || !flowId) {
      Alert.alert("Error", "Please enter the OTP");
      return;
    }
    setIsLoading(true);
    try {
      const { user } = await verifyEmailOTP({ flowId, otp });
      Alert.alert("Success", "Successfully signed in!");
      console.log("Signed in user:", user);
    } catch (error) {
      Alert.alert("Error", error instanceof Error ? error.message : "Failed to verify OTP");
    } finally {
      setIsLoading(false);
    }
  };
  return (
    <View style={{ padding: 20 }}>
      <Text>Email:</Text>
      <TextInput
        value={email}
        onChangeText={setEmail}
        placeholder="Enter your email"
        keyboardType="email-address"
        autoCapitalize="none"
        editable={!isLoading}
        style={{ borderWidth: 1, borderColor: "#ddd", padding: 12, marginBottom: 16 }}
      />
      <TouchableOpacity
        onPress={handleSignIn}
        disabled={isLoading}
        style={{
          backgroundColor: "#007AFF",
          padding: 15,
          borderRadius: 8,
          alignItems: "center",
          marginBottom: 12,
          opacity: isLoading ? 0.6 : 1,
        }}
      >
        <Text style={{ color: "white", fontSize: 16, fontWeight: "600" }}>
          {isLoading ? "Sending..." : "Sign In with Email"}
        </Text>
      </TouchableOpacity>
      {flowId && (
        <>
          <Text>Enter OTP from email:</Text>
          <TextInput
            value={otp}
            onChangeText={setOtp}
            placeholder="Enter 6-digit OTP"
            keyboardType="number-pad"
            maxLength={6}
            editable={!isLoading}
            style={{ borderWidth: 1, borderColor: "#ddd", padding: 12, marginBottom: 16 }}
          />
          <TouchableOpacity
            onPress={handleVerifyOTP}
            disabled={isLoading}
            style={{
              backgroundColor: "#007AFF",
              padding: 15,
              borderRadius: 8,
              alignItems: "center",
              opacity: isLoading ? 0.6 : 1,
            }}
          >
            <Text style={{ color: "white", fontSize: 16, fontWeight: "600" }}>
              {isLoading ? "Verifying..." : "Verify OTP"}
            </Text>
          </TouchableOpacity>
        </>
      )}
    </View>
  );
}

```

### Link Additional Authentication Methods

Once a user is authenticated, you can link additional authentication methods to their account. This allows users to sign in using multiple methods (email, SMS, OAuth providers) with the same embedded wallet.

#### Link an Email Address

Use the `useLinkEmail` hook to link an email address to the currently authenticated user:

```
import { useLinkEmail, useVerifyEmailOTP, useCurrentUser } from "@coinbase/cdp-hooks";
function LinkEmail() {
  const { linkEmail } = useLinkEmail();
  const { verifyEmailOTP } = useVerifyEmailOTP();
  const { currentUser } = useCurrentUser();
  const [flowId, setFlowId] = useState("");
  const handleLinkEmail = async (email: string) => {
    if (!currentUser) {
      console.error("User must be signed in first");
      return;
    }
    try {
      // Initiate email linking
      const result = await linkEmail(email);
      setFlowId(result.flowId);
      // In a real application, you would prompt the user for the OTP
      const otp = "123456";
      // Verify the OTP to complete linking
      await verifyEmailOTP({
        flowId: result.flowId,
        otp
      });
      console.log("Email linked successfully!");
    } catch (error) {
      console.error("Failed to link email:", error);
    }
  };
  return (
    <button
      onClick={() => handleLinkEmail("additional-email@example.com")}
      disabled={!currentUser}
    >
      Link Email
    </button>
  );
}

```

#### Link a Phone Number

Use the `useLinkSms` hook to link a phone number to the currently authenticated user:

```
import { useLinkSms, useVerifySmsOTP, useCurrentUser } from "@coinbase/cdp-hooks";
function LinkPhoneNumber() {
  const { linkSms } = useLinkSms();
  const { verifySmsOTP } = useVerifySmsOTP();
  const { currentUser } = useCurrentUser();
  const [flowId, setFlowId] = useState("");
  const handleLinkSms = async (phoneNumber: string) => {
    if (!currentUser) {
      console.error("User must be signed in first");
      return;
    }
    try {
      // Initiate SMS linking
      const result = await linkSms(phoneNumber);
      setFlowId(result.flowId);
      // In a real application, you would prompt the user for the OTP
      const otp = "123456";
      // Verify the OTP to complete linking
      await verifySmsOTP({
        flowId: result.flowId,
        otp
      });
      console.log("Phone number linked successfully!");
    } catch (error) {
      console.error("Failed to link phone number:", error);
    }
  };
  return (
    <button
      onClick={() => handleLinkSms("+14155552671")}
      disabled={!currentUser}
    >
      Link Phone Number
    </button>
  );
}

```

#### Link a Google Account

Use the `useLinkGoogle` hook to link a Google account to the currently authenticated user:

```
import { useLinkGoogle, useCurrentUser } from "@coinbase/cdp-hooks";
function LinkGoogleAccount() {
  const { linkGoogle } = useLinkGoogle();
  const { currentUser } = useCurrentUser();
  const handleLinkGoogle = async () => {
    if (!currentUser) {
      console.error("User must be signed in first");
      return;
    }
    try {
      // This initiates the OAuth flow to link a Google account
      await linkGoogle();
      // The user will be redirected to Google for authentication
      // After successful authentication, the Google account will be linked
    } catch (error) {
      console.error("Failed to link Google account:", error);
    }
  };
  return (
    <button onClick={handleLinkGoogle} disabled={!currentUser}>
      Link Google Account
    </button>
  );
}

```

#### Link an Apple Account

Use the `useLinkApple` hook to link an Apple account to the currently authenticated user:

```
import { useLinkApple, useCurrentUser } from "@coinbase/cdp-hooks";
function LinkAppleAccount() {
  const { linkApple } = useLinkApple();
  const { currentUser } = useCurrentUser();
  const handleLinkApple = async () => {
    if (!currentUser) {
      console.error("User must be signed in first");
      return;
    }
    try {
      // Link an Apple account
      await linkApple();
    } catch (error) {
      console.error("Failed to link Apple account:", error);
    }
  };
  return (
    <button onClick={handleLinkApple} disabled={!currentUser}>  
      Link Apple Account
    </button>
  );
}

```

#### Link Any OAuth Provider

Use the `useLinkOAuth` hook to link any supported OAuth provider (Google, Apple) to the currently authenticated user:

```
import { useLinkOAuth, useCurrentUser } from "@coinbase/cdp-hooks";
function LinkOAuthProvider() {
  const { linkOAuth } = useLinkOAuth();
  const { currentUser } = useCurrentUser();
  const handleLinkGoogle = async () => {
    if (!currentUser) {
      console.error("User must be signed in first");
      return;
    }
    try {
      // Link a Google account
      await linkOAuth("google");
    } catch (error) {
      console.error("Failed to link Google account:", error);
    }
  };
  const handleLinkApple = async () => {
    if (!currentUser) return;
    try {
      // Link an Apple account
      await linkOAuth("apple");
    } catch (error) {
      console.error("Failed to link Apple account:", error);
    }
  };
  return (
    <div>
      <button onClick={handleLinkGoogle} disabled={!currentUser}>
        Link Google
      </button>
      <button onClick={handleLinkApple} disabled={!currentUser}>
        Link Apple
      </button>
    </div>
  );
}

```

### Sign In with Custom Authentication

If you’re using a third-party identity provider (Auth0, Firebase, AWS Cognito, or any OIDC-compliant provider), you can authenticate users with JWTs from your provider using the `useAuthenticateWithJWT` hook.

#### Prerequisites

Before using custom authentication:

1.  **Configure your identity provider in the CDP Portal**:
    -   Navigate to [Embedded Wallet Configuration](https://portal.cdp.coinbase.com/products/embedded-wallets)
    -   Click on the Custom auth tab
    -   Add your JWKS endpoint URL (e.g., `https://your-domain.auth0.com/.well-known/jwks.json`)
    -   Configure your JWT issuer and audience
2.  **Provide a `customAuth.getJwt` callback** in your provider configuration:

```
import { CDPHooksProvider } from "@coinbase/cdp-hooks";
function App() {
  return (
    <CDPHooksProvider
      config={{
        projectId: "your-project-id",
        customAuth: {
          // This callback should return a fresh JWT from your identity provider
          getJwt: async () => {
            // Return a JWT from your IDP (Auth0, Firebase, Cognito, etc.)
            // This will be called automatically when the SDK needs a fresh token
            const token = await yourAuthProvider.getAccessToken();
            return token;
          }
        },
        ethereum: {
          createOnLogin: "eoa" // Optional: configure wallet creation
        }
      }}
    >
      <YourApp />
    </CDPHooksProvider>
  );
}

```

#### Authenticate a User

```
import { useAuthenticateWithJWT, useCurrentUser } from "@coinbase/cdp-hooks";
function CustomAuthSignIn() {
  const { authenticateWithJWT } = useAuthenticateWithJWT();
  const { currentUser } = useCurrentUser();
  const handleSignIn = async () => {
    try {
      // After your user has signed in to your IDP (Auth0, Firebase, etc.)
      const result = await authenticateWithJWT();
      console.log("User authenticated:", result.user);
      console.log("Is new user:", result.isNewUser);
      // The user is now signed in and wallets are created based on your config
      if (result.user.evmAccountObjects?.[0]?.address) {
        console.log("EVM Address:", result.user.evmAccounts[0]);
      }
    } catch (error) {
      console.error("Authentication failed:", error);
    }
  };
  return (
    <div>
      <button onClick={handleSignIn}>Sign In with Custom Auth</button>
      {currentUser && <p>Signed in as: {currentUser.userId}</p>}
    </div>
  );
}

```

### Multi-Factor Authentication (MFA)

The CDP SDK provides hooks for implementing Multi-Factor Authentication using two methods:

-   **TOTP (Time-based One-Time Password)**: Users enroll using authenticator apps like Google Authenticator or Authy
-   **SMS**: Users receive verification codes via text message to their phone number

MFA adds an extra layer of security for user accounts and sensitive operations.

> **Important**: Users must be authenticated (signed in) before they can enroll in MFA or perform MFA verification.

#### Enroll User in TOTP MFA

Use `useInitiateMfaEnrollment` and `useSubmitMfaEnrollment` to enroll a user in TOTP MFA:

```
import { 
  useInitiateMfaEnrollment, 
  useSubmitMfaEnrollment,
  useCurrentUser 
} from "@coinbase/cdp-hooks";
import { useState } from "react";
import QRCode from "react-qr-code";
function MfaEnrollment() {
  const { initiateMfaEnrollment } = useInitiateMfaEnrollment();
  const { submitMfaEnrollment } = useSubmitMfaEnrollment();
  const { currentUser } = useCurrentUser();
  const [enrollmentData, setEnrollmentData] = useState<{
    authUrl: string;
    secret: string;
  } | null>(null);
  const [mfaCode, setMfaCode] = useState("");
  const handleInitiateEnrollment = async () => {
    try {
      // Step 1: Initiate MFA enrollment
      const result = await initiateMfaEnrollment({ mfaMethod: "totp" });
      
      // Store the authUrl and secret for QR code display
      setEnrollmentData(result);
      
      console.log("Scan QR code or enter secret:", result.secret);
    } catch (error) {
      console.error("Failed to initiate MFA enrollment:", error);
    }
  };
  const handleSubmitEnrollment = async () => {
    try {
      // Step 2: Submit the 6-digit code from authenticator app
      const result = await submitMfaEnrollment({
        mfaMethod: "totp",
        mfaCode: mfaCode,
      });
      
      console.log("MFA enrolled successfully for user:", result.user.userId);
      setEnrollmentData(null);
      setMfaCode("");
    } catch (error) {
      console.error("Failed to submit MFA enrollment:", error);
    }
  };
  if (!currentUser) {
    return <div>Please sign in to enable MFA</div>;
  }
  return (
    <div>
      <h2>Enable MFA</h2>
      
      {!enrollmentData ? (
        <button onClick={handleInitiateEnrollment}>
          Start MFA Enrollment
        </button>
      ) : (
        <div>
          <p>Scan this QR code with your authenticator app:</p>
          <QRCode value={enrollmentData.authUrl} />
          
          <p>Or manually enter this secret: {enrollmentData.secret}</p>
          
          <input
            type="text"
            placeholder="Enter 6-digit code"
            value={mfaCode}
            onChange={(e) => setMfaCode(e.target.value)}
            maxLength={6}
          />
          
          <button 
            onClick={handleSubmitEnrollment}
            disabled={mfaCode.length !== 6}
          >
            Verify and Enable MFA
          </button>
        </div>
      )}
    </div>
  );
}

```

#### Enroll User in SMS MFA

Use `useInitiateMfaEnrollment` and `useSubmitMfaEnrollment` to enroll a user in SMS MFA:

```
import {
  useInitiateMfaEnrollment,
  useSubmitMfaEnrollment,
  useCurrentUser
} from "@coinbase/cdp-hooks";
import { useState } from "react";
function SmsMfaEnrollment() {
  const { initiateMfaEnrollment } = useInitiateMfaEnrollment();
  const { submitMfaEnrollment } = useSubmitMfaEnrollment();
  const { currentUser } = useCurrentUser();
  const [phoneNumber, setPhoneNumber] = useState("");
  const [mfaCode, setMfaCode] = useState("");
  const [enrollmentInitiated, setEnrollmentInitiated] = useState(false);
  const handleInitiateEnrollment = async () => {
    try {
      // Step 1: Initiate SMS MFA enrollment (sends OTP to phone)
      const result = await initiateMfaEnrollment({
        mfaMethod: "sms",
        phoneNumber: phoneNumber // E.164 format (e.g., +14155552671)
      });
      console.log("SMS sent:", result.success);
      setEnrollmentInitiated(true);
    } catch (error) {
      console.error("Failed to initiate SMS MFA enrollment:", error);
    }
  };
  const handleSubmitEnrollment = async () => {
    try {
      // Step 2: Submit the 6-digit code received via SMS
      const result = await submitMfaEnrollment({
        mfaMethod: "sms",
        mfaCode: mfaCode,
      });
      console.log("SMS MFA enrolled successfully for user:", result.user.userId);
      console.log("SMS enrollment info:", result.user.mfaMethods?.sms);
      setEnrollmentInitiated(false);
      setPhoneNumber("");
      setMfaCode("");
    } catch (error) {
      console.error("Failed to submit SMS MFA enrollment:", error);
    }
  };
  if (!currentUser) {
    return <div>Please sign in to enable SMS MFA</div>;
  }
  return (
    <div>
      <h2>Enable SMS MFA</h2>
      {!enrollmentInitiated ? (
        <div>
          <input
            type="tel"
            placeholder="Phone number (e.g., +14155552671)"
            value={phoneNumber}
            onChange={(e) => setPhoneNumber(e.target.value)}
          />
          <button
            onClick={handleInitiateEnrollment}
            disabled={!phoneNumber}
          >
            Send SMS Code
          </button>
        </div>
      ) : (
        <div>
          <p>A 6-digit code has been sent to {phoneNumber}</p>
          <input
            type="text"
            placeholder="Enter 6-digit code"
            value={mfaCode}
            onChange={(e) => setMfaCode(e.target.value)}
            maxLength={6}
          />
          <button
            onClick={handleSubmitEnrollment}
            disabled={mfaCode.length !== 6}
          >
            Verify and Enable SMS MFA
          </button>
        </div>
      )}
    </div>
  );
}

```

#### Verify TOTP MFA for Sensitive Operations

Use `useInitiateMfaVerification` and `useSubmitMfaVerification` to verify TOTP MFA for sensitive operations:

```
import {
  useInitiateMfaVerification,
  useSubmitMfaVerification,
  useCurrentUser
} from "@coinbase/cdp-hooks";
import { useState } from "react";
function MfaVerification() {
  const { initiateMfaVerification } = useInitiateMfaVerification();
  const { submitMfaVerification } = useSubmitMfaVerification();
  const { currentUser } = useCurrentUser();
  const [mfaCode, setMfaCode] = useState("");
  const [verificationInitiated, setVerificationInitiated] = useState(false);
  const handleSensitiveOperation = async () => {
    try {
      // Step 1: Initiate MFA verification
      await initiateMfaVerification({ mfaMethod: "totp" });
      setVerificationInitiated(true);
      
      console.log("MFA verification initiated. Enter your code.");
    } catch (error) {
      console.error("Failed to initiate MFA verification:", error);
    }
  };
  const handleVerifyMfa = async () => {
    try {
      // Step 2: Submit the 6-digit code from authenticator app
      await submitMfaVerification({
        mfaMethod: "totp",
        mfaCode: mfaCode,
      });
      
      console.log("MFA verified successfully");
      setVerificationInitiated(false);
      setMfaCode("");
      
      // Now proceed with your sensitive operation
      // e.g., sign a transaction, export private key, etc.
    } catch (error) {
      console.error("MFA verification failed:", error);
    }
  };
  if (!currentUser) {
    return <div>Please sign in first</div>;
  }
  return (
    <div>
      <h2>Sensitive Operation</h2>
      
      {!verificationInitiated ? (
        <button onClick={handleSensitiveOperation}>
          Perform Sensitive Operation
        </button>
      ) : (
        <div>
          <p>Enter your MFA code to continue:</p>
          
          <input
            type="text"
            placeholder="Enter 6-digit code"
            value={mfaCode}
            onChange={(e) => setMfaCode(e.target.value)}
            maxLength={6}
          />
          
          <button 
            onClick={handleVerifyMfa}
            disabled={mfaCode.length !== 6}
          >
            Verify MFA
          </button>
        </div>
      )}
    </div>
  );
}

```

#### Verify SMS MFA for Sensitive Operations

Use `useInitiateMfaVerification` and `useSubmitMfaVerification` to verify SMS MFA for sensitive operations:

```
import {
  useInitiateMfaVerification,
  useSubmitMfaVerification,
  useCurrentUser
} from "@coinbase/cdp-hooks";
import { useState } from "react";
function SmsMfaVerification() {
  const { initiateMfaVerification } = useInitiateMfaVerification();
  const { submitMfaVerification } = useSubmitMfaVerification();
  const { currentUser } = useCurrentUser();
  const [mfaCode, setMfaCode] = useState("");
  const [verificationInitiated, setVerificationInitiated] = useState(false);
  const handleSensitiveOperation = async () => {
    try {
      // Step 1: Initiate SMS MFA verification (sends code to enrolled phone)
      await initiateMfaVerification({ mfaMethod: "sms" });
      setVerificationInitiated(true);
      console.log("SMS sent to enrolled phone number. Enter your code.");
    } catch (error) {
      console.error("Failed to initiate SMS MFA verification:", error);
    }
  };
  const handleVerifyMfa = async () => {
    try {
      // Step 2: Submit the 6-digit code from SMS
      await submitMfaVerification({
        mfaMethod: "sms",
        mfaCode: mfaCode,
      });
      console.log("SMS MFA verified successfully");
      setVerificationInitiated(false);
      setMfaCode("");
      // Now proceed with your sensitive operation
      // e.g., sign a transaction, export private key, etc.
    } catch (error) {
      console.error("SMS MFA verification failed:", error);
    }
  };
  if (!currentUser) {
    return <div>Please sign in first</div>;
  }
  return (
    <div>
      <h2>Sensitive Operation (SMS MFA)</h2>
      {!verificationInitiated ? (
        <button onClick={handleSensitiveOperation}>
          Perform Sensitive Operation
        </button>
      ) : (
        <div>
          <p>A 6-digit code has been sent to your phone. Enter it to continue:</p>
          <input
            type="text"
            placeholder="Enter 6-digit code"
            value={mfaCode}
            onChange={(e) => setMfaCode(e.target.value)}
            maxLength={6}
          />
          <button
            onClick={handleVerifyMfa}
            disabled={mfaCode.length !== 6}
          >
            Verify SMS MFA
          </button>
        </div>
      )}
    </div>
  );
}

```

#### Check MFA Configuration

Use `useGetMfaConfig` to check whether MFA is enabled for your project:

```
import { useGetMfaConfig } from "@coinbase/cdp-hooks";
import { useState, useEffect } from "react";
function MfaStatus() {
  const { getMfaConfig } = useGetMfaConfig();
  const [config, setConfig] = useState<{
    enabled: boolean;
    totpConfig: { enabled: boolean };
    createdAt: string;
    updatedAt: string;
  } | null>(null);
  useEffect(() => {
    const fetchMfaConfig = async () => {
      try {
        const result = await getMfaConfig();
        setConfig(result);
        console.log("MFA enabled:", result.enabled);
        console.log("TOTP enabled:", result.totpConfig.enabled);
      } catch (error) {
        console.error("Failed to fetch MFA config:", error);
      }
    };
    fetchMfaConfig();
  }, []);
  if (!config) {
    return <div>Loading MFA configuration...</div>;
  }
  return (
    <div>
      <h3>MFA Configuration</h3>
      <p>MFA Enabled: {config.enabled ? "Yes" : "No"}</p>
      <p>TOTP Enabled: {config.totpConfig.enabled ? "Yes" : "No"}</p>
      <p>Last Updated: {new Date(config.updatedAt).toLocaleDateString()}</p>
    </div>
  );
}

```

#### Track MFA Enrollment Prompts

Use `useRecordMfaEnrollmentPrompted` to track when users are shown the MFA enrollment prompt:

```
import { 
  useRecordMfaEnrollmentPrompted, 
  useCurrentUser 
} from "@coinbase/cdp-hooks";
import { useState } from "react";
function MfaEnrollmentPrompt() {
  const { recordMfaEnrollmentPrompted } = useRecordMfaEnrollmentPrompted();
  const { currentUser } = useCurrentUser();
  const [showPrompt, setShowPrompt] = useState(true);
  const handleSkip = async () => {
    try {
      // Record that the user was prompted and chose to skip
      const result = await recordMfaEnrollmentPrompted();
      
      console.log("Enrollment prompt recorded at:", result.enrollmentPromptedAt);
      
      // User's state is now updated with the timestamp
      console.log("Last prompted:", currentUser?.mfaMethods?.enrollmentPromptedAt);
      
      setShowPrompt(false);
    } catch (error) {
      console.error("Failed to record MFA prompt:", error);
    }
  };
  const handleEnroll = async () => {
    // Record the prompt before starting enrollment
    await recordMfaEnrollmentPrompted();
    
    // Navigate to enrollment flow
    console.log("Starting MFA enrollment...");
  };
  // Check if user was recently prompted (within last 30 days)
  const wasRecentlyPrompted = () => {
    if (!currentUser?.mfaMethods?.enrollmentPromptedAt) {
      return false;
    }
    
    const promptedDate = new Date(currentUser.mfaMethods.enrollmentPromptedAt);
    const thirtyDaysAgo = new Date();
    thirtyDaysAgo.setDate(thirtyDaysAgo.getDate() - 30);
    
    return promptedDate > thirtyDaysAgo;
  };
  // Don't show prompt if user was recently prompted or already enrolled
  if (!showPrompt || wasRecentlyPrompted() || currentUser?.mfaMethods?.totp) {
    return null;
  }
  return (
    <div style={{ 
      padding: "20px", 
      backgroundColor: "#f0f9ff", 
      border: "1px solid #bae6fd",
      borderRadius: "8px"
    }}>
      <h3>🔒 Secure Your Account with MFA</h3>
      <p>
        Add an extra layer of security by enabling Multi-Factor Authentication.
      </p>
      
      <div style={{ marginTop: "15px", display: "flex", gap: "10px" }}>
        <button onClick={handleEnroll}>
          Enable MFA
        </button>
        
        <button onClick={handleSkip} style={{ backgroundColor: "#e5e7eb" }}>
          Skip for Now
        </button>
      </div>
      
      <p style={{ fontSize: "12px", marginTop: "10px", color: "#6b7280" }}>
        You won't be prompted again for 30 days
      </p>
    </div>
  );
}

```

**Use Cases for Tracking Enrollment Prompts:**

-   Track when users skip MFA enrollment to avoid over-prompting
-   Implement smart re-prompting logic based on time intervals
-   Analyze MFA adoption rates across your user base
-   Identify users who need additional education about MFA benefits

### View User Information

Once the end user has signed in, you can display their information in your application:

```
import { useCurrentUser, useEvmAddress } from "@coinbase/cdp-hooks";
function UserInformation() {
  const { currentUser: user } = useCurrentUser();
  const { evmAddress } = useEvmAddress();
  if (!user) {
    return <div>Please sign in</div>;
  }
  const emailAddress = user.authenticationMethods.email?.email;
  return (
    <div>
      <h2>User Information</h2>
      <p>User ID: {user.userId}</p>
      <>
        <p>EVM Address (EOA): {evmAddress}</p>
        {user.evmSmartAccountObjects?.[0]?.address && (
          <p>Smart Account: {user.evmSmartAccounts[0]}</p>
        )}
      </>
      {emailAddress && <p>Email Address: {emailAddress}</p>}
    </div>
  );
}

```

### Create Accounts Manually

If you configured your provider without `createOnLogin`, you can manually create accounts for authenticated users when needed using these hooks.

#### Create an EOA Account

Use the `useCreateEoaAccount` hook to create an EOA (Externally Owned Account) for the current user:

```
import { useCreateEoaAccount, useCurrentUser } from "@coinbase/cdp-hooks";
function CreateEoaAccount() {
  const { createEoaAccount } = useCreateEoaAccount();
  const { currentUser } = useCurrentUser();
  const handleCreateAccount = async () => {
    try {
      const evmAddress = await createEoaAccount();
      console.log("Created EOA:", evmAddress);
      // The user object is automatically updated
    } catch (error) {
      console.error("Failed to create EOA:", error);
    }
  };
  return (
    <button
      onClick={handleCreateAccount}
      disabled={!currentUser || currentUser.evmAccounts?.length > 0}
    >
      Create EOA Account
    </button>
  );
}

```

**Note:** This will throw an error if the user already has an EOA account.

#### Create a Smart Account

Use the `useCreateSmartAccount` hook to create a Smart Account for the current user:

```
import { useCreateSmartAccount, useCurrentUser } from "@coinbase/cdp-hooks";
function CreateSmartAccount() {
  const { createSmartAccount } = useCreateSmartAccount();
  const { currentUser } = useCurrentUser();
  const handleCreateAccount = async () => {
    try {
      // Create Smart Account (will automatically create EOA first if needed)
      const smartAccountAddress = await createSmartAccount();
      console.log("Created Smart Account:", smartAccountAddress);
      // The user object is automatically updated
    } catch (error) {
      console.error("Failed to create Smart Account:", error);
    }
  };
  const handleCreateWithSpendPermissions = async () => {
    try {
      // Create Smart Account with spend permissions enabled
      const smartAccountAddress = await createSmartAccount({
        enableSpendPermissions: true
      });
      console.log("Created Smart Account with spend permissions:", smartAccountAddress);
    } catch (error) {
      console.error("Failed to create Smart Account:", error);
    }
  };
  return (
    <div>
      <button
        onClick={handleCreateAccount}
        disabled={!currentUser || currentUser.evmSmartAccounts?.length > 0}
      >
        Create Smart Account
      </button>
      <button
        onClick={handleCreateWithSpendPermissions}
        disabled={!currentUser || currentUser.evmSmartAccounts?.length > 0}
      >
        Create Smart Account with Spend Permissions
      </button>
    </div>
  );
}

```

**Note:** This will throw an error if the user already has a Smart Account. If the user doesn’t have an EOA, one will be automatically created first to serve as the Smart Account owner.

#### Create a Solana Account

Use the `useCreateSolanaAccount` hook to create a Solana account for the current user:

```
import { useCreateSolanaAccount, useCurrentUser } from "@coinbase/cdp-hooks";
function CreateSolanaAccount() {
  const { createSolanaAccount } = useCreateSolanaAccount();
  const { currentUser } = useCurrentUser();
  const handleCreateAccount = async () => {
    try {
      const solanaAddress = await createSolanaAccount();
      console.log("Created Solana account:", solanaAddress);
      // The user object is automatically updated
    } catch (error) {
      console.error("Failed to create Solana account:", error);
    }
  };
  return (
    <button
      onClick={handleCreateAccount}
      disabled={!currentUser || currentUser.solanaAccounts?.length > 0}
    >
      Create Solana Account
    </button>
  );
}

```

**Note:** This will throw an error if the user already has a Solana account.

### Working with Solana

When your application is configured with `solana: { createOnLogin: true }`, you can use Solana-specific hooks to interact with Solana accounts.

#### Access Solana Address

Use the `useSolanaAddress` hook to get the user’s first Solana address:

```
import { useSolanaAddress } from "@coinbase/cdp-hooks";
function SolanaWallet() {
  const { solanaAddress } = useSolanaAddress();
  if (!solanaAddress) {
    return <div>No Solana wallet connected</div>;
  }
  return (
    <div>
      <h3>Your Solana Wallet</h3>
      <p>Address: {solanaAddress}</p>
    </div>
  );
}

```

#### Sign a Solana Transaction

Use the `useSignSolanaTransaction` hook to sign Solana transactions:

```
import { useSignSolanaTransaction, useSolanaAddress } from "@coinbase/cdp-hooks";
function SolanaTransactionSigner() {
  const { signSolanaTransaction } = useSignSolanaTransaction();
  const { solanaAddress } = useSolanaAddress();
  const handleSignTransaction = async () => {
    if (!solanaAddress) return;
    try {
      const result = await signSolanaTransaction({
        solanaAccount: solanaAddress,
        transaction: "base64-encoded-solana-transaction"  // Your Solana transaction here
      });
      
      console.log("Signed Transaction:", result.signedTransaction);
      // The signedTransaction can now be broadcast to the Solana network
    } catch (error) {
      console.error("Failed to sign transaction:", error);
    }
  };
  if (!solanaAddress) {
    return <div>Please connect your Solana wallet first</div>;
  }
  return (
    <div>
      <h3>Sign Solana Transaction</h3>
      <button onClick={handleSignTransaction}>
        Sign Transaction
      </button>
    </div>
  );
}

```

#### Sign a Solana Message

Use the `useSignSolanaMessage` hook to sign arbitrary messages with Solana accounts:

```
import { useSignSolanaMessage, useSolanaAddress } from "@coinbase/cdp-hooks";
function SolanaMessageSigner() {
  const { signSolanaMessage } = useSignSolanaMessage();
  const { solanaAddress } = useSolanaAddress();
  const handleSignMessage = async () => {
    if (!solanaAddress) return;
    try {
      const message = Buffer.from("Hello, Solana!", "utf8").toString("base64");
      const result = await signSolanaMessage({
        solanaAccount: solanaAddress,
        message // Base64 encoded message to sign
      });
      console.log("Message Signature:", result.signature);
      // The signature can be used for authentication or verification purposes
    } catch (error) {
      console.error("Failed to sign message:", error);
    }
  };
  if (!solanaAddress) {
    return <div>Please connect your Solana wallet first</div>;
  }
  return (
    <div>
      <h3>Sign Solana Message</h3>
      <button onClick={handleSignMessage}>
        Sign Message
      </button>
    </div>
  );
}

```

#### Send a Solana Transaction

Use the `useSendSolanaTransaction` hook to sign and send Solana transactions in a single action. This is supported on:

-   Solana Mainnet
-   Solana Devnet

```
import { useSendSolanaTransaction, useSolanaAddress } from "@coinbase/cdp-hooks";
function SolanaTransactionSender() {
  const { sendSolanaTransaction } = useSendSolanaTransaction();
  const { solanaAddress } = useSolanaAddress();
  const handleSendTransaction = async () => {
    if (!solanaAddress) return;
    try {
      const result = await sendSolanaTransaction({
        solanaAccount: solanaAddress,
        network: "solana-devnet", // or "solana" for mainnet
        transaction: "base64-encoded-solana-transaction"  // Your Solana transaction here
      });
      
      console.log("Transaction Signature:", result.transactionSignature);
      // The transaction has been broadcast to the Solana network
    } catch (error) {
      console.error("Failed to send transaction:", error);
    }
  };
  if (!solanaAddress) {
    return <div>Please connect your Solana wallet first</div>;
  }
  return (
    <div>
      <h3>Send Solana Transaction</h3>
      <button onClick={handleSendTransaction}>
        Send Transaction
      </button>
    </div>
  );
}

```

### Send a Transaction

We support signing and sending a Blockchain transaction in a single action on the following networks:

-   Base
-   Base Sepolia
-   Ethereum
-   Ethereum Sepolia
-   Avalanche
-   Arbitrum
-   Optimism
-   Polygon

```
import { useSendEvmTransaction, useEvmAddress } from "@coinbase/cdp-hooks";
function SendTransaction() {
  const { sendEvmTransaction: sendTransaction } = useSendEvmTransaction();
  const { evmAddress } = useEvmAddress();
  const handleSend = async () => {
    if (!evmAddress) return;
    try {
      const result = await sendTransaction({
        evmAccount: evmAddress,
        transaction: {
          to: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
          value: 100000000000000n, // 0.0001 ETH in wei
          nonce: 0,
          gas: 21000n,
          maxFeePerGas: 30000000000n,
          maxPriorityFeePerGas: 1000000000n,
          chainId: 84532, // Base Sepolia
          type: "eip1559",
        }
      });
      console.log("Transaction hash:", result.transactionHash);
    } catch (error) {
      console.error("Transaction failed:", error);
    }
  };
  return <button onClick={handleSend}>Send Transaction</button>;
}

```

For networks other than those supported by the CDP APIs, your end user must sign the transaction, and then you must broadcast the transaction yourself. This example uses the public client from `viem` to broadcast the transaction.

```
import { useSignEvmTransaction, useEvmAddress } from "@coinbase/cdp-hooks";
import { http, createPublicClient } from "viem";
import { tron } from "viem/chains";
function CrossChainTransaction() {
  const { signEvmTransaction: signTransaction } = useSignEvmTransaction();
  const { evmAddress } = useEvmAddress();
  const handleSend = async () => {
    if (!evmAddress) return;
    try {
      // Sign the transaction
      const { signedTransaction } = await signTransaction({
        evmAccount: evmAddress,
        transaction: {
          to: "0x...",
          value: 100000000000000n,
          nonce: 0,
          gas: 21000n,
          maxFeePerGas: 30000000000n,
          maxPriorityFeePerGas: 1000000000n,
          chainId: 728126428, // Tron
          type: "eip1559",
        }
      });
      // Broadcast using a different client
      const client = createPublicClient({
        chain: tron,
        transport: http()
      });
      const hash = await client.sendRawTransaction({
        serializedTransaction: signedTransaction
      });
      console.log("Transaction hash:", hash);
    } catch (error) {
      console.error("Transaction failed:", error);
    }
  };
  return <button onClick={handleSend}>Send Transaction</button>;
}

```

### Sign Messages and Typed Data

End users can sign EVM messages, hashes, and typed data to generate signatures for various onchain applications.

```
import { useSignEvmMessage, useSignEvmTypedData, useEvmAddress } from "@coinbase/cdp-hooks";
function SignData() {
  const { signEvmMessage: signMessage } = useSignEvmMessage();
  const { signEvmTypedData: signTypedData } = useSignEvmTypedData();
  const { signEvmHash: signHash } = useSignEvmHash();
  const { evmAddress } = useEvmAddress();
  const handleSignHash = async () => {
    if (!evmAddress) return;
    const result = await signMessage({
      evmAccount: evmAddress,
      message: "Hello World"
    });
    console.log("Message signature:", result.signature);
  }
  const handleSignMessage = async () => {
    if (!evmAddress) return;
    const result = await signMessage({
      evmAccount: evmAddress,
      message: "Hello World"
    });
    console.log("Message signature:", result.signature);
  };
  const handleSignTypedData = async () => {
    if (!evmAddress) return;
    const result = await signTypedData({
      evmAccount: evmAddress,
      typedData: {
        domain: {
          name: "Example DApp",
          version: "1",
          chainId: 84532,
        },
        types: {
          Person: [
            { name: "name", type: "string" },
            { name: "wallet", type: "address" }
          ]
        },
        primaryType: "Person",
        message: {
          name: "Bob",
          wallet: evmAddress
        }
      }
    });
    console.log("Typed data signature:", result.signature);
  };
  return (
    <div>
      <button onClick={handleSignMessage}>Sign Message</button>
      <button onClick={handleSignTypedData}>Sign Typed Data</button>
      <button onClick={handleSignHash}>Sign Hash</button>
    </div>
  );
}

```

### Export Private Keys

End users can export their private keys from their embedded wallet, allowing them to import them into compatible wallets of their choice.

#### Secure Iframe Export (Recommended)

The secure iframe hooks are the recommended way to export private keys. They create a secure iframe that copies the private key directly to the user’s clipboard without ever exposing it to your application’s JavaScript context.

##### Export EVM Private Key via Iframe

```
import { useEvmKeyExportIframe, useEvmAddress } from "@coinbase/cdp-hooks";
import { useRef, useState } from "react";
function ExportEvmKey() {
  const { evmAddress } = useEvmAddress();
  const containerRef = useRef<HTMLDivElement>(null);
  const [isActive, setIsActive] = useState(false);
  const { status, cleanup } = useEvmKeyExportIframe({
    address: isActive && evmAddress ? evmAddress : "",
    containerRef: isActive ? containerRef : { current: null },
    label: "Copy Private Key",
    copiedLabel: "Copied!",
  });
  return (
    <div>
      {!isActive ? (
        <button onClick={() => setIsActive(true)} disabled={!evmAddress}>
          Show Export Button
        </button>
      ) : (
        <button onClick={() => { cleanup(); setIsActive(false); }}>
          Hide Export Button
        </button>
      )}
      {isActive && (
        <div>
          <div ref={containerRef} />
          <p>Status: {status ?? "initializing"}</p>
        </div>
      )}
    </div>
  );
}

```

##### Export Solana Private Key via Iframe

```
import { useSolanaKeyExportIframe, useSolanaAddress } from "@coinbase/cdp-hooks";
import { useRef, useState } from "react";
function ExportSolanaKey() {
  const { solanaAddress } = useSolanaAddress();
  const containerRef = useRef<HTMLDivElement>(null);
  const [isActive, setIsActive] = useState(false);
  const { status, cleanup } = useSolanaKeyExportIframe({
    address: isActive && solanaAddress ? solanaAddress : "",
    containerRef: isActive ? containerRef : { current: null },
    label: "Copy Private Key",
    copiedLabel: "Copied!",
  });
  return (
    <div>
      {!isActive ? (
        <button onClick={() => setIsActive(true)} disabled={!solanaAddress}>
          Show Export Button
        </button>
      ) : (
        <button onClick={() => { cleanup(); setIsActive(false); }}>
          Hide Export Button
        </button>
      )}
      {isActive && (
        <div>
          <div ref={containerRef} />
          <p>Status: {status ?? "initializing"}</p>
        </div>
      )}
    </div>
  );
}

```

#### Direct Export (Deprecated)

> **⚠️ Deprecated:** The direct export hooks expose the private key to your application’s JavaScript context and will be removed soon. Use the secure iframe hooks above instead.

##### Export EVM Private Key (Deprecated)

```
import { useExportEvmAccount, useEvmAddress } from "@coinbase/cdp-hooks";
function ExportEvmKey() {
  const { exportEvmAccount: exportAccount } = useExportEvmAccount();
  const { evmAddress } = useEvmAddress();
  const handleExport = async () => {
    if (!evmAddress) return;
    try {
      const { privateKey } = await exportAccount({
        evmAccount: evmAddress
      });
      console.log("EVM Private Key:", privateKey);
      // Warning: Handle private keys with extreme care!
    } catch (error) {
      console.error("Export failed:", error);
    }
  };
  return <button onClick={handleExport}>Export EVM Private Key</button>;
}

```

##### Export Solana Private Key (Deprecated)

When your application is configured with `solana: { createOnLogin: true }`, you can export Solana private keys:

```
import { useExportSolanaAccount, useSolanaAddress } from "@coinbase/cdp-hooks";
function ExportSolanaKey() {
  const { exportSolanaAccount: exportAccount } = useExportSolanaAccount();
  const { solanaAddress } = useSolanaAddress();
  const handleExport = async () => {
    if (!solanaAddress) return;
    try {
      const { privateKey } = await exportAccount({
        solanaAccount: solanaAddress
      });
      console.log("Solana Private Key:", privateKey);
      // Warning: Handle private keys with extreme care!
    } catch (error) {
      console.error("Export failed:", error);
    }
  };
  if (!solanaAddress) {
    return <div>Please connect your Solana wallet first</div>;
  }
  return <button onClick={handleExport}>Export Solana Private Key</button>;
}

```

### Smart Account Operations

Smart Accounts provide advanced account abstraction features with React hooks.

#### Create Spend Permissions

Spend permissions allow Smart Accounts to delegate spending authority to other accounts within specified limits and time periods. The `useCreateSpendPermission` hook provides an easy way to create spend permissions with automatic user operation tracking.

```
import { useCreateSpendPermission, useCurrentUser } from "@coinbase/cdp-hooks";
function CreateSpendPermission() {
  const { createSpendPermission, status, data, error } = useCreateSpendPermission();
  const { currentUser } = useCurrentUser();
  const handleCreateSpendPermission = async () => {
    try {
      const result = await createSpendPermission({
        network: "base-sepolia",
        spender: "0x742D35Cc6634C0532925a3b8D6Ec6F1C2b9c1E46", // Address that can spend tokens
        token: "usdc", // Token symbol ("eth", "usdc") or contract address
        allowance: "10000000", // 10 USDC (6 decimals)
        periodInDays: 7, // Weekly recurring allowance
        useCdpPaymaster: true, // Use CDP paymaster for gas sponsorship
      });
      console.log("User Operation Hash:", result.userOperationHash);
    } catch (error) {
      console.error("Failed to create spend permission:", error);
    }
  };
  return (
    <div>
      {status === "idle" && <p>Ready to create spend permission</p>}
      {status === "pending" && (
        <div>
          <p>Creating spend permission...</p>
          {data && <p>User Op Hash: {data.userOpHash}</p>}
        </div>
      )}
      {status === "success" && data && (
        <div>
          <p>Spend permission created successfully!</p>
          <p>Transaction Hash: {data.transactionHash}</p>
          <p>Status: {data.status}</p>
        </div>
      )}
      {status === "error" && (
        <div>
          <p>Failed to create spend permission</p>
          <p>Error: {error?.message}</p>
        </div>
      )}
      <button
        onClick={handleCreateSpendPermission}
        disabled={status === "pending" || !currentUser?.evmSmartAccountObjects?.[0]?.address}
      >
        {status === "pending" ? "Creating..." : "Create Spend Permission"}
      </button>
    </div>
  );
}

```

The hook automatically:

-   Detects the user’s Smart Account (or allows you to specify `evmSmartAccount`)
-   Converts `periodInDays` to seconds
-   Resolves token symbols like “eth” and “usdc” to contract addresses
-   Tracks the user operation status and provides real-time updates

#### List Spend Permissions

Use the `useListSpendPermissions` hook to retrieve all spend permissions for a Smart Account. This hook follows a query-style pattern and automatically fetches permissions when enabled.

```
import { useListSpendPermissions, useCurrentUser } from "@coinbase/cdp-hooks";
function SpendPermissionsList() {
  const { currentUser } = useCurrentUser();
  const { data, error, status, refetch } = useListSpendPermissions({
    evmSmartAccount: currentUser?.evmSmartAccountObjects?.[0]?.address,
    network: "base-sepolia",
    pageSize: 10,
    enabled: true
  });
  if (status === "pending") {
    return <div>Loading spend permissions...</div>;
  }
  if (status === "error") {
    return <div>Error: {error?.message}</div>;
  }
  return (
    <div>
      <button onClick={refetch}>Refresh</button>
      <h3>Spend Permissions ({data?.spendPermissions.length})</h3>
      {data?.spendPermissions.map((permission) => (
        <div key={permission.permissionHash}>
          <p>Hash: {permission.permissionHash}</p>
          <p>Spender: {permission.permission.spender}</p>
          <p>Token: {permission.permission.token}</p>
          <p>Allowance: {permission.permission.allowance}</p>
          <p>Revoked: {permission.revoked ? "Yes" : "No"}</p>
        </div>
      ))}
      {data?.hasNextPage && (
        <p>More permissions available. Use pageToken to load next page.</p>
      )}
    </div>
  );
}

```

#### Revoke Spend Permissions

Use the `useRevokeSpendPermission` hook to revoke a spend permission. The hook automatically tracks the user operation and provides real-time status updates.

```
import { useRevokeSpendPermission } from "@coinbase/cdp-hooks";
function RevokeSpendPermission({ permissionHash }) {
  const { revokeSpendPermission, data, error, status } = useRevokeSpendPermission();
  const handleRevokeSpendPermission = async () => {
    try {
      const result = await revokeSpendPermission({
        network: "base-sepolia",
        permissionHash: permissionHash,
        useCdpPaymaster: true
      });
      console.log("User Operation Hash:", result.userOperationHash);
    } catch (error) {
      console.error("Failed to revoke spend permission:", error);
    }
  };
  return (
    <div>
      {status === "pending" && <p>Revoking permission...</p>}
      {status === "success" && data && (
        <div>
          <p>Permission revoked successfully!</p>
          <p>Transaction Hash: {data.transactionHash}</p>
        </div>
      )}
      {status === "error" && <p>Error: {error?.message}</p>}
      <button
        onClick={handleRevokeSpendPermission}
        disabled={status === "pending"}
      >
        {status === "pending" ? "Revoking..." : "Revoke Permission"}
      </button>
    </div>
  );
}

```

#### Send User Operations

Send user operations from Smart Accounts with support for multiple calls and paymaster sponsorship. The hook returns a method to execute the user operation and `status`, `data`, and `error` properties to read the result of the user operation:

```
import { useSendUserOperation, useCurrentUser } from "@coinbase/cdp-hooks";
function SendUserOperation() {
  const { sendUserOperation, status, data, error } = useSendUserOperation();
  const { currentUser } = useCurrentUser();
  const handleSendUserOperation = async () => {
    const smartAccount = currentUser?.evmSmartAccountObjects?.[0]?.address;
    if (!smartAccount) return;
    try {
      // This will automatically start tracking the user operation status
      const result = await sendUserOperation({
        evmSmartAccount: smartAccount,
        network: "base-sepolia",
        calls: [{
          to: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
          value: 1000000000000000000n,
          data: "0x",
        }],
      });
      console.log("User Operation Hash:", result.userOperationHash);
    } catch (error) {
      console.error("Failed to send user operation:", error);
    }
  };
  return (
    <div>
      {status === "idle" && <p>Ready to send user operation</p>}
      
      {status === "pending" && (
        <div>
          <p>User operation pending...</p>
          {data && <p>User Op Hash: {data.userOpHash}</p>}
        </div>
      )}
      
      {status === "success" && data && (
        <div>
          <p>User operation successful!</p>
          <p>Transaction Hash: {data.transactionHash}</p>
          <p>Status: {data.status}</p>
        </div>
      )}
      
      {status === "error" && (
        <div>
          <p>User operation failed</p>
          <p>Error: {error?.message}</p>
        </div>
      )}
      
      <button onClick={handleSendUserOperation} disabled={status === "pending"}>
        {status === "pending" ? "Sending..." : "Send User Operation"}
      </button>
    </div>
  );
}

```

**Transaction Attribution with EIP-8021:** You can add attribution data to user operations for tracking app usage and revenue sharing:

```
import { useSendUserOperation, useCurrentUser } from "@coinbase/cdp-hooks";
function SendUserOperationWithAttribution() {
  const { sendUserOperation, status } = useSendUserOperation();
  const { currentUser } = useCurrentUser();
  
  // EIP-8021 data suffix for "baseapp" attribution
  const dataSuffix = "0xdddddddd62617365617070070080218021802180218021802180218021";
  const handleSendWithAttribution = async () => {
    const smartAccount = currentUser?.evmSmartAccountObjects?.[0]?.address;
    if (!smartAccount) return;
    await sendUserOperation({
      evmSmartAccount: smartAccount,
      network: "base-sepolia",
      calls: [{ to: "0xTargetContract", value: 0n, data: "0x" }],
      dataSuffix, // Attribution data appended to callData
      useCdpPaymaster: true,
    });
  };
  return (
    <button onClick={handleSendWithAttribution} disabled={status === "pending"}>
      Send with Attribution
    </button>
  );
}

```

The `dataSuffix` parameter accepts a hex-encoded string following the [EIP-8021 standard](https://eip.tools/eip/8021). This enables onchain attribution for tracking and revenue sharing.

#### Track User Operation Status

Use the `useWaitForUserOperation` hook to poll for user operation status and provide real-time updates. This hook immediately fires off a query to get the result of the user operation:

```
import { useWaitForUserOperation, useState } from "react";
function WaitForUserOperation() {
  const { status, data, error } = useWaitForUserOperation({
    userOperationHash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
    evmSmartAccount: "0x1234567890123456789012345678901234567890",
    network: "base-sepolia"
  });
  return (
    <div>
      {status === "idle" && <p>No user operation being tracked</p>}
      
      {status === "pending" && (
        <div>
          <p>User operation pending...</p>
          {data && <p>User Op Hash: {data.userOpHash}</p>}
        </div>
      )}
      
      {status === "success" && data && (
        <div>
          <p>User operation successful!</p>
          <p>Transaction Hash: {data.transactionHash}</p>
          <p>Status: {data.status}</p>
        </div>
      )}
      
      {status === "error" && (
        <div>
          <p>User operation failed</p>
          <p>Error: {error?.message}</p>
        </div>
      )}
    </div>
  );
}

```

#### Conditional Polling

You can control when the `useWaitForUserOperation` hook should start polling using the `enabled` parameter:

```
function ConditionalWaitForUserOperation() {
  const [shouldPoll, setShouldPoll] = useState(false);
  
  const { status, data, error } = useWaitForUserOperation({
    userOperationHash: "0x1234...",
    evmSmartAccount: "0x5678...", 
    network: "base-sepolia",
    enabled: shouldPoll // Only poll when this is true
  });
  return (
    <div>
      <button onClick={() => setShouldPoll(true)}>
        Start Polling
      </button>
      <button onClick={() => setShouldPoll(false)}>
        Stop Polling  
      </button>
      
      <p>Status: {status}</p>
      {data && <p>User Operation Status: {data.status}</p>}
      {error && <p>Error: {error.message}</p>}
    </div>
  );
}

```

### X402 Payment Protocol Support

The SDK includes built-in support for the X402 payment protocol through the `useX402` hook. This enables HTTP requests with micropayments, allowing access to paid APIs and services that require payment for each request.

#### Installation

Ensure you have separately installed the `x402-fetch` package:

#### Basic Usage

The `useX402` hook provides a wrapped fetch API that automatically handles X402 payment requests:

```
import { useX402, useCurrentUser } from "@coinbase/cdp-hooks";
function X402Example() {
  const { currentUser } = useCurrentUser();
  const { fetchWithPayment } = useX402();
  const [data, setData] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  const handleFetchPaidResource = async () => {
    if (!currentUser) {
      setError("Please sign in first");
      return;
    }
    setLoading(true);
    setError(null);
    try {
      const response = await fetchWithPayment("https://api.example.com/paid-endpoint", {
        method: "GET",
        headers: {
          "Content-Type": "application/json"
        }
      });
      
      const result = await response.json();
      setData(result);
      console.log("Paid API response:", result);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to fetch resource");
      console.error("X402 payment failed:", err);
    } finally {
      setLoading(false);
    }
  };
  return (
    <div>
      <h3>X402 Payment Example</h3>
      
      {!currentUser && <p>Please sign in to use X402 payments</p>}
      
      <button 
        onClick={handleFetchPaidResource} 
        disabled={!currentUser || loading}
      >
        {loading ? "Processing..." : "Fetch Paid Resource"}
      </button>
      
      {error && <p style={{ color: 'red' }}>Error: {error}</p>}
      
      {data && (
        <div>
          <h4>Response:</h4>
          <pre>{JSON.stringify(data, null, 2)}</pre>
        </div>
      )}
    </div>
  );
}

```

#### Advanced Configuration

You can customize the X402 behavior with options:

```
import { useX402 } from "@coinbase/cdp-hooks";
function X402AdvancedExample() {
  // Use a specific address for payments (instead of the user's default)
  const { fetchWithPayment: fetchWithSpecificAddress } = useX402({
    address: "0x1234567890123456789012345678901234567890"
  });
  // Use a custom fetch implementation
  const customFetch = (url, options) => {
    console.log("Making request to:", url);
    return fetch(url, options);
  };
  const { fetchWithPayment: fetchWithCustomImpl } = useX402({
    fetch: customFetch
  });
  // Combine both options
  const { fetchWithPayment: fetchWithBothOptions } = useX402({
    address: "0x1234567890123456789012345678901234567890",
    fetch: customFetch
  });
  return (
    <div>
      <p>Multiple X402 configurations available</p>
    </div>
  );
}

```

#### How It Works

1.  When you make a request to an X402-protected resource, the server responds with a `402 Payment Required` status
2.  The wrapped fetch function automatically:
    -   Extracts payment details from the server’s response
    -   Creates and signs a payment transaction using the user’s wallet
    -   Includes the payment proof in a retry request
3.  The server validates the payment and returns the requested resource

#### Smart Account Support

By default, `useX402` will use the user’s Smart Account if available, falling back to their regular EVM account:

```
import { useX402, useCurrentUser } from "@coinbase/cdp-hooks";
function X402SmartAccountExample() {
  const { currentUser } = useCurrentUser();
  const { fetchWithPayment } = useX402();
  // The hook automatically uses the appropriate account type
  const accountUsed = currentUser?.evmSmartAccountObjects?.[0]?.address || currentUser?.evmAccountObjects?.[0]?.address;
  return (
    <div>
      <p>X402 will use account: {accountUsed}</p>
      <p>Account type: {currentUser?.evmSmartAccountObjects?.[0]?.address ? "Smart Account" : "EOA"}</p>
    </div>
  );
}

```

#### Solana Support

Solana support is supported out of the box with `useX402`. If your end user has both an EVM and Solana account, the EVM account will be used by default. You can pass a Solana address to `useX402` to use the Solana account instead.

```
import { useX402, useCurrentUser } from "@coinbase/cdp-hooks";
const { currentUser } = useCurrentUser();
const { fetchWithPayment } = useX402({
  address: currentUser?.solanaAccountObjects?.[0]?.address
});
await fetchWithPayment("https://api.example.com/paid-endpoint", {
  method: "GET",
  headers: {
    "Content-Type": "application/json"
  }
});

```

#### React Native Support

The `useX402` hook works seamlessly in React Native applications:

```
import React, { useState } from "react";
import { View, Text, TouchableOpacity, ActivityIndicator } from "react-native";
import { useX402, useCurrentUser } from "@coinbase/cdp-hooks";
function X402ReactNativeExample() {
  const { currentUser } = useCurrentUser();
  const { fetchWithPayment } = useX402();
  const [loading, setLoading] = useState(false);
  const [data, setData] = useState(null);
  const handleFetchPaidResource = async () => {
    setLoading(true);
    try {
      const response = await fetchWithPayment("https://api.example.com/paid-endpoint");
      const result = await response.json();
      setData(result);
    } catch (error) {
      console.error("Failed to fetch paid resource:", error);
    } finally {
      setLoading(false);
    }
  };
  return (
    <View style={{ padding: 20 }}>
      <TouchableOpacity
        onPress={handleFetchPaidResource}
        disabled={!currentUser || loading}
        style={{
          backgroundColor: "#007AFF",
          padding: 15,
          borderRadius: 8,
          alignItems: "center",
          opacity: !currentUser || loading ? 0.6 : 1,
        }}
      >
        {loading ? (
          <ActivityIndicator color="white" />
        ) : (
          <Text style={{ color: "white", fontSize: 16, fontWeight: "600" }}>
            Fetch X402 Resource
          </Text>
        )}
      </TouchableOpacity>
      
      {data && (
        <View style={{ marginTop: 20, padding: 10, backgroundColor: "#f0f0f0" }}>
          <Text>{JSON.stringify(data, null, 2)}</Text>
        </View>
      )}
    </View>
  );
}

```