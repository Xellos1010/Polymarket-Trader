# toviemaccount

```
function toViemAccount(address: `0x${string}`): {
};

```

Converts a CDP EVM account into a Viem-compatible LocalAccount. This enables the CDP account to be used with any library or framework that accepts Viem accounts.

## Parameters

Parameter

Type

Description

`address`

`` `0x${string}` ``

The EVM address to create a Viem account for

## Returns

A Viem LocalAccount that can sign messages and transactions using CDP’s signing functions

## Throws

If the user is not authenticated or the address is not in the user’s EVM accounts

## Example

```
const user = await getCurrentUser();
const evmAccount = user.evmAccountObjects[0]?.address;
const viemAccount = await toViemAccount(evmAccount);

```