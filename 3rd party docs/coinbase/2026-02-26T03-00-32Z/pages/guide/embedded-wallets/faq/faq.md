# faq

EVMSolana

## General

### What is an Embedded Wallet?

A self-custodial crypto wallet built into your app. Users sign in with email OTP instead of managing seed phrases while maintaining full control of their assets. Wallets are created in under 500ms.

### How is this different from MetaMask?

Embedded Wallets integrate directly into your app’s UI—no browser extensions or pop-ups. Users authenticate with email, SMS, or social login instead of seed phrases.

### Who controls the assets?

Users have full control. Embedded Wallets are self-custodial—only the user can access their private keys. Users can export keys anytime. Coinbase cannot access user funds.

### Can I support both embedded and external wallets?

Yes. You can support both Embedded Wallets and external wallets (like MetaMask) in the same application.

## Authentication

### What authentication methods are available?

-   **Email OTP**: 6-digit codes, expire after 10 minutes
-   **SMS OTP**: 6-digit codes for select countries, expire after 5 minutes
-   **Social login**: Google, Apple, X via OAuth 2.0
-   **Custom authentication**: Use JWTs from your own identity provider (Auth0, Firebase, Cognito, etc.)

### Which countries support SMS?

Brazil, Canada, Colombia, India, Kenya, Mexico, Philippines, United Arab Emirates, United Kingdom, United States of America. Contact [Discord](https://discord.com/invite/cdp) for additional regions.

### How long do sessions last?

Sessions last up to 7 days with automatic refresh. Access tokens expire after 15 minutes, refresh tokens after 7 days.

### Can I customize authentication?

Yes. Use the pre-built `AuthButton`, React hooks for custom UI, or direct methods from `@coinbase/cdp-core` for non-React frameworks. See [Implementation Guide](https://developer.chrome.com/embedded-wallets/implementation-guide).

### Is SMS authentication secure?

SMS is vulnerable to SIM swapping attacks. Email OTP is recommended as the primary method.

### What is custom authentication?

Custom authentication lets you use your existing identity provider (Auth0, Firebase, AWS Cognito, etc.) instead of CDP’s built-in auth methods. You configure your JWKS endpoint in CDP Portal, and CDP validates JWTs from your IDP to create/access wallets. See [Custom Authentication](https://developer.chrome.com/embedded-wallets/custom-authentication).

### Which identity providers work with custom authentication?

Any provider that supports JWKS with RS256 or ES256 signing. Your JWTs must include required claims: `iss`, `sub`, `exp`, `iat`.

## Security

### How secure are embedded wallets?

Cryptographic operations occur in secure, isolated environments. Device-specific keys are generated locally and never exposed to Coinbase. True self-custody with support for up to 5 devices. See [Security and compliance](https://developer.chrome.com/embedded-wallets/welcome#security-and-compliance).

### What is domain allowlisting?

Required security feature that prevents unauthorized use of your project credentials. Configure allowed domains in CDP Portal before production deployment. See [Security Configuration](https://developer.chrome.com/embedded-wallets/domains).

### Can users export private keys?

Yes. Users can export private keys for wallet migration to MetaMask, hardware wallets, or other applications. See [Security & Export](https://developer.chrome.com/embedded-wallets/security-export).

### Is HTTPS required?

Yes. HTTPS is required in production to protect credentials and prevent attacks.

### How do I validate sessions server-side?

Always validate access tokens on your backend for sensitive operations using the CDP SDK. See [Server-side validation](https://developer.chrome.com/embedded-wallets/implementation-guide#server-side-validation).

## Technical

### What networks are supported?

All EVM-compatible chains (Base, Ethereum, Arbitrum, Polygon, Optimism), Solana (mainnet and devnet), and testnets. See [Supported Networks](https://developer.chrome.com/get-started/supported-networks).

### How fast is wallet creation?

Under 500ms for instant user onboarding.

### Can I import existing addresses?

No. Embedded Wallets creates new addresses for security. Use [Server Wallets](https://developer.chrome.com/server-wallets/v2/introduction/welcome) for backend key import.

### What frameworks are supported?

React, React Native, and Next.js have full support. Other frameworks use `@coinbase/cdp-core` for vanilla JS/TS, Vue, Angular, Svelte, etc.

### Can I customize the UI?

Yes. Use pre-built components, theming options, or build completely custom UI with hooks. See [Theming](https://developer.chrome.com/embedded-wallets/theming).

### Are there rate limits?

Yes. Authentication endpoints have rate limiting. Implement additional rate limiting on your endpoints as best practice.

## Pricing & Features

### How much does it cost?

See [Pricing page](https://developer.chrome.com/embedded-wallets/pricing).

### What features are included?

User authentication, wallet creation, transaction signing (EVM and Solana), onramp/offramp, USDC Rewards (3.35% for US developers across Embedded Wallets including Smart Accounts, and Server Wallets), and multi-device support (up to 5).

### Can I use onramp/offramp?

Yes. Built-in onramp/offramp lets users buy crypto with fiat and cash out to bank accounts. See [Native ApplePay Onramp](https://developer.chrome.com/embedded-wallets/onramp/apple-pay) or [Cross-Platform Onramp](https://developer.chrome.com/embedded-wallets/onramp/cross-platform).

### What are USDC Rewards?

US developers earn 3.35% on USDC balances across CDP Embedded Wallets (including Smart Accounts) and Server Wallets. Built-in, no integration required. See [USDC Rewards](https://developer.chrome.com/embedded-wallets/usdc-rewards).

## Troubleshooting

### Why am I getting “User is already authenticated” errors?

This error occurs when calling `verifyEmailOTP` or `verifySmsOTP` while a user is already signed in. Always check authentication state before starting a new auth flow:

```
const user = await getCurrentUser();
if (user) {
  // User already authenticated, no need to verify
  return;
}

```

See [Best Practices](https://developer.chrome.com/embedded-wallets/best-practices#state-management).

### What causes invalid or expired OTP errors?

-   Email OTPs expire after 10 minutes
-   SMS OTPs expire after 5 minutes
-   User entered wrong code
-   Network delays prevented timely entry

Request a new OTP if the code has expired.

### Why are sessions expiring unexpectedly?

Sessions last up to 7 days with automatic token refresh. Unexpected expiration usually means:

-   User cleared browser storage/cookies
-   Multiple devices exceeded the 5-device limit (oldest session terminated)
-   User manually signed out on another device

Implement `onAuthStateChange()` to detect and handle session changes gracefully. See [Session Management](https://developer.chrome.com/embedded-wallets/session-management).

### Why is my domain being rejected?

Ensure your domain is allowlisted in CDP Portal. Common issues:

-   Domain not added to allowlist
-   Using HTTPS in Portal but HTTP locally (or vice versa)
-   Subdomain mismatch (added `app.example.com` but using `www.example.com`)
-   Localhost not configured for development

See [Security Configuration](https://developer.chrome.com/embedded-wallets/domains).

### How do I test in development?

1.  Add `localhost:3000` (or your dev port) to allowed domains in CDP Portal
2.  Use testnet networks: Base Sepolia, Ethereum Sepolia, Solana Devnet
3.  Get testnet funds from [Base Faucet](https://portal.cdp.coinbase.com/products/faucet)
4.  Test in incognito mode to simulate new users
5.  Test with multiple browsers to ensure cross-browser compatibility

### Where can I get help?

[Quickstart Guide](https://developer.chrome.com/embedded-wallets/quickstart) or [Discord](https://discord.com/invite/cdp).

## What to read next

-   **[Quickstart Guide](https://developer.chrome.com/embedded-wallets/quickstart)**: Get started with embedded wallets in under 10 minutes
-   **[Best Practices](https://developer.chrome.com/embedded-wallets/best-practices)**: Security recommendations and production readiness
-   **[Authentication Methods](https://developer.chrome.com/embedded-wallets/authentication-methods)**: Learn about available authentication options
-   **[Troubleshooting common issues](https://developer.chrome.com/embedded-wallets/best-practices#user-experience-tips)**: Error handling and UX guidance