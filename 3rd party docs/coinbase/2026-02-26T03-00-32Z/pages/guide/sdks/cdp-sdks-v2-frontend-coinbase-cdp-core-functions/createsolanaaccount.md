# createsolanaaccount

```
function createSolanaAccount(): Promise<string>;

```

Creates a Solana account for the current user. Multiple Solana accounts can be created per user.

## 

[​](#returns)

Returns

`Promise`<`string`\> The address of the newly created Solana account.

## 

[​](#throws)

Throws

Error if the user is not signed in.