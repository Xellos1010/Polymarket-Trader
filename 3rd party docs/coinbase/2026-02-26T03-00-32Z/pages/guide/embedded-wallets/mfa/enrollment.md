# enrollment

EVMSolana

MFA enrollment is a one-time setup process where users add an authenticator app or phone number to their account. Users can enroll in both methods for backup.

## Choose your approach

### Ready-to-use components (recommended)

[`@coinbase/cdp-react`](https://developer.chrome.com/embedded-wallets/react-components) provides ready-to-use components with a polished UI.

```
import { EnrollMfaModal } from "@coinbase/cdp-react";
function App() {
  return (
    <EnrollMfaModal onEnrollSuccess={() => console.log("Enrolled!")}>
      <button>Set up MFA</button>
    </EnrollMfaModal>
  );
}

```

### Custom UIs using React Hooks

For custom UI or SMS enrollment, use hooks from [`@coinbase/cdp-hooks`](https://developer.chrome.com/embedded-wallets/react-hooks). Enrollment is a two-step process:

1.  **Initiate** — Call `useInitiateMfaEnrollment` to start enrollment (returns QR code for TOTP, sends SMS for text message)
2.  **Submit** — Call `useSubmitMfaEnrollment` with the 6-digit code to complete enrollment

-   TOTP
    
-   SMS
    

```
import { useState } from "react";
import { useInitiateMfaEnrollment, useSubmitMfaEnrollment } from "@coinbase/cdp-hooks";
function TotpEnrollment() {
  const [qrCodeUrl, setQrCodeUrl] = useState<string | null>(null);
  const { initiateMfaEnrollment } = useInitiateMfaEnrollment();
  const { submitMfaEnrollment } = useSubmitMfaEnrollment();
  async function startEnrollment() {
    // Step 1: Initiate — returns QR code data
    const result = await initiateMfaEnrollment({ mfaMethod: "totp" });
    setQrCodeUrl(result.authUrl); // Display as QR code
    // result.secret → manual entry fallback
  }
  async function completeEnrollment(code: string) {
    // Step 2: Submit — user enters code from authenticator app
    await submitMfaEnrollment({ mfaMethod: "totp", mfaCode: code });
  }
  // Render: QR code from qrCodeUrl, input for code, submit button
}

```

```
import { useState } from "react";
import { useInitiateMfaEnrollment, useSubmitMfaEnrollment } from "@coinbase/cdp-hooks";
function SmsEnrollment() {
  const [codeSent, setCodeSent] = useState(false);
  const { initiateMfaEnrollment } = useInitiateMfaEnrollment();
  const { submitMfaEnrollment } = useSubmitMfaEnrollment();
  async function startEnrollment(phoneNumber: string) {
    // Step 1: Initiate — sends SMS to phone number (E.164 format)
    await initiateMfaEnrollment({ mfaMethod: "sms", phoneNumber });
    setCodeSent(true);
  }
  async function completeEnrollment(code: string) {
    // Step 2: Submit — user enters code from SMS
    await submitMfaEnrollment({ mfaMethod: "sms", mfaCode: code });
  }
  // Render: phone input, send button, then code input + verify button
}

```

### Direct API calls for non-React

For non-React applications, use functions from [`@coinbase/cdp-core`](https://developer.chrome.com/embedded-wallets/quickstart).

-   TOTP
    
-   SMS
    

```
import { initiateMfaEnrollment, submitMfaEnrollment } from "@coinbase/cdp-core";
// Step 1: Initiate — returns QR code data
const enrollment = await initiateMfaEnrollment({ mfaMethod: "totp" });
// enrollment.authUrl → use with any QR code library
// enrollment.secret → manual entry fallback
// Step 2: Submit — user enters code from authenticator app
await submitMfaEnrollment({ mfaMethod: "totp", mfaCode: "123456" });

```

```
import { initiateMfaEnrollment, submitMfaEnrollment } from "@coinbase/cdp-core";
// Step 1: Initiate — sends SMS to phone number (E.164 format)
await initiateMfaEnrollment({ mfaMethod: "sms", phoneNumber: "+14155552671" });
// Step 2: Submit — user enters code from SMS
await submitMfaEnrollment({ mfaMethod: "sms", mfaCode: "123456" });

```

## Checking enrollment status

Before prompting users to enroll, check if they already have MFA enabled.

-   React
    
-   Core SDK
    

```
import { useCurrentUser } from "@coinbase/cdp-hooks";
import { getEnrolledMfaMethods, isEnrolledInMfa } from "@coinbase/cdp-core";
function MyComponent() {
  const { currentUser } = useCurrentUser();
  if (!currentUser) return null;
  const hasMfa = isEnrolledInMfa(currentUser);
  const methods = getEnrolledMfaMethods(currentUser);
  // methods: ['totp'], ['sms'], ['totp', 'sms'], or []
  // Use hasMfa and methods to drive your UI
}

```

```
import { getCurrentUser, getEnrolledMfaMethods, isEnrolledInMfa } from "@coinbase/cdp-core";
const user = await getCurrentUser();
// Check if user has any MFA method enrolled
const hasMfa = isEnrolledInMfa(user);
// Get list of enrolled methods
const methods = getEnrolledMfaMethods(user);
// methods: ['totp'], ['sms'], ['totp', 'sms'], or []
// Check specific method
isEnrolledInMfa(user, "totp"); // true or false
isEnrolledInMfa(user, "sms");  // true or false

```

## Validating phone numbers

For SMS enrollment, validate phone numbers before submission:

```
import { validatePhoneNumber } from "@coinbase/cdp-core";
try {
  validatePhoneNumber("+14155552671"); // Valid
  validatePhoneNumber("+442071838750"); // Valid (UK)
  validatePhoneNumber("4155552671");    // Throws - missing +
} catch (error) {
  console.error(error.message);
}

```

E.164 format requirements

-   Must start with `+` followed by country code
-   No spaces, hyphens, or parentheses
-   Examples: `+14155552671` (US), `+442071838750` (UK), `+81312345678` (Japan)

## Troubleshooting