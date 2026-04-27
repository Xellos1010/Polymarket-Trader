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

The User object.

## Properties

Property

Type

Description

`userId`

`string`

The user ID.

`authenticationMethods`

[`AuthenticationMethods`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/AuthenticationMethods)

The authentication methods used by the user.

`evmAccounts?`

[`EvmAddress`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/EvmAddress)\[\]

**DEPRECATED**: Use `evmAccountObjects` instead for richer account information. The EVM accounts associated with the user. **Deprecated** Use evmAccountObjects instead

`evmSmartAccounts?`

[`EvmAddress`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/EvmAddress)\[\]

**DEPRECATED**: Use `evmSmartAccountObjects` instead for richer account information. The EVM smart accounts associated with the user. **Deprecated** Use evmSmartAccountObjects instead

`solanaAccounts?`

[`SolanaAddress`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SolanaAddress)\[\]

**DEPRECATED**: Use `solanaAccountObjects` instead for richer account information. The Solana accounts associated with the user. **Deprecated** Use solanaAccountObjects instead

`evmAccountObjects?`

[`EndUserEvmAccount`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/EndUserEvmAccount)\[\]

The EVM account objects associated with the user. Users can have up to 10 EVM accounts.

`evmSmartAccountObjects?`

[`EndUserEvmSmartAccount`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/EndUserEvmSmartAccount)\[\]

The EVM smart account objects associated with the user. Each EVM EOA can own one smart account.

`solanaAccountObjects?`

[`EndUserSolanaAccount`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/EndUserSolanaAccount)\[\]

The Solana account objects associated with the user. Users can have up to 10 Solana accounts.

`mfaMethods?`

[`MFAMethods`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/MFAMethods)

The MFA methods enrolled for the user.

`lastAuthenticatedAt?`

`string`

The date and time when the user was last authenticated, in ISO 8601 format.