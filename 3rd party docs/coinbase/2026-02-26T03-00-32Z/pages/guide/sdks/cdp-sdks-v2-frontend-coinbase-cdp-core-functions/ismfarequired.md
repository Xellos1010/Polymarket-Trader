# ismfarequired

```
function isMfaRequired(): boolean;

```

Checks if the next MFA-protected action will require MFA verification. Returns true if the user is enrolled in MFA but hasn’t verified recently (within the configured verification window). When true, calling an MFA-protected action will trigger the registered MFA verification handler. Use this to pre-warn users or show UI hints before they start a sensitive flow.

## Returns

`boolean` True if MFA verification will be required for the next sensitive action.

## Example

```
import { isMfaRequired, signEvmHash } from '@coinbase/cdp-core';
async function handleSign() {
  // prompt mfa before calling the sign hash function
  if (isMfaRequired()) {
    await ensureMfaVerified(); // User will be prompted
  }
  await signEvmHash({ ... }); // MFA handled automatically by withMfa
}

```