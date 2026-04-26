# createevmeoaaccount

```
function createEvmEoaAccount(): Promise<`0x${string}`>;

```

Creates an EVM EOA (Externally Owned Account) for the current user. Multiple EVM EOA accounts can be created per user.

## 

[​](#returns)

Returns

`Promise`<`` `0x${string}` ``\> The address of the newly created EVM EOA account.

## 

[​](#throws)

Throws

Error if the user is not signed in.