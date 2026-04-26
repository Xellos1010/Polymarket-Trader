# domains

## Overview

The Coinbase Developer Platform (CDP) [Portal](https://portal.cdp.coinbase.com/) requires you configure which domains are authorized to access CDP APIs. These domains are configured using Cross-Origin Resource Sharing (CORS), ensuring your users are protected while maintaining a seamless experience.

More on CORS

CORS (Cross-Origin Resource Sharing) is a browser security mechanism that controls access between different web origins. An origin is defined by the combination of protocol (http/https), domain, and port.By default, browsers enforce the **same-origin policy**, blocking requests between different origins for security. CORS provides a way to safely relax this restriction:

-   **Without CORS**: Your website at `https://myapp.com` cannot access APIs at `https://api.cdp.coinbase.com`
-   **With CORS**: The API server explicitly allows specific origins, enabling secure cross-origin communication

Learn more about CORS fundamentals in the [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS).

By properly configuring your embedded wallet domains, you create a secure boundary that ensures only authorized applications can access our APIs, preventing malicious websites from exploiting your wallet integration, and protecting your users from cross-site scripting attacks.

## Example

Let’s walk through a practical example:

1.  A dapp at `https://app.developer.com` wants to send a POST request to `https://api.cdp.coinbase.com/embedded-wallet-api/projects/{projectId}` (e.g., to create a wallet).
2.  When Coinbase Developer Platform (CDP) receives the request, it will look up the list of allowed domains for the given project ID.
3.  CDP queries its database and sees that the developer has configured `https://app.developer.com` as an allowed domain for the project.
4.  CDP responds to the API with the following header set, allowing the response to return successfully:

```
Access-Control-Allow-Origin: https://app.developer.com

```

## How to configure domains

## What to read next

-   **[Quickstart Guide](https://developer.chrome.com/embedded-wallets/quickstart)**: Build your first embedded wallet app in under 10 minutes
-   **[React Hooks Reference](https://developer.chrome.com/embedded-wallets/react-hooks)**: Learn about available hooks like `useSignInWithEmail`, `useEvmAddress`, `useSendSolanaTransaction`, and more
-   **[React Components Guide](https://developer.chrome.com/embedded-wallets/react-components)**: Explore pre-built components for authentication, wallet management, and transactions