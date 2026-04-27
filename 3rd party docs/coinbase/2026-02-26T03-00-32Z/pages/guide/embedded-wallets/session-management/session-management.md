# session management

EVMSolana

## Overview

Understanding how user sessions work with Embedded Wallets is crucial for building secure applications with proper authentication flows. Sessions are managed through a dual-token system designed to balance security and user experience.

## Session duration

User sessions are managed through a dual-token system:

-   **Maximum session length**: 7 days
-   **Access token expiry**: 15 minutes
-   **Refresh token expiry**: 7 days

## How session tokens work

When a user successfully authenticates, they receive:

1.  **Access token**: Short-lived (15 minutes) token used for API requests
2.  **Refresh token**: Long-lived (7 days) token used to obtain new access tokens

The refresh token automatically generates new access tokens as needed, providing seamless authentication for up to 7 days. After 7 days, users must re-authenticate.

## Session lifecycle

## Implementation considerations

-   Monitor authentication state using `onAuthStateChange()` to handle session expiration
-   Implement graceful fallback when tokens expire
-   Consider showing session timeout warnings to users approaching the 7-day limit
-   Test your application’s behavior when refresh tokens expire

## Sign out functionality

Always provide a clear way for users to sign out using the `signOut()` method from `@coinbase/cdp-core` or the `AuthButton` component which handles sign out automatically. For React applications, you can also use the `useSignOut` hook:

```
import { useSignOut } from '@coinbase/cdp-hooks';
function SignOutButton() {
  const { signOut } = useSignOut();
  return <button onClick={signOut}>Sign Out</button>;
}

```

## Custom authentication sessions

If you’re using [custom authentication](https://developer.chrome.com/embedded-wallets/custom-authentication) with your own identity provider, session management works differently:

-   **Token lifecycle**: Managed by your identity provider (Auth0, Firebase, etc.)
-   **No CDP token refresh**: CDP always requests a fresh JWT via the `getJwt` callback
-   **Session duration**: Controlled by your IDP’s configuration
-   **Sign out**: Only need to sign out from your IDP

For more details on custom authentication, see the [Custom Authentication guide](https://developer.chrome.com/embedded-wallets/custom-authentication).

## What to read next

-   **[Authentication Methods](https://developer.chrome.com/embedded-wallets/authentication-methods)**: Learn about available authentication options
-   **[Implementation Guide](https://developer.chrome.com/embedded-wallets/implementation-guide)**: Step-by-step authentication integration
-   **[Server-side validation](https://developer.chrome.com/embedded-wallets/implementation-guide#server-side-validation)**: Validate user sessions on your backend
-   **[Best Practices](https://developer.chrome.com/embedded-wallets/best-practices)**: Security recommendations and production readiness