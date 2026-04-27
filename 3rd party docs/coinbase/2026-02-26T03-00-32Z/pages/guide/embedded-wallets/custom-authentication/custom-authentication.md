# custom authentication

EVMSolana

## Overview

Custom authentication enables applications with existing authentication systems to integrate Embedded Wallets seamlessly. Instead of using CDP’s built-in authentication methods (email OTP, SMS, OAuth), you can use JSON Web Tokens (JWTs) from your own Identity Provider. This approach is ideal when:

-   You already have users authenticated via Auth0, Firebase, AWS Cognito, or a custom solution
-   You want to implement single sign-on (SSO) across your entire platform
-   You need to integrate with corporate identity systems
-   You must use specific authentication providers for regulatory compliance

## How it works

Custom authentication follows a straightforward flow that leverages your existing identity infrastructure:

## Prerequisites

Before implementing custom authentication, ensure your Identity Provider supports:

-   **JWKS (JSON Web Key Sets)** with **RS256 or ES256** signing algorithms
-   **Required JWT claims**:
    -   `iss` (issuer): Your Identity Provider’s domain
    -   `sub` (subject): Unique identifier that identifies the particular user on your application (default, but configurable - see [User Identification](#user-identification))
    -   `exp` (expiration): Token expiration timestamp
    -   `iat` (issued at): Token issuance timestamp

## 1\. Choose how to identify users

CDP uses a specific claim from your JWT to uniquely identify users and associate them with their embedded wallets. By default, CDP uses the standard `sub` (subject) claim, but you can configure an arbitrary JWT claim to serve as the user identifier.

### Default claim

If you don’t specify a custom user identifier claim, CDP will use the JWT’s `sub` claim:

```
{
  "iss": "https://example.auth0.com/",
  "sub": "auth0|1234567890",  // Used as user identifier by default
  "aud": ["test-app"],
  "exp": 1700000000,
  "iat": 1699996400
}

```

### Custom claim

You can configure CDP to use an arbitrary claim in your JWT as the user identifier. Common alternatives include:

### Best practices

-   Use the default `sub` claim unless you have a specific reason to use a different claim
-   Avoid claims that users can change (like display names or mutable usernames)
-   Test your configuration thoroughly before going to production
-   Valid claim name examples: `email`, `username`, `user_id`, `external_id`
-   Invalid claim name examples: `Email` (uppercase), `user-id` (dash), `user.id` (dot)

## 2\. Configure CDP Portal

To configure custom authentication in the CDP Portal:

## 3\. Integrate the SDK

The CDP Frontend SDK provides built-in support for custom authentication through the `customAuth` configuration option.

### React

For React applications, use `CDPHooksProvider` or `CDPReactProvider` with the `customAuth` configuration:

### Non-React

For vanilla JavaScript/TypeScript or other frameworks, use the `initialize` method from `@coinbase/cdp-core`:

## 4\. Implement auth flow

Once you’ve configured custom authentication, you need to explicitly authenticate the user with CDP after they log in with your Identity Provider.

### Triggering auth

After your user logs in with your IDP (Auth0, Firebase, etc.), call `authenticateWithJWT()` to authenticate with CDP:

-   **First-time users**: CDP will create a new embedded wallet for the user (based on the configured user identifier claim, defaulting to `sub`)
-   **Returning users**: CDP will retrieve their existing wallet (using the same user identifier claim value)

```
import { useAuthenticateWithJWT } from '@coinbase/cdp-hooks';
import { useAuth0 } from '@auth0/auth0-react';
function LoginComponent() {
  const { authenticateWithJWT, isLoading } = useAuthenticateWithJWT();
  const { loginWithRedirect, isAuthenticated } = useAuth0();
  const handleLogin = async () => {
    // First, log in with your IDP (Auth0 in this example)
    await loginWithRedirect();
  };
  // After Auth0 login completes, authenticate with CDP
  React.useEffect(() => {
    if (isAuthenticated) {
      authenticateWithJWT()
        .then(() => console.log('Successfully authenticated with CDP'))
        .catch((error) => console.error('CDP authentication failed:', error));
    }
  }, [isAuthenticated, authenticateWithJWT]);
  return (
    <button onClick={handleLogin} disabled={isLoading}>
      {isLoading ? 'Authenticating...' : 'Sign In'}
    </button>
  );
}

```

### Accessing wallet data

Once authenticated with CDP, you can access the user’s wallet:

```
import { useCurrentUser, useEvmAddress } from '@coinbase/cdp-hooks';
function WalletComponent() {
  const { currentUser } = useCurrentUser();
  const { evmAddress } = useEvmAddress();
  if (!currentUser) {
    return <p>Please sign in with your account</p>;
  }
  return (
    <div>
      <p>User ID: {currentUser.id}</p>
      <p>Wallet Address: {evmAddress}</p>
    </div>
  );
}

```

### Monitoring auth state

Use the `useIsSignedIn` hook to monitor authentication state:

```
import { useIsSignedIn } from '@coinbase/cdp-hooks';
function App() {
  const { isSignedIn } = useIsSignedIn();
  return (
    <div>
      {isSignedIn ? (
        <p>You are signed in</p>
      ) : (
        <p>Please sign in</p>
      )}
    </div>
  );
}

```

## 5\. Test and debug

### Verify JWT structure

Before integrating with CDP, verify your JWT contains the required claims:

1.  **Obtain a JWT** from your Identity Provider
2.  **Decode it** using [jwt.io](https://jwt.io/) or similar tool
3.  **Verify claims**:
    -   `iss`: Matches your configured issuer
    -   `aud`: Matches your configured audience
    -   `exp`: Token expiration is in the future
    -   `iat`: Token issuance timestamp
    -   **User identifier claim**: Contains a unique, stable user ID
        -   If using default: Verify `sub` claim exists
        -   If using custom claim: Verify your configured claim (e.g., `email`, `user_id`) exists and is non-empty

Example JWT payload with default `sub` claim:

```
{
  "iss": "https://example.auth0.com/",
  "sub": "auth0|1234567890",
  "aud": ["test-app"],
  "exp": 1700000000,
  "iat": 1699996400
}

```

Example JWT payload with custom `email` claim as user identifier:

```
{
  "iss": "https://example.auth0.com/",
  "sub": "auth0|1234567890",
  "email": "user@example.com",
  "aud": ["test-app"],
  "exp": 1700000000,
  "iat": 1699996400
}

```

### Validate JWKS endpoint

Ensure your JWKS endpoint is accessible and returns valid keys:

1.  **Access your JWKS URL** in a browser (e.g., `https://example.auth0.com/.well-known/jwks.json`)
2.  **Verify the response** contains public keys

## Session management

Custom authentication sessions differ from CDP’s built-in authentication in several key ways:

### Token lifecycle

-   **Managed by your Identity Provider**: Session duration, token expiration, and refresh are controlled by your Identity Provider
-   **Always fresh**: CDP calls `getJwt` whenever it needs authentication, ensuring tokens are always current
-   **No CDP refresh**: CDP does not store or refresh tokens, it relies entirely on your `getJwt` callback
-   **Maximum TTL**: JWTs must have an expiration time (`exp`) within 7 days from issuance

### Sign out

When using custom authentication, signing out from your Identity Provider is sufficient:

## Complete example

For a complete working implementation of custom authentication with Auth0, see our example application **[React Custom Auth Demo App](https://github.com/coinbase/cdp-wallet-demo-apps/tree/main/apps/react-custom-auth)**.

## What to read next

-   **[Authentication Methods](https://developer.chrome.com/embedded-wallets/authentication-methods)**: Overview of all authentication options
-   **[Session Management](https://developer.chrome.com/embedded-wallets/session-management)**: Understanding session lifecycle with custom auth
-   **[Implementation Guide](https://developer.chrome.com/embedded-wallets/implementation-guide)**: General authentication implementation patterns
-   **[Security Configuration](https://developer.chrome.com/embedded-wallets/domains)**: Configure domain allowlisting
-   **[Best Practices](https://developer.chrome.com/embedded-wallets/best-practices)**: Security recommendations and production readiness