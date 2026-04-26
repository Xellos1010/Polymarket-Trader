# best practices

EVMSolana

## Overview

Follow these best practices to build secure, production-ready applications with Embedded Wallets. This guide covers security recommendations, state management patterns, user experience tips, and a production readiness checklist.

## Security recommendations

1.  **Domain allowlisting**: Always configure your [allowed domains](https://developer.chrome.com/embedded-wallets/domains) in CDP Portal
2.  **HTTPS only**: Never use embedded wallets on non-HTTPS sites in production
3.  **Rate limiting**: Implement rate limiting on your authentication endpoints
4.  **Session management**: Use appropriate session timeouts for your use case

## State management

1.  **Always check authentication state**: Before starting any authentication flow, verify if the user is already signed in:
    
    ```
    const user = await getCurrentUser();
    if (user) {
      // User is already authenticated
      return;
    }
    
    ```
    
2.  **Implement sign out**: Provide a clear way for users to sign out:
    
    ```
    import { signOut } from '@coinbase/cdp-core';
    // Sign out the current user
    await signOut();
    
    ```
    
    For React applications, use the `AuthButton` component which handles sign out automatically, or the `useSignOut` hook:
    
    ```
    import { useSignOut } from '@coinbase/cdp-hooks';
    function SignOutButton() {
      const { signOut } = useSignOut();
      return <button onClick={signOut}>Sign Out</button>;
    }
    
    ```
    
3.  **Avoid redundant verification**: Don’t call `verifyEmailOTP` or `verifySmsOTP` when a user is already authenticated. This will result in an error and may leave your application in an inconsistent state.

## User experience tips

1.  **Clear messaging**: Explain why users need to verify their email
2.  **Error handling**: Provide helpful error messages for common issues
3.  **Loading states**: Show progress during authentication steps
4.  **Success feedback**: Confirm when authentication is complete

## Choosing the right approach

When to use the AuthButton component

Use the AuthButton when:

-   You want the fastest possible integration
-   You’re building a React application
-   You don’t need custom authentication UI
-   You want a production-ready solution out of the box
-   You’re prototyping or building an MVP

When to use React hooks

Use `@coinbase/cdp-hooks` when:

-   You need custom authentication UI
-   You want to control the authentication flow step-by-step
-   You need to integrate with existing form components
-   You want fine-grained control over error handling
-   You’re building a custom authentication experience

When to use direct methods

Use `@coinbase/cdp-core` when:

-   Building vanilla JavaScript/TypeScript applications
-   Working with frameworks other than React (Vue, Angular, Svelte, etc.)
-   You need maximum control over the authentication flow
-   Building server-side applications
-   Creating custom authentication wrappers for any framework

## What to read next

-   **[Authentication Methods](https://developer.chrome.com/embedded-wallets/authentication-methods)**: Learn about available authentication options
-   **[Implementation Guide](https://developer.chrome.com/embedded-wallets/implementation-guide)**: Step-by-step authentication integration
-   **[Session Management](https://developer.chrome.com/embedded-wallets/session-management)**: Understand session lifecycle and token management
-   **[Server-side validation](https://developer.chrome.com/embedded-wallets/implementation-guide#server-side-validation)**: Validate user sessions on your backend
-   **[Security Configuration](https://developer.chrome.com/embedded-wallets/domains)**: Configure domain allowlisting and security settings
-   **[React Components](https://developer.chrome.com/embedded-wallets/react-components)**: Ready-to-use UI components including AuthButton