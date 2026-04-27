# auth method linking

## Overview

Auth method linking enables users to associate multiple authentication methods with a single embedded wallet account. This allows users to sign in using different methods (email, SMS, OAuth) while maintaining access to the same wallet and user identity.

## Why link authentication methods?

By default, each authentication method creates a separate user identity. For example, if a user signs in with their email and later signs in with their phone number, they would have two different embedded wallets. Auth method linking solves this problem by allowing users to:

-   **Access their wallet using multiple methods**: Sign in with email, phone, or social providers interchangeably.
-   **Meet 2FA requirements**: For applications that require 2FA, Embedded Wallets provide a smooth integration.
-   **Improve account security**: Add additional authentication factors as users accumulate more funds.
-   **Enhance account recovery**: Multiple methods provide backup options if one method becomes unavailable.

## Supported authentication methods

You can link any combination of the following authentication methods to a single user account:

-   Email OTP
-   SMS OTP
-   All supported OAuth providers

## Security features

Auth method linking maintains the same security standards as initial authentication:

## Implementation examples

### Using React components

#### Basic usage

Render a `LinkAuth` component with an `onLinkSuccess` handler:

```
import { LinkAuth } from '@coinbase/cdp-react';
function ManageAuthMethods() {
  return (
    <LinkAuth
      onLinkSuccess={(method) => {
        console.log(`Successfully linked ${method}`);
      }}
    />
  );
}

```

#### Accessing state

Display a message based on the LinkAuth state:

```
import {
  LinkAuth,
  LinkAuthError,
  LinkAuthFlow,
  LinkAuthFlowBackButton,
  LinkAuthTitle,
} from '@coinbase/cdp-react';
function App() {
  return (
    <LinkAuth>
      {(state) => (
        <>
          <div className="header">
            <LinkAuthTitle />
            <LinkAuthFlowBackButton />
          </div>
          <div className="message">
            {state.methodToLink ? (
              <span>Currently linking: {state.methodToLink}</span>
            ) : (
              <span>Select a method to link</span>
            )}
          </div>
          <div className="error">
            <LinkAuthError />
          </div>
          <LinkAuthFlow />
        </>
      )}
    </LinkAuth>
  );
}

```

#### Advanced example: Using SignInModal

Customize LinkAuthItems to show a modal for non-OAuth methods instead of transitioning in place:

```
import {
  LinkAuth,
  LinkAuthItems,
  SignInModal,
  SignInModalTrigger,
  useLinkAuthFlow,
  useAppConfig,
  type AuthMethod,
} from '@coinbase/cdp-react';
import { useCallback, useMemo, useState } from 'react';
function CustomLinkAuthItems() {
  const { link, back } = useLinkAuthFlow();
  const { authMethods } = useAppConfig();
  const [openModal, setOpenModal] = useState<AuthMethod | null>(null);
  const modalMethods = useMemo(
    () => authMethods.filter(method => !method.startsWith([REDACTED]),
    [authMethods],
  );
  const handleClose = useCallback(() => {
    setOpenModal(null);
    back();
  }, [back, setOpenModal]);
  const handleLink = useCallback(
    (method: AuthMethod) => {
      link(method);
      if (!method.startsWith([REDACTED])) {
        setOpenModal(method);
      }
    },
    [link, setOpenModal],
  );
  return (
    <>
      <LinkAuthItems onLink={handleLink} />
      {modalMethods.map(method => {
        return (
          <SignInModal
            key={method}
            open={openModal === method}
            authMethods={[method]}
            setIsOpen={isOpen => (isOpen ? setOpenModal(method) : handleClose())}
            onSuccess={() => setOpenModal(null)}
          >
            <SignInModalTrigger>null</SignInModalTrigger>
          </SignInModal>
        );
      })}
    </>
  );
}
function App() {
  return (
    <LinkAuth>
      <h2>Link a profile</h2>
      <CustomLinkAuthItems />
    </LinkAuth>
  );
}

```

### Using React hooks

#### Link an email address

Use the `useLinkEmail` hook to link an email address to the currently authenticated user. This follows the same two-step flow as email sign-in: initiate the flow and then verify the OTP.

```
import { useLinkEmail, useVerifyEmailOTP, useCurrentUser } from "@coinbase/cdp-hooks";
import { useState } from "react";
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

#### Link a phone number

Use the `useLinkSms` hook to link a phone number to the currently authenticated user. Like email linking, this requires OTP verification.

```
import { useLinkSms, useVerifySmsOTP, useCurrentUser } from "@coinbase/cdp-hooks";
import { useState } from "react";
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

#### Link a Google account

Use the `useLinkGoogle` hook to link a Google account to the currently authenticated user. This initiates the OAuth flow for Google authentication.

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

#### Link an Apple account

Use the `useLinkApple` hook to link an Apple account to the currently authenticated user. This initiates the OAuth flow for Apple authentication.

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
      // This initiates the OAuth flow to link an Apple account
      await linkApple();
      // The user will be redirected to Apple for authentication
      // After successful authentication, the Apple account will be linked
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

#### Link an X account

Use the `useLinkOAuth` hook to link an X account to the currently authenticated user. This initiates the OAuth flow for X authentication.

```
import { useLinkOAuth, useCurrentUser } from "@coinbase/cdp-hooks";
function LinkXAccount() {
  const { linkOAuth } = useLinkOAuth();
  const { currentUser } = useCurrentUser();
  const handleLinkX = async () => {
    if (!currentUser) {
      console.error("User must be signed in first");
      return;
    }
    try {
      // This initiates the OAuth flow to link an X account
      await linkOAuth("x");
      // The user will be redirected to X for authentication
      // After successful authentication, the X account will be linked
    } catch (error) {
      console.error("Failed to link X account:", error);
    }
  };
  return (
    <button onClick={handleLinkX} disabled={!currentUser}>
      Link X Account
    </button>
  );
}

```

#### Link a Telegram account

Use the `useLinkOAuth` hook to link a Telegram account to the currently authenticated user. This initiates the OAuth flow for Telegram authentication.

```
import { useLinkOAuth, useCurrentUser } from "@coinbase/cdp-hooks";
function LinkTelegramAccount() {
  const { linkOAuth } = useLinkOAuth();
  const { currentUser } = useCurrentUser();
  const handleLinkTelegram = async () => {
    if (!currentUser) {
      console.error("User must be signed in first");
      return;
    }
    try {
      // This initiates the OAuth flow to link a Telegram account
      await linkOAuth("telegram");
      // The user will be redirected to Telegram for authentication
      // After successful authentication, the Telegram account will be linked
    } catch (error) {
      console.error("Failed to link Telegram account:", error);
    }
  };
  return (
    <button onClick={handleLinkTelegram} disabled={!currentUser}>
      Link Telegram Account
    </button>
  );
}

```

#### Link any OAuth provider

Use the `useLinkOAuth` hook to link any supported OAuth provider to the currently authenticated user. This provides a unified interface for all OAuth providers.

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
  const handleLinkX = async () => {
    if (!currentUser) return;
    try {
      // Link an X account
      await linkOAuth("x");
    } catch (error) {
      console.error("Failed to link X account:", error);
    }
  };
  const handleLinkTelegram = async () => {
    if (!currentUser) return;
    try {
      // Link a Telegram account
      await linkOAuth("telegram");
    } catch (error) {
      console.error("Failed to link Telegram account:", error);
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
      <button onClick={handleLinkX} disabled={!currentUser}>
        Link X
      </button>
      <button onClick={handleLinkTelegram} disabled={!currentUser}>
        Link Telegram
      </button>
      <button onClick={handleLinkCoinbase} disabled={!currentUser}>
        Link Coinbase
      </button>
    </div>
  );
}

```

## User experience best practices

When implementing auth method linking, consider these UX recommendations:

## Error States

Common errors you may encounter when linking authentication methods:

Error

Description

`METHOD_ALREADY_LINKED`

The authentication method is already linked to this or another account.

`ACCOUNT_EXISTS`

The intended account to link already belongs to another user.

## What to read next

-   **[Authentication Methods](https://developer.chrome.com/embedded-wallets/authentication-methods)**: Learn about available authentication methods
-   **[React Hooks](https://developer.chrome.com/embedded-wallets/react-hooks)**: Comprehensive guide to CDP React hooks
-   **[Onramp Integration](https://developer.chrome.com/embedded-wallets/onramp/cross-platform)**: Integrate Coinbase Onramp with linked authentication
-   **[Session Management](https://developer.chrome.com/embedded-wallets/session-management)**: Understand how sessions work with linked methods
-   **[Best Practices](https://developer.chrome.com/embedded-wallets/best-practices)**: Security recommendations for production applications