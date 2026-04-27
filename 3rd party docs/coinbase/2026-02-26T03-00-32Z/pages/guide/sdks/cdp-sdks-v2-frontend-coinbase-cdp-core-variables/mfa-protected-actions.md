# mfa protected actions

```
const MFA_PROTECTED_ACTIONS: (
  | "signEvmHash"
  | "signEvmTransaction"
  | "signSolanaTransaction"
  | "sendEvmTransaction"
  | "sendSolanaTransaction"
  | "signEvmMessage"
  | "signSolanaMessage"
  | "signEvmTypedData"
  | "sendUserOperation"
  | "exportEvmAccount"
  | "exportSolanaAccount"
  | "createEvmKeyExportIframe"
  | "createSolanaKeyExportIframe")[];

```

List of action names that require MFA verification when the user is enrolled. These actions involve sensitive operations like signing transactions, sending funds, or exporting private keys.

## Example

```
import { MFA_PROTECTED_ACTIONS } from '@coinbase/cdp-core';
console.log(MFA_PROTECTED_ACTIONS);
// ['signEvmHash', 'signEvmTransaction', 'signSolanaTransaction', ...]

```