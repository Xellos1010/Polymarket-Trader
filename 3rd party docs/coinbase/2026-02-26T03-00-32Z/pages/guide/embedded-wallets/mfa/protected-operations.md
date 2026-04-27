# protected operations

EVMSolana

## Overview

Some operations, like signing transactions or exporting keys, are **protected** by MFA. Knowing which operations trigger MFA helps you design better UX (like showing lock icons) and handle edge cases.

## MFA-protected operations

These operations trigger MFA verification when the user is enrolled:

## Checking MFA status

Use `isMfaRequired()` to check if the user needs to verify before protected operations:

```
import { isMfaRequired } from "@coinbase/cdp-core";
if (isMfaRequired()) {
  console.log("MFA verification required");
}

```

Use `isMfaProtectedAction()` to check if a specific action triggers MFA (useful for showing lock icons):

```
import { isMfaProtectedAction } from "@coinbase/cdp-core";
if (isMfaProtectedAction("signEvmTransaction")) {
  // Show a lock icon on the button
}

```

## Custom handling

For applications using the SDK directly (without `CDPReactProvider`), or if you want a custom MFA UI, register a listener to intercept MFA requirements.

-   React hooks
    
-   Core SDK
    

```
import { useRegisterMfaListener, useCancelMfaVerification } from "@coinbase/cdp-hooks";
function App() {
  const { cancelMfaVerification } = useCancelMfaVerification();
  // Register a global listener for MFA requirements
  useRegisterMfaListener(({ methods }) => {
    // methods: available MFA methods (e.g., ["totp", "sms"])
    // Show your custom MFA UI here
  });
  // To cancel (user closes modal without verifying):
  // cancelMfaVerification();
}

```

For scoped handling (different MFA UX in different parts of your app):

```
import { useRegisterMfaListener } from "@coinbase/cdp-hooks";
import { useRef } from "react";
function ExportSection() {
  const containerRef = useRef(null);
  // Only responds to MFA triggers from within this container
  useRegisterMfaListener(
    ({ methods }) => {
      // Show inline MFA UI for this section
    },
    { scope: containerRef }
  );
  return <div ref={containerRef}>{/* Export UI */}</div>;
}

```

```
import { registerMfaListener, cancelMfaVerification } from "@coinbase/cdp-core";
// Register a global MFA handler
const unregister = registerMfaListener(({ methods }) => {
  // methods: available MFA methods (e.g., ["totp", "sms"])
  // Show your custom MFA UI here
});
// To cancel (user closes modal without verifying):
cancelMfaVerification();
// Clean up when done:
unregister();

```

## Disabling defaults

To handle MFA entirely yourself, disable the automatic modal at the provider level:

```
<CDPReactProvider
  config={{
    projectId: "your-project-id",
    mfa: {
      disableAutoPrompt: true,
    },
  }}
>
  <App />
</CDPReactProvider>

```

Some components like `ExportWalletModal` have built-in MFA flows. To handle MFA separately, use `skipMfa`:

```
<ExportWalletModal
  address={evmAccount}
  skipMfa={true}
/>

```

## Troubleshooting