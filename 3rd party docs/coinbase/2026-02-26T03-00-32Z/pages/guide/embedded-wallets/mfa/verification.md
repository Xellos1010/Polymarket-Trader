# verification

EVMSolana

After users enroll in MFA, they’ll see a prompt to enter their code whenever they perform sensitive operations like signing transactions or exporting keys. This page covers how to handle those prompts.

## Choose your approach

### Ready-to-use components (recommended)

[`@coinbase/cdp-react`](https://developer.chrome.com/embedded-wallets/react-components) provides ready-to-use components with a polished UI.

-   Modal
    
-   Programmatic
    
-   Inline
    

```
import { VerifyMfaModal } from "@coinbase/cdp-react";
function App() {
  return (
    <VerifyMfaModal
      onSuccess={() => console.log("Verified!")}
      onCancel={() => console.log("Cancelled")}
    >
      <button>Verify MFA</button>
    </VerifyMfaModal>
  );
}

```

Use `useVerifyMfaModal` when you need to trigger verification from code. Call `open()` to show the modal; it closes automatically on success.

```
import { useVerifyMfaModal } from "@coinbase/cdp-react";
function ProtectedAction() {
  const { open } = useVerifyMfaModal({
    onSuccess: () => {
      console.log("Verified!");
      performSensitiveAction();
    },
    onCancel: () => console.log("Cancelled"),
  });
  return <button onClick={open}>Perform Sensitive Action</button>;
}

```

Use `VerifyMfaInline` for step-by-step flows like wallet export where you want verification embedded in the page rather than a modal.

```
import { VerifyMfaInline, VerifyMfaInlineFlow } from "@coinbase/cdp-react";
function ExportWalletWithMfa() {
  return (
    <VerifyMfaInline
      verifyFirst
      onVerified={() => console.log("MFA complete")}
    >
      <VerifyMfaInlineFlow>
        <MyProtectedContent />
      </VerifyMfaInlineFlow>
    </VerifyMfaInline>
  );
}

```

### Custom UIs using React Hooks

For custom UI or SMS verification, use hooks from [`@coinbase/cdp-hooks`](https://developer.chrome.com/embedded-wallets/react-hooks). Verification is a two-step process:

1.  **Initiate** — Call `useInitiateMfaVerification` to start verification (for SMS, this sends a new code)
2.  **Submit** — Call `useSubmitMfaVerification` with the 6-digit code to complete verification

-   TOTP
    
-   SMS
    
-   Multiple methods
    

```
import { useState } from "react";
import { useInitiateMfaVerification, useSubmitMfaVerification } from "@coinbase/cdp-hooks";
function TotpVerification({ onSuccess }: { onSuccess: () => void }) {
  const [showInput, setShowInput] = useState(false);
  const { initiateMfaVerification } = useInitiateMfaVerification();
  const { submitMfaVerification } = useSubmitMfaVerification();
  async function startVerification() {
    // Step 1: Initiate — prepares for TOTP verification
    await initiateMfaVerification({ mfaMethod: "totp" });
    setShowInput(true);
  }
  async function completeVerification(code: string) {
    // Step 2: Submit — user enters code from authenticator app
    await submitMfaVerification({ mfaMethod: "totp", mfaCode: code });
    onSuccess(); // Retry the original operation
  }
  // Render: code input and verify button
}

```

```
import { useState } from "react";
import { useInitiateMfaVerification, useSubmitMfaVerification } from "@coinbase/cdp-hooks";
function SmsVerification({ onSuccess }: { onSuccess: () => void }) {
  const [codeSent, setCodeSent] = useState(false);
  const { initiateMfaVerification } = useInitiateMfaVerification();
  const { submitMfaVerification } = useSubmitMfaVerification();
  async function startVerification() {
    // Step 1: Initiate — sends SMS to enrolled phone number
    await initiateMfaVerification({ mfaMethod: "sms" });
    setCodeSent(true);
  }
  async function completeVerification(code: string) {
    // Step 2: Submit — user enters code from SMS
    await submitMfaVerification({ mfaMethod: "sms", mfaCode: code });
    onSuccess(); // Retry the original operation
  }
  // Render: send button, then code input + verify button
}

```

```
import { getEnrolledMfaMethods } from "@coinbase/cdp-core";
import { useCurrentUser } from "@coinbase/cdp-hooks";
function MfaMethodSelector() {
  const { currentUser } = useCurrentUser();
  
  // Check which methods user has enrolled
  const enrolledMethods = getEnrolledMfaMethods(currentUser);
  // Returns: ["totp"], ["sms"], or ["totp", "sms"]
  // Render: method selector if multiple, then appropriate verification UI
}

```

### Direct API calls for non-React

For non-React applications, use functions from [`@coinbase/cdp-core`](https://developer.chrome.com/embedded-wallets/quickstart).

-   TOTP
    
-   SMS
    

```
import { initiateMfaVerification, submitMfaVerification } from "@coinbase/cdp-core";
// Step 1: Initiate — prepares for TOTP verification
await initiateMfaVerification({ mfaMethod: "totp" });
// Step 2: Submit — user enters code from authenticator app
await submitMfaVerification({ mfaMethod: "totp", mfaCode: "123456" });
// Retry the original operation

```

```
import { initiateMfaVerification, submitMfaVerification } from "@coinbase/cdp-core";
// Step 1: Initiate — sends SMS to enrolled phone number
await initiateMfaVerification({ mfaMethod: "sms" });
// Step 2: Submit — user enters code from SMS
await submitMfaVerification({ mfaMethod: "sms", mfaCode: "123456" });
// Retry the original operation

```

## Handling MFA errors

When a sensitive operation requires MFA, it throws an error with code `MFA_REQUIRED`:

```
import { signEvmTransaction } from "@coinbase/cdp-core";
try {
  await signEvmTransaction({ /* ... */ });
} catch (error) {
  if (error.code === "MFA_REQUIRED") {
    // Show MFA verification UI
    // After verification, retry the operation
  }
}

```

With React hooks, you can detect this and show a verification UI:

```
const handleOperation = async () => {
  try {
    await signEvmTransaction({ /* ... */ });
  } catch (error) {
    if (error.code === "MFA_REQUIRED") {
      setShowMfaModal(true);
    }
  }
};

```

## Troubleshooting