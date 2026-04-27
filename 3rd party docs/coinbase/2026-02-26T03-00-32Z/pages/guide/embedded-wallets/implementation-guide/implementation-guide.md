# implementation guide

EVMSolana

## Overview

There are three ways to implement authentication in your application, each offering different levels of customization and control:

1.  **`AuthButton` component from `@coinbase/cdp-react`**: Pre-built UI component (fastest integration)
2.  **React hooks from `@coinbase/cdp-hooks`**: For custom React UIs with state management
3.  **Direct methods from `@coinbase/cdp-core`**: For vanilla JavaScript/TypeScript or non-React frameworks

## AuthButton component (simplest)

For the fastest integration, `@coinbase/cdp-react` provides a pre-built `AuthButton` component that handles the entire authentication flow with a single line of code.

## React hooks

For React applications, `@coinbase/cdp-hooks` provides convenient hooks that handle state management and re-renders automatically.

## Direct methods

The `@coinbase/cdp-core` package provides the low-level authentication primitives for maximum control over the user experience. This approach is ideal for non-React applications or when you need fine-grained control.

OAuth authentication in React Native requires configuring a deep link callback URL:

The SDK uses `expo-web-browser` to automatically handle the OAuth redirect and callback. You don’t need to add `Linking.addEventListener` or other deep link handling code.

## Server-side validation

Some developers take additional action (fetching additional data, starting asynchronous processes) based on a user having an active session. For security reasons, it is important that you check authentication status by validating the access token Coinbase grants a user when they log in.

## What to read next

-   **[Authentication Methods](https://developer.chrome.com/embedded-wallets/authentication-methods)**: Learn about email OTP, SMS OTP, and social login options
-   **[React Hooks](https://developer.chrome.com/embedded-wallets/react-hooks)**: Pre-built hooks for authentication and wallet management
-   **[React Components](https://developer.chrome.com/embedded-wallets/react-components)**: Ready-to-use UI components including AuthButton
-   **[Session Management](https://developer.chrome.com/embedded-wallets/session-management)**: Understand session lifecycle and token management
-   **[Best Practices](https://developer.chrome.com/embedded-wallets/best-practices)**: Security recommendations and choosing the right approach