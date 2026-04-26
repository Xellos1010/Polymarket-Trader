# ismfaprotectedaction

```
function isMfaProtectedAction(actionName: string): actionName is "signEvmHash" | "signEvmTransaction" | "signSolanaTransaction" | "sendEvmTransaction" | "sendSolanaTransaction" | "signEvmMessage" | "signSolanaMessage" | "signEvmTypedData" | "sendUserOperation" | "exportEvmAccount" | "exportSolanaAccount" | "createEvmKeyExportIframe" | "createSolanaKeyExportIframe";

```

Checks if an action name is MFA-protected. MFA-protected actions require MFA verification when the user is enrolled in MFA. Use this to show visual indicators (like a lock icon) on sensitive action buttons.

## Parameters

Parameter

Type

Description

`actionName`

`string`

The name of the action to check.

## Returns

actionName is “signEvmHash” | “signEvmTransaction” | “signSolanaTransaction” | “sendEvmTransaction” | “sendSolanaTransaction” | “signEvmMessage” | “signSolanaMessage” | “signEvmTypedData” | “sendUserOperation” | “exportEvmAccount” | “exportSolanaAccount” | “createEvmKeyExportIframe” | “createSolanaKeyExportIframe” True if the action requires MFA when the user is enrolled.

## Example

```
import { isMfaProtectedAction } from '@coinbase/cdp-core';
if (isMfaProtectedAction('signEvmTransaction')) {
  // Show lock icon on button
}

```