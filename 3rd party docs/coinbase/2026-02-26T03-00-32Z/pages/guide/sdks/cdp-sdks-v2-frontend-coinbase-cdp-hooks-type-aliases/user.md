# user

```
type User = {
  userId: string;
  authenticationMethods: AuthenticationMethods;
  evmAccounts?: EvmAddress[];
  evmSmartAccounts?: EvmAddress[];
  solanaAccounts?: SolanaAddress[];
  evmAccountObjects?: EndUserEvmAccount[];
  evmSmartAccountObjects?: EndUserEvmSmartAccount[];
  solanaAccountObjects?: EndUserSolanaAccount[];
  mfaMethods?: MFAMethods;
  lastAuthenticatedAt?: string;
};

```

## Properties

Property

Type

`userId`

`string`

`authenticationMethods`

`AuthenticationMethods`

`evmAccounts?`

[`EvmAddress`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Type-Aliases/EvmAddress)\[\]

`evmSmartAccounts?`

[`EvmAddress`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Type-Aliases/EvmAddress)\[\]

`solanaAccounts?`

`SolanaAddress`\[\]

`evmAccountObjects?`

[`EndUserEvmAccount`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/EndUserEvmAccount)\[\]

`evmSmartAccountObjects?`

[`EndUserEvmSmartAccount`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/EndUserEvmSmartAccount)\[\]

`solanaAccountObjects?`

[`EndUserSolanaAccount`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/EndUserSolanaAccount)\[\]

`mfaMethods?`

[`MFAMethods`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/MFAMethods)

`lastAuthenticatedAt?`

`string`