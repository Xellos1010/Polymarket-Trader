# overview

EVMSolana

## Overview

Multi-Factor Authentication (MFA) adds an extra layer of security to Embedded Wallets by requiring users to verify their identity through a secondary authentication method.

These guides will walk you through how to integrate Coinbase Developer Platform’s MFA into your application. For more detailed customization options, see the [SDK reference](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks).

## How it works

## Configuration

Configure which MFA methods (TOTP, SMS, or both) are available for your project in the [CDP Portal](https://portal.cdp.coinbase.com/products/embedded-wallets/authentication). You can update these settings at any time; changes apply to new enrollments only and don’t affect users who are already enrolled.

## MFA-protected operations

The following actions automatically trigger MFA verification when the user is enrolled:

## Supported authenticator apps

Any TOTP-compatible authenticator app works. These are commonly used:

-   [Google Authenticator](https://play.google.com/store/apps/details?id=com.google.android.apps.authenticator2)
-   [Microsoft Authenticator](https://play.google.com/store/apps/details?id=com.azure.authenticator)
-   [Authy](https://authy.com/)
-   [1Password](https://1password.com/)
-   [Duo Mobile](https://duo.com/product/multi-factor-authentication-mfa/duo-mobile-app)

## Quickstart

If you’re using `@coinbase/cdp-react`, use the pre-built enrollment component to let users enable MFA:

```
import { EnrollMfaModal } from "@coinbase/cdp-react";
function Settings() {
  return (
    <EnrollMfaModal onEnrollSuccess={() => console.log("MFA enabled!")}>
      <button>Enable Two-Factor Authentication</button>
    </EnrollMfaModal>
  );
}

```

Once enrolled in MFA, users see a verification modal whenever they attempt a [protected operation](https://developer.chrome.com/embedded-wallets/mfa/protected-operations):

```
import { CDPReactProvider } from "@coinbase/cdp-react";
import { useSendEvmTransaction } from "@coinbase/cdp-hooks";
function App() {
  return (
    <CDPReactProvider config={{ projectId: "your-project-id" }}>
      <SendButton />
    </CDPReactProvider>
  );
}
function SendButton() {
  const { sendEvmTransaction, isPending } = useSendEvmTransaction();
  const handleSend = async () => {
    // If user is enrolled in MFA, modal appears automatically
    const hash = await sendEvmTransaction({
      to: "0x...",
      value: "1000000000000000000",
    });
  };
  return <button onClick={handleSend} disabled={isPending}>Send</button>;
}

```

That’s it! The SDK handles:

1.  Detecting when MFA verification is needed
2.  Showing the verification modal
3.  Completing the operation after successful verification

## What to read next