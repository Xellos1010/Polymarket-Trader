# authentication methods

EVMSolana

## Overview

Embedded Wallets provide secure, user-friendly authentication methods that eliminate the complexity of traditional crypto wallets. Users can access their wallets through familiar authentication patterns like email one-time passwords (OTP), SMS, and social logins, without ever dealing with seed phrases or browser extensions.

## Email OTP

Email OTP is the primary authentication method for Embedded Wallets, providing a secure and familiar experience for users.

### Email Customization

By default, all emails are sent without customization. If you’d like to use a custom email template featuring your app’s name and logo, submit [this form](https://docs.google.com/forms/d/e/1FAIpQLSf6Ou2eRa37uE_skNkk8YFodzkSnomEbZzVgDiGkr26t_kGSg/viewform?usp=header), and we’ll get you set up within one business day. Emails are sent from “no-reply <[no-reply@info.coinbase.com](mailto:no-reply@info.coinbase.com)\>”; this field is not currently customizable.

## SMS OTP

SMS-based one-time passwords are available as an additional authentication method, providing users with more flexibility in how they access their wallets.

This feature is currently supported for phone numbers from the following countries - Antigua and Barbuda, Australia, Austria, Bahamas, Belgium, Brazil, Bulgaria, Canada, Colombia, Croatia, Cyprus, Czech Republic, Denmark, Dominican Republic, Estonia, Finland, France, Germany, Greece, Grenada, Guyana, Hungary, India, Indonesia, Ireland, Italy, Japan, Kenya, Latvia, Lithuania, Luxembourg, Malta, Mexico, Netherlands, Philippines, Poland, Portugal, Romania, Saint Vincent and the Grenadines, Singapore, Slovakia, Slovenia, South Korea, Spain, Suriname, Sweden, Switzerland, Turkey, United Arab Emirates, United Kingdom, United States. If you’d like to enable the feature in additional regions, reach out to us on [Discord](https://discord.com/invite/cdp).

Social login through Google, Apple, X, and Telegram are supported via our SDK using OAuth 2.0. We offer Coinbase-owned OAuth login, allowing users to recognize and trust Coinbase’s brand during the login process.

### Examples

Sign in with social providers using the OAuth flow. Note that the page from which the `signInWithOAuth` call occurs will be redirected back to after the user authenticates with their provider. The user will be automatically logged-in when `@coinbase/cdp-core` re-initializes.

## Auth method linking

Once a user is authenticated, you can enable them to link additional authentication methods to their account. This allows users to sign in using multiple methods (email, SMS, OAuth providers) while maintaining access to the same embedded wallet.

For detailed implementation examples and code snippets, see the [Auth Method Linking guide](https://developer.chrome.com/embedded-wallets/auth-method-linking).

## Custom authentication

Custom authentication enables applications with existing authentication systems to integrate Embedded Wallets seamlessly. Instead of using CDP’s built-in authentication (email OTP, SMS, OAuth), you can use JWTs from your own identity provider.

### Requirements

Your identity provider must:

-   Support JWKS (JSON Web Key Sets) with RS256 or ES256 signing
-   Provide required JWT claims: `iss`, `exp`, `iat`, and a user identifier claim (default: `sub`, or a custom claim you configure)

### Getting started

See the complete [Custom Authentication guide](https://developer.chrome.com/embedded-wallets/custom-authentication) for setup instructions and code examples.

## Multi-Factor Authentication (MFA)

Add an extra layer of security to your embedded wallets with Time-based One-Time Password (TOTP) multi-factor authentication. Users can enroll using popular authenticator apps like Google Authenticator, Authy, or 1Password.

### Getting started with MFA

See the complete [Multi-Factor Authentication guide](https://developer.chrome.com/embedded-wallets/multi-factor-authentication) for implementation details and code examples.

## What to read next

-   **[Implementation Guide](https://developer.chrome.com/embedded-wallets/implementation-guide)**: Step-by-step guide to implementing these authentication methods
-   **[Auth Method Linking](https://developer.chrome.com/embedded-wallets/auth-method-linking)**: Link multiple authentication methods to a single wallet
-   **[Session Management](https://developer.chrome.com/embedded-wallets/session-management)**: Understand session lifecycle and token management
-   **[Best Practices](https://developer.chrome.com/embedded-wallets/best-practices)**: Security recommendations and production readiness
-   **[Server-side validation](https://developer.chrome.com/embedded-wallets/implementation-guide#server-side-validation)**: Validate user sessions on your backend