# cdp core

This package contains core business logic for the CDP Frontend SDK. It is intended for non-React applications that use pure Typescript.

## Quickstart

This guide will help you get started with `@coinbase/cdp-core`. You’ll learn how to install the package, initialize the SDK, and make your first API call.

### Installation

First, add the package to your project using your preferred package manager.

```
# With pnpm
pnpm add @coinbase/cdp-core
# With yarn
yarn add @coinbase/cdp-core
# With npm
npm install @coinbase/cdp-core

```

### Gather your CDP Project information

1.  Sign in or create an account on the [CDP Portal](https://portal.cdp.coinbase.com/)
2.  On your dashboard, select a project from the dropdown at the at the top, and copy the Project ID

### Allowlist your local app

1.  Navigate to the [Embedded Wallet Configuration](https://portal.cdp.coinbase.com/products/embedded-wallets/cors) in CDP Portal, and click Add origin to include your local app
2.  Enter the origin of your locally running app - e.g., `http://localhost:3000`
3.  Click Add origin again to save your changes

### Initialize the SDK

Before calling any methods in the SDK, you must first initialize it:

```
import { Config, initialize } from "@coinbase/cdp-core";
const config: Config = {
  // Copy and paste your project ID here.
  projectId: "your-project-id",
}
await initialize(config);

```

#### Analytics Opt-Out

By default the SDK will emit usage analytics to help us improve the SDK. If you would like to opt-out, you can do so by setting the `disableAnalytics` configuration option to `true`.

```
const config: Config = {
  projectId: "your-project-id",
  disableAnalytics: true,
}
await initialize(config);

```

#### Account Configuration

You can configure the SDK to create different types of accounts for new users: **Smart Account Configuration:**

```
const config: Config = {
  projectId: "your-project-id",
  ethereum: {
    createOnLogin: "smart", // Creates Smart Accounts instead of EOAs
  },
}
await initialize(config);

```

When `ethereum.createOnLogin` is set to `"smart"`, the SDK will:

1.  Automatically create an EOA (Externally Owned Account) as the owner
2.  Create a Smart Account owned by that EOA
3.  Both accounts will be available on the user object

**Solana Account Configuration:**

```
const config: Config = {
  projectId: "your-project-id",
  solana: {
    createOnLogin: true, // Creates Solana accounts
  },
}
await initialize(config);

```

When `solana.createOnLogin` is set to `true`, the SDK will:

1.  Create a Solana account for new users
2.  The Solana account will be available on the `solanaAccounts` property

#### Deferred Account Creation

You can omit `createOnLogin` entirely to prevent automatic account creation and instead create accounts manually when needed:

```
const config: Config = {
  projectId: "your-project-id",
  // No ethereum or solana createOnLogin configuration
}
await initialize(config);

```

When `createOnLogin` is omitted, the SDK will:

1.  Not create any accounts automatically upon user login
2.  Require manual account creation using the account creation actions (see below)
3.  Give you full control over when and what types of accounts to create

#### Multi-Account Support

Users can have multiple accounts of each type:

-   Up to 10 EVM EOA accounts
-   Up to 10 Solana accounts
-   Up to 10 EVM Smart Accounts (each EVM EOA can own one Smart Account)

**Using Account Objects (Recommended):** The SDK provides rich account objects with additional metadata like creation timestamps and owner relationships:

```
import { getCurrentUser } from "@coinbase/cdp-core";
const user = await getCurrentUser();
// Access all EVM EOA accounts with metadata
if (user?.evmAccountObjects) {
  user.evmAccountObjects.forEach((account, index) => {
    console.log(`EVM Account ${index + 1}:`, account.address);
    console.log(`Created:`, new Date(account.createdAt).toLocaleDateString());
  });
}
// Access all Solana accounts with metadata
if (user?.solanaAccountObjects) {
  user.solanaAccountObjects.forEach((account, index) => {
    console.log(`Solana Account ${index + 1}:`, account.address);
    console.log(`Created:`, new Date(account.createdAt).toLocaleDateString());
  });
}
// Access all EVM Smart Accounts with owner information
if (user?.evmSmartAccountObjects) {
  user.evmSmartAccountObjects.forEach((account, index) => {
    console.log(`Smart Account ${index + 1}:`, account.address);
    console.log(`Owners:`, account.ownerAddresses.join(', '));
    console.log(`Created:`, new Date(account.createdAt).toLocaleDateString());
  });
}

```

**Legacy Account Arrays (Deprecated):** For backward compatibility, the SDK still provides simple address arrays, but these are deprecated:

```
// ⚠️ DEPRECATED - Use evmAccountObjects instead
const evmAddresses = user?.evmAccounts; // string[]
// ⚠️ DEPRECATED - Use solanaAccountObjects instead
const solanaAddresses = user?.solanaAccounts; // string[]
// ⚠️ DEPRECATED - Use evmSmartAccountObjects instead
const smartAccounts = user?.evmSmartAccounts; // string[]

```

### Sign In a User

You’re now ready to start calling the APIs provided by the package! The following code signs in an end user:

```
import { signInWithEmail, verifyEmailOTP } from "@coinbase/cdp-core";
// Send an email to user@example.com with a One Time Password (OTP).
const authResult = await signInWithEmail({
  email: "user@example.com"
});
// Input the OTP sent to user@example.com.
const verifyResult = await verifyEmailOTP({
  flowId: authResult.flowId,
  otp: "123456", // Hardcoded for convenience here.
});
// Get the authenticated end user.
const user = verifyResult.user;

```

### Link Additional Authentication Methods

Once a user is authenticated, you can link additional authentication methods to their account. This allows users to sign in using multiple methods (email, SMS, OAuth providers) with the same embedded wallet.

#### Link an Email Address

```
import { linkEmail, verifyEmailOTP } from "@coinbase/cdp-core";
// User must be signed in first
const result = await linkEmail("additional-email@example.com");
// Verify the OTP sent to the email
await verifyEmailOTP({
  flowId: result.flowId,
  otp: "123456"
});

```

#### Link a Phone Number

```
import { linkSms, verifySmsOTP } from "@coinbase/cdp-core";
// User must be signed in first
const result = await linkSms("+14155552671");
// Verify the OTP sent via SMS
await verifySmsOTP({
  flowId: result.flowId,
  otp: "123456"
});

```

#### Link a Google Account

```
import { linkGoogle } from "@coinbase/cdp-core";
// User must be signed in first
// This initiates the OAuth flow to link a Google account
await linkGoogle();
// The user will be redirected to Google for authentication
// After successful authentication, the Google account will be linked

```

#### Link an Apple Account

```
import { linkApple } from "@coinbase/cdp-core";
// User must be signed in first
await linkApple();

```

#### Link Any OAuth Provider

```
import { linkOAuth } from "@coinbase/cdp-core";
// User must be signed in first
// Link a Google account
await linkOAuth("google");
// Link an Apple account
await linkOAuth("apple");

```

### Sign In with Custom Authentication

If you’re using a third-party identity provider (Auth0, Firebase, AWS Cognito, or any OIDC-compliant provider), you can authenticate users with JWTs from your provider.

#### Prerequisites

Before using custom authentication:

1.  **Configure your identity provider in the CDP Portal**:
    -   Navigate to [Embedded Wallet Configuration](https://portal.cdp.coinbase.com/products/embedded-wallets)
    -   Click on the Custom auth tab
    -   Add your JWKS endpoint URL (e.g., `https://your-domain.auth0.com/.well-known/jwks.json`)
    -   Configure your JWT issuer and audience
2.  **Provide a `customAuth.getJwt` callback** when initializing the SDK:

```
import { initialize, authenticateWithJWT } from "@coinbase/cdp-core";
await initialize({
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
});

```

#### Authenticate a User

Once configured, call `authenticateWithJWT()` to authenticate the user:

```
import { authenticateWithJWT, getCurrentUser } from "@coinbase/cdp-core";
// After your user has signed in to your IDP (Auth0, Firebase, etc.)
const result = await authenticateWithJWT();
console.log("User authenticated:", result.user);
console.log("Is new user:", result.isNewUser);
// The user is now signed in and wallets are created based on your config
const user = await getCurrentUser();
if (user?.evmAccounts?.[0]) {
  console.log("EVM Address:", user.evmAccountObjects[0]?.address);
}

```

#### How it Works

1.  Your user signs in to your identity provider (Auth0, Firebase, Cognito, etc.)
2.  You call `authenticateWithJWT()` which internally calls your `customAuth.getJwt` callback
3.  The SDK sends the JWT to CDP’s backend, which validates it against your configured JWKS
4.  If valid, the user is authenticated and wallets are auto-created based on your configuration
5.  The `customAuth.getJwt` callback is called automatically whenever the SDK needs a fresh token

### View User Information

Once the end user has signed in, you can display their information in your application:

```
import { getCurrentUser, isSignedIn } from "@coinbase/cdp-core";
// Check if user is signed in
const signedIn = await isSignedIn();
if (signedIn) {
  // Get the user's information
  const user = await getCurrentUser();
  console.log("User ID:", user.userId);
  // Display different account types based on configuration
  if (user.evmAccountObjects?.length > 0) {
    console.log("EVM Accounts (EOAs):", user.evmAccountObjects);
  }
  if (user.evmSmartAccountObjects?.length > 0) {
    console.log("EVM Smart Accounts:", user.evmSmartAccountObjects);
  }
  if (user.solanaAccountObjects?.length > 0) {
    console.log("Solana Accounts:", user.solanaAccountObjects);
  }
  // Find the user's email address (if they logged in with email/otp)
  const email = user.authenticationMethods.email?.email;
  console.log("Email Address:", email);
}

```

### Multi-Factor Authentication

The SDK supports two methods of multi-factor authentication to add an extra layer of security to your application:

-   **TOTP (Time-based One-Time Password)**: Users enroll using authenticator apps like Google Authenticator or Authy
-   **SMS**: Users receive verification codes via text message to their phone number

> **Important**: Users must be authenticated (signed in) before they can enroll in MFA or perform MFA verification.

#### MFA Enrollment Flow

The enrollment flow consists of two steps:

1.  **Initiate enrollment** - For TOTP: generate a secret and QR code. For SMS: send OTP to phone number
2.  **Submit enrollment** - Verify the user’s code to complete enrollment

##### TOTP Enrollment

```
import {
  initiateMfaEnrollment,
  submitMfaEnrollment,
  getCurrentUser
} from "@coinbase/cdp-core";
// Step 1: Initiate TOTP MFA enrollment (user must be signed in)
const enrollment = await initiateMfaEnrollment({
  mfaMethod: "totp"
});
// Display QR code for user to scan with their authenticator app
console.log("Scan this QR code URL:", enrollment.authUrl);
// Or display the secret for manual entry
console.log("Or enter this secret manually:", enrollment.secret);
// Step 2: After user adds to their authenticator app, verify with the 6-digit code
const result = await submitMfaEnrollment({
  mfaMethod: "totp",
  mfaCode: "123456" // The 6-digit code from the user's authenticator app
});
// After successful enrollment, the user object is updated with MFA information
console.log("MFA enrolled for user:", result.user.userId);
console.log("TOTP enrollment info:", result.user.mfaMethods?.totp);
// Output: { enrolledAt: "2024-01-01T00:00:00Z" }

```

##### SMS Enrollment

```
import {
  initiateMfaEnrollment,
  submitMfaEnrollment
} from "@coinbase/cdp-core";
// Step 1: Initiate SMS MFA enrollment (user must be signed in)
// Phone number must be in E.164 format (e.g., +14155552671)
const enrollment = await initiateMfaEnrollment({
  mfaMethod: "sms",
  phoneNumber: "+14155552671"  // Can be any phone number, doesn't have to match auth phone
});
console.log("SMS sent:", enrollment.success);
// User receives SMS with 6-digit code
// Step 2: Submit the code received via SMS
const result = await submitMfaEnrollment({
  mfaMethod: "sms",
  mfaCode: "654321" // The 6-digit code from SMS
});
console.log("SMS MFA enrolled for user:", result.user.userId);
console.log("SMS enrollment info:", result.user.mfaMethods?.sms);
// Output: { enrolledAt: "2024-01-16T10:30:00Z" }
// Check current user's MFA methods
const user = await getCurrentUser();
console.log("User MFA methods:", user.mfaMethods);

```

#### MFA Verification Flow

When performing sensitive operations that require MFA verification, use the verification flow:

##### TOTP Verification

```
import {
  initiateMfaVerification,
  submitMfaVerification
} from "@coinbase/cdp-core";
// Step 1: Initiate TOTP MFA verification (user must be signed in and enrolled)
await initiateMfaVerification({
  mfaMethod: "totp"
});
// Step 2: Submit the 6-digit code from the user's authenticator app
await submitMfaVerification({
  mfaMethod: "totp",
  mfaCode: "654321" // The current 6-digit code from the authenticator app
});
// MFA verification successful - user can now perform sensitive operations
console.log("TOTP verification completed");

```

##### SMS Verification

```
import {
  initiateMfaVerification,
  submitMfaVerification
} from "@coinbase/cdp-core";
// Step 1: Initiate SMS MFA verification (user must be signed in and enrolled)
// This sends a 6-digit code to the user's enrolled phone number
await initiateMfaVerification({
  mfaMethod: "sms"
});
console.log("SMS sent to enrolled phone number");
// User receives SMS with 6-digit code
// Step 2: Submit the 6-digit code from SMS
await submitMfaVerification({
  mfaMethod: "sms",
  mfaCode: "123456" // The code received via SMS
});
// MFA verification successful - user can now perform sensitive operations
console.log("SMS verification completed");

```

#### Complete Example: MFA Setup and Usage

```
import {
  initialize,
  signInWithEmail,
  verifyEmailOTP,
  initiateMfaEnrollment,
  submitMfaEnrollment,
  initiateMfaVerification,
  submitMfaVerification,
  getCurrentUser,
  signEvmTransaction
} from "@coinbase/cdp-core";
// Initialize the SDK
await initialize({
  projectId: "your-project-id"
});
// Sign in the user first
const { flowId } = await signInWithEmail({ 
  email: "user@example.com" 
});
const { user } = await verifyEmailOTP({
  flowId,
  otp: "123456"
});
// Check if user has MFA enabled
if (!user.mfaMethods?.totp) {
  // Enroll in MFA
  const enrollment = await initiateMfaEnrollment({ 
    mfaMethod: "totp" 
  });
  
  // Show QR code to user (in a real app, you'd display this as an actual QR code)
  console.log("Please scan this QR code with your authenticator app:");
  console.log(enrollment.authUrl);
  
  // Get the code from user input
  const mfaCode = prompt("Enter the 6-digit code from your authenticator app:");
  
  // Complete enrollment
  const result = await submitMfaEnrollment({
    mfaMethod: "totp",
    mfaCode
  });
  
  console.log("MFA successfully enabled!");
  console.log("Enrolled at:", result.user.mfaMethods.totp.enrolledAt);
}
// Later, when performing a sensitive operation that requires MFA...
try {
  // Attempt the operation
  await signEvmTransaction({ /* ... */ });
} catch (error) {
  // If MFA is required, the operation will fail
  // Initiate MFA verification
  await initiateMfaVerification({ 
    mfaMethod: "totp" 
  });
  
  // Get MFA code from user
  const verificationCode = prompt("Enter your 6-digit MFA code:");
  
  // Submit verification
  await submitMfaVerification({
    mfaMethod: "totp",
    mfaCode: verificationCode
  });
  
  // Retry the operation after successful MFA verification
  await signEvmTransaction({ /* ... */ });
}

```

#### Handling Multiple MFA Methods

When users have multiple MFA methods enrolled, use helper functions to check enrollment status and let users choose their preferred method:

```
import {
  getCurrentUser,
  getEnrolledMfaMethods,
  isEnrolledInMfa,
  initiateMfaVerification,
  submitMfaVerification
} from "@coinbase/cdp-core";
const user = await getCurrentUser();
// Get all enrolled MFA methods
const methods = getEnrolledMfaMethods(user);
console.log("Enrolled methods:", methods);
// Output: ['totp', 'sms'] or ['totp'] or ['sms'] or []
// Check if user has any MFA enrolled
if (isEnrolledInMfa(user)) {
  console.log("User has MFA enabled");
}
// Check if user has specific method enrolled
if (isEnrolledInMfa(user, "totp")) {
  console.log("User has TOTP enabled");
}
if (isEnrolledInMfa(user, "sms")) {
  console.log("User has SMS enabled");
}
// When user has multiple methods, let them choose
if (methods.length > 1) {
  console.log("Please choose your MFA method:");
  methods.forEach((method, index) => {
    console.log(`${index + 1}. ${method.toUpperCase()}`);
  });
  // Get user's choice (in a real app, this would be a UI selection)
  const choice = prompt("Enter 1 for TOTP or 2 for SMS:");
  const selectedMethod = methods[parseInt(choice) - 1];
  // Use the selected method for verification
  await initiateMfaVerification({ mfaMethod: selectedMethod });
  const code = prompt(`Enter your ${selectedMethod.toUpperCase()} code:`);
  await submitMfaVerification({ mfaMethod: selectedMethod, mfaCode: code });
}

```

#### Phone Number Validation

When enrolling in SMS MFA, phone numbers must be in E.164 format. The SDK provides strict validation:

```
import { validatePhoneNumber } from "@coinbase/cdp-core";
try {
  // Valid formats
  validatePhoneNumber("+14155552671");  // US
  validatePhoneNumber("+442071838750"); // UK
  validatePhoneNumber("+81312345678");  // Japan
  console.log("Phone number is valid");
} catch (error) {
  // Invalid formats will throw an error
  console.error(error.message);
  // Example: "Invalid phone number format: '4155552671'. Phone number must be in E.164 format..."
}
// Common invalid formats:
// validatePhoneNumber("4155552671");       // Missing +
// validatePhoneNumber("+1-415-555-2671");  // Contains hyphens
// validatePhoneNumber("+1 415 555 2671");  // Contains spaces
// validatePhoneNumber("+0412345678");      // Starts with 0

```

#### Check MFA Configuration

Check whether MFA is enabled for your project and view configuration settings:

```
import { getMfaConfig } from "@coinbase/cdp-core";
// Retrieve MFA configuration for the current project
const config = await getMfaConfig();
console.log("MFA enabled:", config.enabled);
console.log("TOTP enabled:", config.totpConfig.enabled);
console.log("SMS enabled:", config.smsConfig?.enabled);
console.log("Configuration created:", config.createdAt);
console.log("Last updated:", config.updatedAt);
// Example response:
// {
//   projectId: "your-project-id",
//   enabled: true,
//   totpConfig: { enabled: true },
//   smsConfig: { enabled: true },
//   createdAt: "2024-01-01T00:00:00Z",
//   updatedAt: "2024-01-15T10:30:00Z"
// }

```

#### Track MFA Enrollment Prompts

Record when users are shown the MFA enrollment prompt to track enrollment opportunities:

```
import { recordMfaEnrollmentPrompted, getCurrentUser } from "@coinbase/cdp-core";
// When showing the MFA enrollment prompt to a user (must be signed in)
const result = await recordMfaEnrollmentPrompted();
console.log("Enrollment prompt recorded at:", result.enrollmentPromptedAt);
// The user's state is updated with the prompt timestamp
const user = await getCurrentUser();
console.log("Last prompted:", user.mfaMethods?.enrollmentPromptedAt);
// Output: "2024-01-15T10:30:00Z"
// This helps track when users skip MFA enrollment so you can
// decide when to prompt them again

```

**Use Cases for Tracking Enrollment Prompts:**

-   Track when users skip MFA enrollment
-   Implement smart re-prompting logic (e.g., don’t prompt again for 30 days)
-   Analyze MFA adoption rates
-   Identify users who have been prompted but haven’t enrolled

### Create Accounts Manually

If you configured your SDK without `createOnLogin`, you can manually create accounts for authenticated users when needed. This gives you full control over when accounts are created.

#### Create an EVM EOA Account

```
import { createEvmEoaAccount, getCurrentUser } from "@coinbase/cdp-core";
// User must be signed in first
const user = await getCurrentUser();
if (!user.evmAccountObjects?.length) {
  // Create an EVM EOA (Externally Owned Account)
  const evmAddress = await createEvmEoaAccount();
  console.log("Created EVM EOA:", evmAddress);
  // The user object is automatically updated
  const updatedUser = await getCurrentUser();
  console.log("User now has EVM EOA:", updatedUser.evmAccountObjects[0]?.address);
}

```

**Note:** `createEvmEoaAccount()` will throw an error if the user already has an EVM EOA account.

#### Create an EVM Smart Account

```
import { createEvmSmartAccount, getCurrentUser } from "@coinbase/cdp-core";
// User must be signed in first
const user = await getCurrentUser();
if (!user.evmSmartAccountObjects?.length) {
  // Create a Smart Account (a new EOA will be created as the owner)
  const smartAccountAddress = await createEvmSmartAccount();
  console.log("Created Smart Account:", smartAccountAddress);
  // The user object is automatically updated
  const updatedUser = await getCurrentUser();
  console.log("User now has Smart Account:", updatedUser.evmSmartAccountObjects[0]?.address);
  console.log("And EOA (used as owner):", updatedUser.evmAccountObjects[0]?.address);
}

```

**Note:** By default, `createEvmSmartAccount()` always creates a new EOA to serve as the Smart Account owner. You can also enable spend permissions when creating a Smart Account:

```
import { createEvmSmartAccount } from "@coinbase/cdp-core";
// Create Smart Account with spend permissions enabled
const smartAccountAddress = await createEvmSmartAccount({
  enableSpendPermissions: true
});
console.log("Created Smart Account with spend permissions:", smartAccountAddress);

```

**Using a Specific Owner:** You can specify an existing EOA address to use as the owner instead of creating a new one:

```
import { createEvmSmartAccount, getCurrentUser } from "@coinbase/cdp-core";
const user = await getCurrentUser();
const existingEoaAddress = user.evmAccountObjects[0]?.address;
// Create Smart Account with a specific owner
// Note: The owner must not already control another smart account for this user
const smartAccountAddress = await createEvmSmartAccount({
  owner: existingEoaAddress,
  enableSpendPermissions: false
});
console.log("Created Smart Account with specific owner:", smartAccountAddress);

```

#### Create a Solana Account

```
import { createSolanaAccount, getCurrentUser } from "@coinbase/cdp-core";
// User must be signed in first
const user = await getCurrentUser();
if (!user.solanaAccountObjects?.length) {
  // Create a Solana account
  const solanaAddress = await createSolanaAccount();
  console.log("Created Solana account:", solanaAddress);
  // The user object is automatically updated
  const updatedUser = await getCurrentUser();
  console.log("User now has Solana account:", updatedUser.solanaAccountObjects[0]?.address);
}

```

**Note:** `createSolanaAccount()` will throw an error if the user already has a Solana account.

### Send an EVM Transaction

We support signing and sending an EVM transaction in a single call on the following networks:

-   Base
-   Base Sepolia
-   Ethereum
-   Ethereum Sepolia
-   Avalanche
-   Arbitrum
-   Optimism
-   Polygon

```
import { sendEvmTransaction, getCurrentUser } from "@coinbase/cdp-core";
const user = await getCurrentUser();
const evmAccount = user.evmAccountObjects[0]?.address;
const result = await sendEvmTransaction({
  evmAccount,
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
console.log("Transaction Hash:", result.transactionHash);

```

For EVM networks other than those supported by the CDP APIs, your end user must sign the transaction, and then you must broadcast the transaction yourself. This example uses the public client from `viem` to broadcast the transaction.

```
import { signEvmTransaction, getCurrentUser } from "@coinbase/cdp-core";
import { http, createPublicClient } from "viem";
import { tron } from "viem/chains";
const user = await getCurrentUser();
const evmAccount = user.evmAccountObjects[0]?.address;
// Sign the transaction
const { signedTransaction } = await signEvmTransaction({
  evmAccount,
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
// Broadcast signed transaction to non-Base chain
const client = createPublicClient({
  chain: tron,
  transport: http()
});
const hash = await client.sendRawTransaction({
  serializedTransaction: signedTransaction
});

```

### Smart Account Operations

Smart Accounts provide advanced account abstraction features, including user operations and paymaster support.

#### Create Spend Permissions

Spend permissions allow Smart Accounts to delegate spending authority to other accounts within specified limits and time periods. This enables use cases like subscription payments, automated DeFi strategies, and automatic topping up of AI agent funds.

```
import { createSpendPermission, getCurrentUser } from "@coinbase/cdp-core";
const user = await getCurrentUser();
const smartAccount = user.evmSmartAccountObjects[0]?.address;
const result = await createSpendPermission({
  evmSmartAccount: smartAccount,
  network: "base-sepolia",
  spender: "0x742D35Cc6634C0532925a3b8D6Ec6F1C2b9c1E46", // Address that can spend tokens
  token: "eth", // Token symbol ("eth", "usdc") or contract address
  allowance: "1000000000000000000", // 1 ETH in wei
  period: 86400, // 24 hours in seconds
  start: new Date(), // Start time (optional, defaults to now)
  end: new Date(Date.now() + 86400 * 30 * 1000), // End time (optional, defaults to no expiration)
  useCdpPaymaster: true, // Use CDP paymaster for gas sponsorship
});
console.log("User Operation Hash:", result.userOperationHash);

```

You can also use `periodInDays` for a more human-friendly API:

```
const result = await createSpendPermission({
  evmSmartAccount: smartAccount,
  network: "base-sepolia",
  spender: "0x742D35Cc6634C0532925a3b8D6Ec6F1C2b9c1E46",
  token: "usdc", // USDC token
  allowance: "10000000", // 10 USDC (6 decimals)
  periodInDays: 7, // Weekly recurring allowance
  useCdpPaymaster: true
});

```

#### List Spend Permissions

Retrieve all spend permissions for a Smart Account:

```
import { listSpendPermissions, getCurrentUser } from "@coinbase/cdp-core";
const user = await getCurrentUser();
const smartAccount = user.evmSmartAccountObjects[0]?.address;
const result = await listSpendPermissions({
  evmSmartAccount: smartAccount,
  network: "base-sepolia",
  pageSize: 10
});
console.log("Found", result.spendPermissions.length, "spend permissions");
for (const permission of result.spendPermissions) {
  console.log("Permission:", permission.permissionHash, "Revoked:", permission.revoked);
  console.log("Spender:", permission.permission.spender);
  console.log("Token:", permission.permission.token);
  console.log("Allowance:", permission.permission.allowance);
}
// Paginate through results if needed
if (result.hasNextPage) {
  const nextPage = await listSpendPermissions({
    evmSmartAccount: smartAccount,
    network: "base-sepolia",
    pageToken: result.nextPageToken
  });
}

```

#### Revoke Spend Permissions

Revoke a spend permission for a Smart Account:

```
import { revokeSpendPermission, getCurrentUser } from "@coinbase/cdp-core";
const user = await getCurrentUser();
const smartAccount = user.evmSmartAccountObjects[0]?.address;
const result = await revokeSpendPermission({
  evmSmartAccount: smartAccount,
  network: "base-sepolia",
  permissionHash: "0x5678...",
  useCdpPaymaster: true
});
console.log("User Operation Hash:", result.userOperationHash);

```

### Sign a Solana Transaction

When your application is configured with `solana: { createOnLogin: true }`, you can sign Solana transactions:

```
import { signSolanaTransaction, getCurrentUser } from "@coinbase/cdp-core";
const user = await getCurrentUser();
const solanaAccount = user.solanaAccountObjects[0]?.address;
const result = await signSolanaTransaction({
  solanaAccount,
  transaction: "base64-encoded-solana-transaction"  // Your Solana transaction here
});
console.log("Signed Transaction:", result.signedTransaction);
// The signedTransaction can now be broadcast to the Solana network

```

### Sign a Solana Message

You can also sign arbitrary messages with Solana accounts:

```
import { signSolanaMessage, getCurrentUser } from "@coinbase/cdp-core";
const user = await getCurrentUser();
const solanaAccount = user.solanaAccountObjects[0]?.address;
const message = Buffer.from("Hello, Solana!", "utf8").toString("base64");
const result = await signSolanaMessage({
  solanaAccount,
  message // Base64 encoded message to sign
});
console.log("Message Signature:", result.signature);
// The signature can be used for authentication or verification purposes

```

### Send a Solana Transaction

You can sign and send a Solana transaction in a single call on the following Solana networks:

-   Solana Mainnet
-   Solana Devnet

```
import { sendSolanaTransaction, getCurrentUser } from "@coinbase/cdp-core";
const user = await getCurrentUser();
const solanaAccount = user.solanaAccountObjects[0]?.address;
const result = await sendSolanaTransaction({
  solanaAccount,
  network: "solana-devnet", // or "solana" for mainnet
  transaction: "base64-encoded-solana-transaction"  // Your Solana transaction here
});
console.log("Transaction Signature:", result.transactionSignature);
// The transaction has been broadcast to the Solana network

```

#### Send User Operations

Send user operations from a Smart Account:

```
import { sendUserOperation, getCurrentUser } from "@coinbase/cdp-core";
const user = await getCurrentUser();
const smartAccount = user.evmSmartAccountObjects[0]?.address;
const result = await sendUserOperation({
  evmSmartAccount: smartAccount,
  network: "base-sepolia",
  calls: [
    {
      to: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
      value: 1000000000000000000n, // 1 ETH in wei
      data: "0x", // Optional contract interaction data
    }
  ],
  // Optional paymaster for gas sponsorship. Get your free Base paymaster URL [from the CDP Portal](https://portal.cdp.coinbase.com/products/node).
  paymasterUrl: "https://paymaster.example.com",
});
console.log("User Operation Hash:", result.userOperationHash);

```

**Transaction Attribution with EIP-8021:** You can add attribution data to user operations for tracking app usage and revenue sharing:

```
import { sendUserOperation, getCurrentUser } from "@coinbase/cdp-core";
const user = await getCurrentUser();
const smartAccount = user.evmSmartAccountObjects[0]?.address;
// EIP-8021 data suffix for "baseapp" attribution (Schema ID 0)
// Format: {codes}{codesLength}{schemaId}{ercSuffix}
const dataSuffix = "0xdddddddd62617365617070070080218021802180218021802180218021";
const result = await sendUserOperation({
  evmSmartAccount: smartAccount,
  network: "base-sepolia",
  calls: [
    {
      to: "0xTargetContract",
      value: 0n,
      data: "0x",
    }
  ],
  dataSuffix, // Attribution data appended to callData
  useCdpPaymaster: true,
});
console.log("User Operation Hash:", result.userOperationHash);
// The dataSuffix will be appended to the user operation's callData for onchain attribution

```

The `dataSuffix` is a hex-encoded string that follows the [EIP-8021 standard](https://eip.tools/eip/8021) for transaction attribution. It allows apps to track usage and enable revenue sharing mechanisms. The suffix is automatically appended to the user operation’s `callData` before submission.

#### Get User Operation Status

After sending a user operation, you can get its status and retrieve the result:

```
import { getUserOperation } from "@coinbase/cdp-core";
// Get the status of a user operation
const userOperationResult = await getUserOperation({
  userOperationHash: result.userOperationHash,
  evmSmartAccount: smartAccount,
  network: "base-sepolia"
});
console.log("Status:", userOperationResult.status); // "pending", "complete", or "failed"
if (userOperationResult.status === "complete") {
  console.log("Transaction Hash:", userOperationResult.transactionHash);
  console.log("Block Number:", userOperationResult.receipts?.[0]?.blockNumber);
} else if (userOperationResult.status === "failed") {
  console.log("Failure reason:", userOperationResult.receipts?.[0]?.revert?.message);
}

```

### Sign Messages and Typed Data

End users can sign EVM messages, hashes, and typed data to generate signatures for various onchain applications.

```
import { signEvmMessage, signEvmTypedData, getCurrentUser } from "@coinbase/cdp-core";
const user = await getCurrentUser();
const evmAccount = user.evmAccountObjects[0]?.address;
// Sign a message
const messageResult = await signEvmMessage({
  evmAccount,
  message: "Hello World"
});
// Sign typed data (EIP-712)
const typedDataResult = await signEvmTypedData({
  evmAccount,
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
      wallet: evmAccount
    }
  }
});

```

### Export Private Keys

End users can export their private keys from their embedded wallet, allowing them to import them into compatible wallets of their choice.

#### Secure Iframe Export (Recommended)

The secure iframe approach is the recommended way to export private keys. It creates a secure iframe that copies the private key directly to the user’s clipboard without ever exposing it to your application’s JavaScript context.

##### Export EVM Private Key via Iframe

```
import { createEvmKeyExportIframe } from "@coinbase/cdp-core";
const container = document.getElementById("key-export-container");
const { cleanup } = await createEvmKeyExportIframe({
  address: "0x1234...",
  target: container,
  projectId: "your-project-id",
  label: "Copy Private Key",
  onStatusUpdate: (status, message) => {
    if (status === "success") {
      console.log("Key copied to clipboard!");
    } else if (status === "error") {
      console.error("Error:", message);
    }
  }
});
// The iframe will auto-cleanup when the session expires, or you can manually cleanup:
// cleanup();

```

##### Export Solana Private Key via Iframe

```
import { createSolanaKeyExportIframe } from "@coinbase/cdp-core";
const container = document.getElementById("key-export-container");
const { cleanup } = await createSolanaKeyExportIframe({
  address: "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM",
  target: container,
  projectId: "your-project-id",
  label: "Copy Private Key",
  onStatusUpdate: (status, message) => {
    if (status === "success") {
      console.log("Key copied to clipboard!");
    } else if (status === "error") {
      console.error("Error:", message);
    }
  }
});

```

#### Direct Export (Deprecated)

> **⚠️ Deprecated:** The direct export functions expose the private key to your application’s JavaScript context and will be removed soon. Use the secure iframe approach above instead.

##### Export EVM Private Key (Deprecated)

```
import { exportEvmAccount, getCurrentUser } from "@coinbase/cdp-core";
const user = await getCurrentUser();
const evmAccount = user.evmAccountObjects[0]?.address;
const { privateKey } = await exportEvmAccount({
  evmAccount
});
// WARNING: Handle private keys with extreme care!
console.log("EVM Private Key:", privateKey);

```

##### Export Solana Private Key (Deprecated)

When your application is configured with `solana: { createOnLogin: true }`, you can export Solana private keys:

```
import { exportSolanaAccount, getCurrentUser } from "@coinbase/cdp-core";
const user = await getCurrentUser();
const solanaAccount = user.solanaAccountObjects[0]?.address;
const { privateKey } = await exportSolanaAccount({
  solanaAccount
});
// WARNING: Handle private keys with extreme care!
console.log("Solana Private Key:", privateKey);

```

### X402 Payment Protocol Support

The SDK includes built-in support for the X402 payment protocol, which enables HTTP requests with micropayments. This allows accessing paid APIs and services that require payment for each request.

#### Installation

Ensure you have separately installed the `x402-fetch` package:

#### Basic Usage

The `fetchWithX402` function provides a wrapped fetch API that automatically handles X402 payment requests:

```
import { fetchWithX402, getCurrentUser } from "@coinbase/cdp-core";
// The user must be authenticated first
const user = await getCurrentUser();
// Create a fetch function with X402 payment handling
const { fetchWithPayment } = fetchWithX402();
// Make a request to an X402-protected resource
try {
  const response = await fetchWithPayment("https://api.example.com/paid-endpoint", {
    method: "GET",
    headers: {
      "Content-Type": "application/json"
    }
  });
  
  const data = await response.json();
  console.log("Paid API response:", data);
} catch (error) {
  console.error("X402 payment failed:", error);
}

```

#### Advanced Configuration

You can customize the X402 behavior with options:

```
// Use a specific address for payments (instead of the user's default)
const { fetchWithPayment } = fetchWithX402({
  address: "0x1234567890123456789012345678901234567890"
});
// Use a custom fetch implementation
const customFetch = (url, options) => {
  console.log("Making request to:", url);
  return fetch(url, options);
};
const { fetchWithPayment } = fetchWithX402({
  fetch: customFetch
});

```

#### How It Works

1.  When you make a request to an X402-protected resource, the server responds with a `402 Payment Required` status
2.  The wrapped fetch function automatically:
    -   Extracts payment details from the server’s response
    -   Creates and signs a payment transaction using the user’s wallet
    -   Includes the payment proof in a retry request
3.  The server validates the payment and returns the requested resource

#### Smart Account Support

By default, `fetchWithX402` will use the user’s Smart Account if available, falling back to their regular EVM account:

```
const user = await getCurrentUser();
console.log("Using account:", user.evmSmartAccountObjects?.[0] || user.evmAccountObjects?.[0]);
// This will automatically use the appropriate account type
const { fetchWithPayment } = await fetchWithX402();

```

#### Solana Support

Solana is supported out of the box with `fetchWithX402`. If your end user has both an EVM and Solana account, the EVM account will be used by default. You can pass a Solana address to `fetchWithX402` to use the Solana account instead.

```
const user = await getCurrentUser();
const { fetchWithPayment } = fetchWithX402({
  address: user.solanaAccountObjects[0]?.address
});

```

### EIP-1193 Provider

The core package includes an EIP-1193 compatible provider. This provider can be used to sign and send transactions. The provider is created by calling `createCDPEmbeddedWallet`, which exposes a `.provider` attribute. `createCDPEmbeddedWallet` must be called with the desired chains to support as well as the transports for these chains. The provider will initially connect to the first chain in the `chains` array. The transports are typically HTTP RPC endpoints, which are used internally for broadcasting non-Base transactions. For more information on transports, see [Wagmi’s `createConfig` setup](https://wagmi.sh/react/api/createConfig).

```
import { base, mainnet } from "viem/chains";
import { http } from "viem"
// Basic usage with default configuration
const wallet = createCDPEmbeddedWallet({
  chains:[base, mainnet],
  transports: {
    [base.id]: http(),
    [mainnet.id]: http()
  }
});
const provider = wallet.provider;
// Request account access
const accounts = await provider.request({
  method: "eth_requestAccounts"
});
// Sign a message
const signature = await provider.request({
  method: "personal_sign",
  params: ["Hello, World!", accounts[0]]
});
// Send a transaction
const txHash = await provider.request({
  method: "eth_sendTransaction",
  params: [{
    from: accounts[0],
    to: "0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b6",
    value: "0x1000000000000000000"
  }]
});
// Listen for connection events
provider.on("connect", (connectInfo) => {
  console.log("Connected to chain:", connectInfo.chainId);
});
provider.on("disconnect", () => {
  console.log("Disconnected from wallet");
});

```

### Viem Accounts

The core package includes a `toViemAccount` utility function that enables wrapping an embedded wallet into a Viem account compatible interface. This allows the account to act as a drop-in replacement for any library or framework that accepts Viem accounts.

```
import { toViemAccount, getCurrentUser } from "@coinbase/cdp-core";
import { createWalletClient } from "viem";
import { mainnet } from "viem/chains";
import { http } from "viem";
const user = await getCurrentUser();
const evmAccount = user.evmAccountObjects[0]?.address;
const viemAccount = toViemAccount(evmAccount);
const client = createWalletClient({
  account: viemAccount,
  transport: http("https://example.com"),
  chain: mainnet,
});

```